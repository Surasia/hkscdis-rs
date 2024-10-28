use crate::errors::HkscError;

use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE, BE};
use std::{fs::File, io::BufReader};

bitflags! {
    #[derive(Debug, Default)]
    /// Flags for enabling `HavokScript` features, such as global memoization.
    pub struct HSCompatability: u8 {
        /// Enable memoization.
        const MEMOIZATION = 1 << 0;
        /// Enable extended structures.
        const STRUCTURES = 1 << 1;
        /// Enable self references.
        const SELF = 1 << 2;
        /// Enable double precision numbers.
        const DOUBLES = 1 << 3;
        /// Enable native integers. (Does not respect t_size)
        const NATIVEINT = 1 << 4;
    }
}

#[derive(Debug, Default)]
/// Header of a `HavokScript` file.
pub struct HSHeader {
    pub magic: u32,
    pub version: u8,
    pub fmt: u8,
    pub is_little_endian: bool,
    pub int_size: u8,
    pub t_size: u8,
    pub instruction_size: u8,
    pub number_size: u8,
    pub is_integer: bool,
    pub compatability: HSCompatability,
    pub shared: u8,
    pub enum_count: u32,
}

// **Important Note:**
// The `is_little_endian` flag is disabled in every bytecode file I have come across.
// I currently do not plan to implement it, so all files are assumed to be big endian.

impl HSHeader {
    pub fn read(&mut self, reader: &mut BufReader<File>) -> Result<(), HkscError> {
        self.magic = reader.read_u32::<LE>()?;
        self.version = reader.read_u8()?;
        self.fmt = reader.read_u8()?;
        self.is_little_endian = reader.read_u8()? != 0;
        self.int_size = reader.read_u8()?;
        self.t_size = reader.read_u8()?;
        self.instruction_size = reader.read_u8()?;
        self.number_size = reader.read_u8()?;
        self.is_integer = reader.read_u8()? != 0;
        self.compatability = HSCompatability::from_bits_truncate(reader.read_u8()?);
        self.shared = reader.read_u8()?;
        self.enum_count = reader.read_u32::<BE>()?;
        Ok(())
    }
}
