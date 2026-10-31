use std::env::args;

use crate::{args::AppArgs, error::AppError};

mod args;
mod error;
mod tests;

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
