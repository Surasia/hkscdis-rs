use super::{hs_header::HSHeader, hs_reader::read_string};
use byteorder::{ReadBytesExt, BE, LE};
use std::{fs::File, io::BufReader};

#[derive(Debug, Default, Clone)]
pub struct HSFunctionDebugInfoLocals {
    pub local_name: String,
    pub start: u32,
    pub end: u32,
}

impl HSFunctionDebugInfoLocals {
    pub fn read(&mut self, reader: &mut BufReader<File>, header: &HSHeader) -> std::io::Result<()> {
        self.local_name = read_string(reader, header)?;
        self.start = reader.read_u32::<LE>()?;
        self.end = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct HSFunctionDebugInfo {
    pub line_count: u32,
    pub locals_count: u32,
    pub up_value_count: u32,
    pub line_begin: u32,
    pub line_end: u32,
    pub path: String,
    pub function_name: String,
    pub lines: Vec<u32>,
    pub locals: Vec<HSFunctionDebugInfoLocals>,
    pub up_values: Vec<String>,
}

impl HSFunctionDebugInfo {
    pub fn read(&mut self, reader: &mut BufReader<File>, header: &HSHeader) -> std::io::Result<()> {
        self.line_count = reader.read_u32::<BE>()?;
        self.locals_count = reader.read_u32::<BE>()?;
        self.up_value_count = reader.read_u32::<BE>()?;
        self.line_begin = reader.read_u32::<BE>()?;
        self.line_end = reader.read_u32::<BE>()?;
        self.path = read_string(reader, header)?;
        self.function_name = read_string(reader, header)?;

        self.lines = (0..self.line_count)
            .map(|_| reader.read_u32::<BE>())
            .collect::<Result<_, _>>()?;

        self.locals = vec![HSFunctionDebugInfoLocals::default(); self.locals_count as usize];
        for local in &mut self.locals {
            local.read(reader, header)?;
        }

        self.up_values = (0..self.up_value_count)
            .map(|_| read_string(reader, header))
            .collect::<Result<_, _>>()?;

        Ok(())
    }
}
