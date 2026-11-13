use std::env::args;

use ini_parse::{AppArgs, AppError, Config};

fn main() {
    match run() {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("ERROR: {}", error),
    };
}

fn run<'a>() -> Result<String, AppError> {
    let args: Vec<String> = args().into_iter().skip(1).collect();
    let args = AppArgs::new(&args)?;
    let content = Config::load(args.path)?;
    let config =
        Config::parse(&content).map_err(|e| AppError::Parse(args.path.to_path_buf(), e))?;

    let mut result = "".to_string();

    for (index, lookup) in args.lookups.iter().enumerate() {
        if let Some(value) = config.get(lookup.section, lookup.key) {
            result.push_str(value);
            if index < args.lookups.len() - 1 {
                result.push(if args.new_line { '\n' } else { args.separator });
            }
        }
    }

    Ok(result)
}
