# hkscdis-rs

Rust re-write of my [hksc-disassembler](https://github.com/Surasia/hksc-disassembler) project I wrote to practice rust. Parses and reads HavokScript 5.1 bytecode, outputting it in a human-readable format.

## Usage
```
A CLI tool to disassemble Havok Script 5.1 files

Usage: hkscdis-rs.exe --path <FILE>

Options:
  -p, --path <FILE>
  -h, --help         Print help
```

## Dependencies
- bitflags
- byteorder
- clap (with derive)
- color-print
- num_enum

## Credits
- Soupstream for the amazing [havok-script-tools](https://github.com/soupstream/havok-script-tools), most of which this project is based off of.
- Jake-NotTheMuss for their very insightful [hksc](https://github.com/Jake-NotTheMuss/hksc).
- Katalash for their  [DSLuaDecompiler](https://github.com/katalash/DSLuaDecompiler), a very advanced decompiler for HavokScript.
