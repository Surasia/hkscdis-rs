use super::{
    hs_header::{HSCompatability, HSHeader},
    hs_opcodes::HSType,
    hs_reader::read_string,
};
use byteorder::{ReadBytesExt, BE};
use std::{
    fs::File,
    io::{BufReader, Result},
};

#[derive(Default, Debug)]
pub struct HSStructHeader {
    pub name: String,
    pub unk0: u32,
    pub struct_id: i32,
    pub _type: HSType,
    pub unk1: u32,
    pub unk2: u32,
}

impl HSStructHeader {
    pub fn read(&mut self, reader: &mut BufReader<File>, header: &HSHeader) -> Result<()> {
        self.name = read_string(reader, header)?;
        self.unk0 = reader.read_u32::<BE>()?;
        self.struct_id = reader.read_i32::<BE>()?;
        self._type = HSType::try_from(reader.read_u32::<BE>()? as u8)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Unknown type"))?;
        self.unk1 = reader.read_u32::<BE>()?;
        self.unk2 = reader.read_u32::<BE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct HSStructMember {
    pub header: HSStructHeader,
    pub index: i32,
}

impl HSStructMember {
    pub fn read(&mut self, reader: &mut BufReader<File>, header: &HSHeader) -> Result<()> {
        self.header.read(reader, header)?;
        self.index = reader.read_i32::<BE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct HSStructBlock {
    pub header: HSStructHeader,
    pub member_count: u32,
    pub extend_count: u32,
    pub extended_structs: Vec<String>,
    pub members: Vec<HSStructMember>,
}

impl HSStructBlock {
    pub fn read(&mut self, reader: &mut BufReader<File>, header: &HSHeader) -> Result<()> {
        self.header.read(reader, header)?;
        self.member_count = reader.read_u32::<BE>()?;

        if header.compatability.contains(HSCompatability::STRUCTURES) {
            self.extend_count = reader.read_u32::<BE>()?;
            self.extended_structs = (0..self.extend_count)
                .map(|_| read_string(reader, header))
                .collect::<Result<Vec<_>>>()?;
        }

        self.members = (0..self.member_count)
            .map(|_| {
                let mut member = HSStructMember::default();
                member.read(reader, header)?;
                Ok(member)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    }
}
