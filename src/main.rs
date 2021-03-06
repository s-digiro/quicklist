#[macro_use]
extern crate simple_error;
extern crate subprocess;
extern crate left_pad;
extern crate termion;

use std::error::Error;
use std::env;

mod parse;
mod op;
mod list;
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    // get args
    let args: Vec<String> = env::args().collect();

    let op = parse::parse(args);

    op.exec()?;

    Ok(())
}

