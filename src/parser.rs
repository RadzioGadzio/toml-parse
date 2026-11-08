use crate::{AppError, Section};

pub fn parse(src: &str) -> Result<Vec<Section<'_>>, AppError> {
    let mut sections: Vec<Section<'_>> = Vec::new();
    let mut current = Section::new(None, Vec::new());

    for (idx, raw) in src.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw.trim();

        // Skip empty or comments
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        // Parse section lines
        if let Some(rest) = line.strip_prefix('[') {
            let name = rest
                .strip_suffix(']')
                .ok_or(AppError::Parse {
                    line: line_no,
                    message: "unclosed section header",
                })?
                .trim();

            if name.is_empty() {
                return Err(AppError::Parse {
                    line: line_no,
                    message: "empty section name",
                });
            }

            // Push old section
            sections.push(current);

            // Create new section
            current = Section::new(Some(name), Vec::new());

            continue;
        }

        // Retrieve key=value pair
        let (key, value) = line.split_once('=').ok_or(AppError::Parse {
            line: line_no,
            message: "expected 'key = value'",
        })?;
        let key = key.trim();
        let mut value = value.trim();
        // Removes any inline comments
        value = value.split_once(';').map_or(value, |v| v.0.trim());
        value = value.split_once('#').map_or(value, |v| v.0.trim());

        if key.is_empty() {
            return Err(AppError::Parse {
                line: line_no,
                message: "empty key",
            });
        }

        if current.entries.iter().any(|(k, _)| *k == key) {
            return Err(AppError::Parse {
                line: line_no,
                message: "duplicate key",
            });
        }

        if value.contains('=') {
            return Err(AppError::Parse {
                line: line_no,
                message: "value cannot contain '='",
            });
        }

        current.entries.push((key, value));
    }

    // Push last section
    sections.push(current);

    // Remove sections which don't have a name or do not have any entries empty
    sections.retain(|s| s.name.is_some() || !s.entries.is_empty());

    Ok(sections)
}
