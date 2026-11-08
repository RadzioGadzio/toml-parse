use std::{fs, path::Path};

use crate::{
    error::AppError,
    parser::{ParseError, parse},
};

#[derive(Debug)]
pub struct Config<'a> {
    sections: Vec<Section<'a>>,
}

#[derive(Debug)]
pub struct Section<'a> {
    pub name: Option<&'a str>,
    pub entries: Vec<(&'a str, &'a str)>,
}

impl<'a> Config<'a> {
    pub fn load(path: &Path) -> Result<String, AppError> {
        let content = fs::read_to_string(path).map_err(|e| AppError::FileRead {
            path: path.to_path_buf(),
            cause: e,
        })?;

        Ok(content)
    }

    pub fn parse(src: &'a str) -> Result<Config<'a>, ParseError> {
        Ok(Config {
            sections: parse(src)?,
        })
    }

    pub fn get(&self, section: Option<&str>, key: &str) -> Option<&'a str> {
        *&self
            .sections
            .iter()
            .filter(|s| match (s.name, section) {
                (Some(section), Some(selected)) => section == selected,
                (None, None) => true,
                _ => false,
            })
            .find_map(|s| s.entries.iter().find(|(k, v)| *k == key).map(|(_, v)| *v))
    }

    pub fn get_parsed<T: std::str::FromStr>(
        &self,
        section: Option<&str>,
        key: &str,
    ) -> Option<Result<T, T::Err>> {
        todo!()
    }
}

impl<'a> Section<'a> {
    pub fn new(name: Option<&'a str>, entries: Vec<(&'a str, &'a str)>) -> Section<'a> {
        Section { name, entries }
    }
}
