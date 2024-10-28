use crate::{common::extensions::{BufReaderExt, HeaderReadable}, errors::HkscError};
use super::{
    hs_header::{HSCompatability, HSHeader},
    hs_opcodes::HSType,
    hs_reader::read_string,
};

use byteorder::{ReadBytesExt, BE};
use std::io::BufRead;

#[derive(Default, Debug)]
/// Header of the structure definition containing info on reading its entries.
pub struct HSStructHeader {
    /// Name of the struct.
    pub name: String,
    /// Unknown, might be an enum?
    unk0: u32,
    /// Index of the structure in the entire file.
    struct_id: i32,
    /// Type of structure (TSTRUCT).
    pub type_: HSType,
    /// Unknown.
    unk1: u32,
    /// Unknown.
    unk2: u32,
}

impl HeaderReadable for HSStructHeader {
    fn read<R>(&mut self, reader: &mut R, header: &HSHeader) -> Result<(), HkscError>
    where
        R: BufRead + BufReaderExt
    {
        self.name = read_string(reader, header)?;
        self.unk0 = reader.read_u32::<BE>()?;
        self.struct_id = reader.read_i32::<BE>()?;
        let type_byte = u8::try_from(reader.read_u32::<BE>()?)?;
        self.type_ = HSType::try_from(type_byte).map_err(|_| HkscError::UnknownType(type_byte))?;
        self.unk1 = reader.read_u32::<BE>()?;
        self.unk2 = reader.read_u32::<BE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct HSStructMember {
    pub header: HSStructHeader,
    index: i32,
}

impl HeaderReadable for HSStructMember {
    fn read<R>(&mut self, reader: &mut R, header: &HSHeader) -> Result<(), HkscError>
    where
        R: BufRead + BufReaderExt
    {
        self.header.read(reader, header)?;
        self.index = reader.read_i32::<BE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct HSStructBlock {
    pub header: HSStructHeader,
    member_count: u32,
    extend_count: u32,
    pub extended_structs: Vec<String>,
    pub members: Vec<HSStructMember>,
}

impl HeaderReadable for HSStructBlock {
    fn read<R>(&mut self, reader: &mut R, header: &HSHeader) -> Result<(), HkscError>
    where
        R: BufRead + BufReaderExt
    {
        self.header.read(reader, header)?;
        self.member_count = reader.read_u32::<BE>()?;
        if header.compatability.contains(HSCompatability::STRUCTURES) {
            self.extend_count = reader.read_u32::<BE>()?;
            self.extended_structs = (0..self.extend_count)
                .map(|_| read_string(reader, header))
                .collect::<Result<Vec<_>, HkscError>>()?;
        }

        self.members = reader.read_header_enumerable::<HSStructMember>(self.member_count.into(), header)?;

        Ok(())
    }
}
