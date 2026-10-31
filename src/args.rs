use std::{iter::Peekable, path::Path, vec::IntoIter};

use crate::error::AppError;

#[derive(Debug)]
pub struct AppArgs<'a> {
    pub path: &'a Path,
    pub lookups: Peekable<IntoIter<Lookup<'a>>>,
}

#[derive(Debug)]
pub struct Lookup<'a> {
    pub section: Option<&'a str>,
    pub key: &'a str,
}

impl<'a> AppArgs<'a> {
    pub fn new(args: &'a [String]) -> Result<AppArgs<'a>, AppError> {
        let mut args = args.into_iter().peekable();

        let path = args
            .next()
            .ok_or(AppError::InvalidArgs("Missing file path"))?;

        let lookups = args
            .map(String::as_str)
            .map(Lookup::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(AppArgs {
            path: Path::new(path),
            lookups: lookups.into_iter().peekable(),
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
            return Err(AppError::InvalidArgs("Empty section or key"));
        }
        Ok(Lookup { section, key })
    }
}
