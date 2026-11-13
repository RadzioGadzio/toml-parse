use std::path::Path;

use crate::error::AppError;

#[derive(Debug)]
pub struct AppArgs<'a> {
    pub path: &'a Path,
    pub new_line: bool,
    pub separator: char,
    pub lookups: Vec<Lookup<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Lookup<'a> {
    pub section: Option<&'a str>,
    pub key: &'a str,
}

impl<'a> AppArgs<'a> {
    pub fn new(args: &'a [String]) -> Result<AppArgs<'a>, AppError> {
        let mut args = args.into_iter().peekable();

        let mut new_line = false;
        let mut separator = ' ';

        while let Some(arg) = args.next_if(|arg| arg.starts_with('-')) {
            let arg = arg.trim_start_matches('-');
            if arg == "new-line" {
                new_line = true
            }
            if arg == "sep"
                && let Some(next_arg) = args.peek()
            {
                let ch = next_arg.chars().nth(0);
                if let Some(ch) = ch {
                    separator = ch;
                }

                args.next();
            }
        }

        let path = args
            .next()
            .ok_or(AppError::InvalidArgs("missing file path"))?;

        let lookups = args
            .map(String::as_str)
            .map(Lookup::parse)
            .collect::<Result<Vec<_>, _>>()?;

        if lookups.is_empty() {
            return Err(AppError::InvalidArgs("missing lookup"));
        }

        Ok(AppArgs {
            path: Path::new(path),
            new_line,
            separator,
            lookups,
        })
    }
}

impl<'a> Lookup<'a> {
    fn parse(arg: &'a str) -> Result<Self, AppError> {
        let (section, key) = match arg.split_once('.') {
            Some((section, key)) => (Some(section), key),
            None => (None, arg),
        };
        if key.is_empty() || section == Some("") {
            return Err(AppError::InvalidArgs("empty section or key"));
        }
        Ok(Lookup { section, key })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn missing_path() {
        let args = args(&[]);
        let result = AppArgs::new(&args);

        assert!(matches!(result, Err(AppError::InvalidArgs(_))));
    }

    #[test]
    fn missing_lookup() {
        let args = args(&["file"]);
        let result = AppArgs::new(&args);
        assert!(matches!(result, Err(AppError::InvalidArgs(_))));
    }

    #[test]
    fn path_and_lookups() {
        let args = args(&["file", "server.host", "port"]);
        let parsed = AppArgs::new(&args).unwrap();

        assert_eq!(parsed.path, Path::new("file"));
        assert_eq!(
            parsed.lookups,
            vec![
                Lookup {
                    section: Some("server"),
                    key: "host"
                },
                Lookup {
                    section: None,
                    key: "port"
                }
            ]
        );
    }

    #[test]
    fn loopup_with_section() {
        assert_eq!(
            Lookup::parse("server.host").unwrap(),
            Lookup {
                section: Some("server"),
                key: "host"
            }
        )
    }
    #[test]
    fn loopup_without_section() {
        assert_eq!(
            Lookup::parse("port").unwrap(),
            Lookup {
                section: None,
                key: "port"
            }
        )
    }

    #[test]
    fn lookup_extra_dots_stay_in_key() {
        assert_eq!(
            Lookup::parse("a.b.c").unwrap(),
            Lookup {
                section: Some("a"),
                key: "b.c"
            }
        )
    }
    #[test]
    fn invalid_lookups() {
        for input in [".host", "server.", ".", ""] {
            assert!(Lookup::parse(input).is_err(), "should reject {input:?}")
        }
    }
}
