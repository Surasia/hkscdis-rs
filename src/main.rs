#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod cli;
pub mod common;
pub mod loader;

use crate::common::errors::HkscError;
use clap::Parser;
use cli::disassembler::print_disassembly;
use loader::hs::HavokScriptFile;
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Parser)]
#[command(name = "Havok Script Disassembler")]
/// A CLI tool to disassemble Havok Script 5.1 files
struct Disassembler {
    #[arg(short, long, value_name = "FILE")]
    path: PathBuf,
}

fn main() -> Result<(), HkscError> {
    let cli = Disassembler::parse();

    let file = File::open(cli.path)?;
    let mut reader = BufReader::new(file);
    let mut havok_script_file = HavokScriptFile::default();

    havok_script_file.read(&mut reader)?;
    print_disassembly(&havok_script_file);

    Ok(())
}
