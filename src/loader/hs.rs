use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
};

use super::{
    hs_enums::HSEnum, hs_function::HSFunction, hs_header::HSHeader, hs_structure::HSStructBlock,
};

#[derive(Default)]
pub struct HavokScriptFile {
    pub header: HSHeader,
    pub enums: Vec<HSEnum>,
    pub main_function: HSFunction,
    pub structs: Vec<HSStructBlock>,
}

impl HavokScriptFile {
    pub fn read(&mut self, reader: &mut BufReader<File>) -> std::io::Result<()> {
        self.header = HSHeader::default();
        self.header.read(reader)?;

        self.enums = HSEnum::read(reader)?;

        self.main_function = HSFunction::default();
        self.main_function.read(reader, &self.header)?;

        reader.seek(SeekFrom::Current(4))?;

        self.structs = Vec::new();
        while reader.read_u64::<LE>()? != 0 {
            reader.seek(SeekFrom::Current(-8))?;
            let mut structure = HSStructBlock::default();
            structure.read(reader, &self.header)?;
            self.structs.push(structure);
        }

        Ok(())
    }
}
