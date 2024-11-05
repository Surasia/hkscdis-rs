use super::{
    hs_enums::HSEnum,
    hs_function::HSFunction,
    hs_header::{HSFeatures, HSHeader},
    hs_reader::read_string,
    hs_structure::HSStructPrototype,
};
use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, HeaderReadable},
};

use byteorder::{ByteOrder, ReadBytesExt, BE, LE};
use colored::Colorize;
use std::{fmt::Display, fs::File, io::BufReader};

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
    pub structs: Vec<HSStructPrototype>,
}

impl HavokScriptFile {
    pub fn read(
        &mut self,
        reader: &mut BufReader<File>,
        enable_inheritance: bool,
    ) -> Result<(), HkscError> {
        self.header.read(reader)?;
        if self.header.is_little_endian {
            self.enums = reader.read_enumerable::<HSEnum, LE>(self.header.enum_count.into())?;
            self.main_function.read::<LE>(reader, &self.header)?;
            self.read_structures::<LE>(reader, enable_inheritance)?;
        } else {
            self.enums = reader.read_enumerable::<HSEnum, BE>(self.header.enum_count.into())?;
            self.main_function.read::<BE>(reader, &self.header)?;
            self.read_structures::<BE>(reader, enable_inheritance)?;
        }
        Ok(())
    }

    pub fn read_structures<T: ByteOrder>(
        &mut self,
        reader: &mut BufReader<File>,
        enable_inheritance: bool,
    ) -> Result<(), HkscError> {
        if self.header.features.contains(HSFeatures::STRUCTURES) {
            let check = reader.read_u32::<T>()?;
            if check != 1 {
                return Ok(());
            }

            loop {
                let name = read_string::<T>(reader, &self.header)?;
                if name.is_empty() {
                    break;
                }
                let mut structure = HSStructPrototype::default();
                structure.read::<T>(reader, &self.header, enable_inheritance)?;
                structure.name = name;
                self.structs.push(structure);
            }
        }
        Ok(())
    }
}

impl Display for HavokScriptFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} \n{}", "[Header]".green(), self.header)?;

        writeln!(f, "{}", "[Enums]".green())?;
        for item in &self.enums {
            writeln!(f, "{item}")?;
        }
        writeln!(f)?;
        writeln!(f, "{}", self.main_function)?;

        if !self.structs.is_empty() {
            for struc in &self.structs {
                writeln!(f, "{struc}")?;
                for member in &struc.slots {
                    writeln!(f, "{} {}", "-".yellow(), member)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
