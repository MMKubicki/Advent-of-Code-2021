use std::fs;
use std::io;
use std::io::BufRead;
use std::path;

use clap::Parser;
use miette::{IntoDiagnostic, Result};

use crate::util::{Command, Position, PositionPart1, PositionPart2};

mod util;

#[derive(Parser)]
struct Opts {
    #[clap(parse(from_os_str))]
    input: path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let file = fs::File::open(opts.input).into_diagnostic()?;
    let reader = io::BufReader::new(file);

    let values = reader
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse())
        .filter_map(Result::ok)
        .collect::<Vec<Command>>();

    let pos1 = values
        .iter()
        .copied()
        .fold(PositionPart1::default(), PositionPart1::applyf);

    println!("Task 1: {:?}", pos1);
    println!("Value: {}", pos1.multiplied_value());

    let pos2 = values
        .iter()
        .copied()
        .fold(PositionPart2::default(), PositionPart2::applyf);

    println!("Task 2: {:?}", pos2);
    println!("Value: {}", pos2.multiplied_value());

    Ok(())
}
