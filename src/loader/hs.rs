use super::{
    hs_enums::HSEnum,
    hs_function::HSFunction,
    hs_header::{HSFeatures, HSHeader},
    hs_structure::HSStructBlock,
};
use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, HeaderReadable},
};

use byteorder::{ByteOrder, ReadBytesExt, BE, LE};
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
        if self.header.is_little_endian {
            self.enums = reader.read_enumerable::<HSEnum, LE>(self.header.enum_count.into())?;
            self.main_function.read::<LE>(reader, &self.header)?;
            reader.seek_relative(4)?;
            self.read_structures::<LE>(reader)?;
        } else {
            self.enums = reader.read_enumerable::<HSEnum, BE>(self.header.enum_count.into())?;
            self.main_function.read::<BE>(reader, &self.header)?;
            reader.seek_relative(4)?;
            self.read_structures::<BE>(reader)?;
        }
        Ok(())
    }

    pub fn read_structures<T: ByteOrder>(
        &mut self,
        reader: &mut BufReader<File>,
    ) -> Result<(), HkscError> {
        while self.header.features.contains(HSFeatures::STRUCTURES) && reader.read_u64::<T>()? != 0
        {
            reader.seek_relative(-8)?; // Compensate for the previous read
            let mut structure = HSStructBlock::default();
            structure.read::<T>(reader, &self.header)?;
            self.structs.push(structure);
        }
        Ok(())
    }
}
