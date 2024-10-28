use super::{
    hs_enums::HSEnum, hs_function::HSFunction, hs_header::HSHeader, hs_structure::HSStructBlock,
};
use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, HeaderReadable},
};

use byteorder::{ReadBytesExt, LE};
use std::{fs::File, io::BufReader};

#[derive(Default)]
/// Main container for the Havok Script file.
pub struct HavokScriptFile {
    /// Header contains information on how to continue reading the file.
    /// This includes WORD size, endianness etc.
    pub header: HSHeader,
    /// Representation of possible types that can occur in the file.
    /// Seems to remain the same.
    pub enums: Vec<HSEnum>,
    /// The "main" function that all other functions branch off of.
    /// This can be an initializer, actual main function, or just the first function.
    pub main_function: HSFunction,
    /// Havok structure definitions that allow interop with game engine.
    pub structs: Vec<HSStructBlock>,
}

impl HavokScriptFile {
    pub fn read(&mut self, reader: &mut BufReader<File>) -> Result<(), HkscError> {
        self.header.read(reader)?;
        self.enums = reader.read_enumerable::<HSEnum>(self.header.enum_count.into())?;
        self.main_function.read(reader, &self.header)?;

        reader.seek_relative(4)?; // Padding?

        // End of the file is indicated by a u64 with a value of 0, so we loop until we find it,
        // reading structs as we go
        while reader.read_u64::<LE>()? != 0 {
            reader.seek_relative(-8)?; // Compensate for the previous read
            let mut structure = HSStructBlock::default();
            structure.read(reader, &self.header)?;
            self.structs.push(structure);
        }

        Ok(())
    }
}
