use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use std::{fs::File, io::BufReader};

bitflags! {
    #[derive(Debug, Clone, Default)]
    pub struct HSCompatability: u8 {
        const MEMOIZATION = 1 << 0;
        const STRUCTURES = 1 << 1;
        const SELF = 1 << 2;
        const DOUBLES = 1 << 3;
        const NATIVEINT = 1 << 4;
    }
}

#[derive(Debug, Clone, Default)]
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
}

/* Important Note */
// The "is little endian" flag is disabled in every bytecode file I have come across.
// Currently it's not worth implementing (and its not as easy as python)

impl HSHeader {
    pub fn read(&mut self, reader: &mut BufReader<File>) -> Result<(), std::io::Error> {
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
        Ok(())
    }
}
