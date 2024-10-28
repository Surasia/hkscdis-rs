use super::{
    hs_header::{HSCompatability, HSHeader},
    hs_opcodes::HSType,
    hs_reader::read_string,
};
use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, HeaderReadable},
};

use byteorder::{ReadBytesExt, BE};
use std::io::BufRead;

#[derive(Default, Debug)]
/// Information about a structure member's type and metadata
pub struct HSStructMemberInfo {
    /// Name of the struct member
    pub name: String,
    /// Unknown flags or enum value
    unk0: u32,
    /// Unique identifier for this struct within the file
    struct_id: i32,
    /// Type of the struct member (TSTRUCT)
    pub type_: HSType,
    /// Unknown value 1
    unk1: u32,
    /// Unknown value 2
    unk2: u32,
}

impl HeaderReadable for HSStructMemberInfo {
    fn read<R>(&mut self, reader: &mut R, header: &HSHeader) -> Result<(), HkscError>
    where
        R: BufRead + BufReaderExt,
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
/// A member of a structure, containing type information and an index
pub struct HSStructMember {
    /// Type and metadata information for this member
    pub header: HSStructMemberInfo,
    /// Index of this member within its parent structure
    index: i32,
}

impl HeaderReadable for HSStructMember {
    fn read<R>(&mut self, reader: &mut R, header: &HSHeader) -> Result<(), HkscError>
    where
        R: BufRead + BufReaderExt,
    {
        self.header.read(reader, header)?;
        self.index = reader.read_i32::<BE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// A block containing structure information including members and inheritance
pub struct HSStructBlock {
    /// Type and metadata information for this structure block
    pub header: HSStructMemberInfo,
    /// Number of members in this structure
    member_count: u32,
    /// Number of structures this structure extends/inherits from
    extend_count: u32,
    /// Names of structures that this structure extends/inherits from
    pub extended_structs: Vec<String>,
    /// List of structure members
    pub members: Vec<HSStructMember>,
}

impl HeaderReadable for HSStructBlock {
    fn read<R>(&mut self, reader: &mut R, header: &HSHeader) -> Result<(), HkscError>
    where
        R: BufRead + BufReaderExt,
    {
        self.header.read(reader, header)?;
        self.member_count = reader.read_u32::<BE>()?;
        if header.compatability.contains(HSCompatability::STRUCTURES) {
            self.extend_count = reader.read_u32::<BE>()?;
            self.extended_structs = (0..self.extend_count)
                .map(|_| read_string(reader, header))
                .collect::<Result<Vec<_>, HkscError>>()?;
        }

        self.members =
            reader.read_header_enumerable::<HSStructMember>(self.member_count.into(), header)?;

        Ok(())
    }
}
