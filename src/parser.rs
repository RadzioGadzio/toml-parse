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
        // Removes any inline comments
        value = value.split_once(';').map_or(value, |v| v.0.trim());
        value = value.split_once('#').map_or(value, |v| v.0.trim());

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
