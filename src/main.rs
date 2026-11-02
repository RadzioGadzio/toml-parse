use std::env::args;

use ini_parse::{AppArgs, AppError};

fn main() {
    match run() {
        Err(error) => eprintln!("ERROR: {}", error),
        _ => (),
    };
}

fn run<'a>() -> Result<(), AppError> {
    let args: Vec<String> = args().into_iter().skip(1).collect();
    let args = AppArgs::new(&args)?;

    println!("{:?}", args);
    Ok(())
}
