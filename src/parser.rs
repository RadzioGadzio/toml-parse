use crate::{Section};

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub line: usize,
    pub message: &'static str,
}

pub fn parse(src: &str) -> Result<Vec<Section<'_>>, ParseError> {
    let mut sections: Vec<Section<'_>> = vec![Section::new(None, Vec::new())];
    let mut current: usize = 0;

    for (line_no, raw) in src.lines().enumerate() {
        let line = raw.trim();

        // Skip empty or comments
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        // Parse section lines
        if let Some(rest) = line.strip_prefix('[') {
            let name = rest
                .strip_suffix(']')
                .ok_or(ParseError {
                    line: line_no,
                    message: "unclosed section header",
                })?
                .trim();

            if name.is_empty() {
                return Err(ParseError {
                    line: line_no,
                    message: "empty section name",
                });
            }

            // Find position of section in `section` or create new one
            current = match sections.iter().position(|s| s.name == Some(name)) {
                Some(i) => i,
                None => {
                    sections.push(Section::new(Some(name), Vec::new()));
                    sections.len() - 1
                }
            };

            continue;
        }

        // Retrieve key=value pair
        let (key, value) = line.split_once('=').ok_or(ParseError {
            line: line_no,
            message: "expected 'key = value'",
        })?;
        let key = key.trim();
        let mut value = value.trim();

        if key.is_empty() {
            return Err(ParseError {
                line: line_no,
                message: "empty key",
            });
        }
        // Removes any inline comments
        value = value.split_once(" ;").map_or(value, |v| v.0.trim());
        value = value.split_once(" #").map_or(value, |v| v.0.trim());

        if value.contains('=') {
            return Err(ParseError {
                line: line_no,
                message: "value cannot contain '='",
            });
        }

        let section = &mut sections[current];

        if section.entries.iter().any(|(k, _)| *k == key) {
            return Err(ParseError {
                line: line_no,
                message: "duplicate key",
            });
        }

        section.entries.push((key, value));
    }

    // Remove sections which don't have a name or do not have any entries empty
    sections.retain(|s| s.name.is_some() || !s.entries.is_empty());

    Ok(sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sections_and_keys() {
        let got = parse(
            "
        [a]
        x = 1
        y = 2
        [b]
        z = 3
        ",
        )
        .unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].name, Some("a"));
        assert_eq!(got[0].entries, vec![("x", "1"), ("y", "2")]);
        assert_eq!(got[1].name, Some("b"));
        assert_eq!(got[1].entries, vec![("z", "3"),]);
    }

    #[test]
    fn global_keys() {
        let got = parse(
            "
        x = 1
        [a]
        y = 2
        ",
        )
        .unwrap();

        assert_eq!(got.len(), 2);
        assert_eq!(got[0].name, None);
        assert_eq!(got[0].entries, vec![("x", "1"),]);
        assert_eq!(got[1].name, Some("a"));
        assert_eq!(got[1].entries, vec![("y", "2"),])
    }

    #[test]
    fn strips_comments() {
        let got = parse(
            "
        # Comment 1
        ; Comment 2
        x = 0
        y = 1 # Some comment
        z = 2 ; Some other comment
        ",
        )
        .unwrap();

        assert_eq!(got.len(), 1);
        assert_eq!(got[0].entries, vec![("x", "0"), ("y", "1"), ("z", "2"),])
    }

    #[test]
    fn repeated_section_is_merged() {
        let got = parse(
            "
      [a]
      x=1
      [b]
      y=2
      [a]
      z=3
      ",
        )
        .unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].name, Some("a"));
        assert_eq!(got[0].entries, vec![("x", "1"), ("z", "3"),])
    }

    #[test]
    fn same_key_in_different_sections() {
        assert!(
            parse(
                "
        [a]
        x = 1
        [b]
        x = 2
        "
            )
            .is_ok()
        )
    }

    #[test]
    fn key_errors() {
        assert_eq!(
            parse(
                "
          x = 1
          error
          ",
            )
            .unwrap_err(),
            ParseError {
                line: 2,
                message: "expected 'key = value'"
            }
        );

        assert_eq!(
            parse(
                "
            = 2
            "
            )
            .unwrap_err(),
            ParseError {
                line: 1,
                message: "empty key"
            }
        );
    }

    #[test]
    fn value_errors() {
        assert_eq!(
            parse(
                "
          x = a=b
          "
            )
            .unwrap_err(),
            ParseError {
                line: 1,
                message: "value cannot contain '='"
            }
        );
    }

    #[test]
    fn header_errors() {
        assert_eq!(
            parse(
                "
        [a
        x = 0
        "
            )
            .unwrap_err(),
            ParseError {
                line: 1,
                message: "unclosed section header"
            }
        );
        assert_eq!(
            parse(
                "
        []
        x = 0
        "
            )
            .unwrap_err(),
            ParseError {
                line: 1,
                message: "empty section name"
            }
        );
    }

    #[test]
    fn duplicate_key_errors() {
        assert_eq!(
            parse(
                "
          [a]
          x = 1
          x = 2
          "
            )
            .unwrap_err(),
            ParseError {
                line: 3,
                message: "duplicate key"
            }
        );

        assert_eq!(
            parse(
                "
        [a]
        x = 1
        [b]
        y = 2
        [a]
        x = 3
        "
            )
            .unwrap_err(),
            ParseError {
                line: 6,
                message: "duplicate key"
            }
        );
    }
}
