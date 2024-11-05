#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod common;
pub mod loader;

use crate::common::errors::HkscError;
use clap::Parser;
use loader::hs::HavokScriptFile;
use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "Havok Script Disassembler")]
/// A CLI tool to disassemble Havok Script 5.1 files
struct Disassembler {
    #[arg(short, long, value_name = "FILE")]
    /// File to disassemble.
    path: PathBuf,
    #[arg(short = 'i', long)]
    /// Enable extensions for structure inheritance.
    enable_inheritance: bool,
    #[arg(short = 'c', long, default_value = "false")]
    /// Disable displaying colors with the disassembly.
    disable_colors: bool,
    #[arg(short = 'o', long, value_name = "FILE")]
    /// Optional output file. If not specified, output goes to stdout.
    output: Option<PathBuf>,
}

fn main() -> Result<(), HkscError> {
    let cli = Disassembler::parse();
    let file = File::open(cli.path)?;
    let mut reader = BufReader::new(file);
    let mut havok_script_file = HavokScriptFile::default();

    if cli.disable_colors {
        colored::control::set_override(false);
    }

    havok_script_file.read(&mut reader, cli.enable_inheritance)?;

    match cli.output {
        Some(path) => {
            colored::control::set_override(false); // ANSI escape codes don't work in files
            let mut output_file = File::create(path)?;
            write!(output_file, "{havok_script_file}")?;
        }
        None => println!("{havok_script_file}"),
    }
    Ok(())
}
