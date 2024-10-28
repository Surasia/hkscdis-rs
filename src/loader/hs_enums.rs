use byteorder::{ReadBytesExt, BE};
use std::io::BufRead;

use crate::{common::extensions::{BufReaderExt, Readable}, errors::HkscError};

#[derive(Default)]
/// Represents an enum in a `HavokScript` file.
pub struct HSEnum {
    /// Value of the enum (index).
    pub value: u32,
    /// Length of the name of the enum.
    length: u32,
    /// Name of the enum.
    pub name: String,
}

impl Readable for  HSEnum {
    fn read<R>(&mut self, reader: &mut R) -> Result<(), HkscError>
    where
        R: BufRead + BufReaderExt
    {
        self.value = reader.read_u32::<BE>()?;
        self.length = reader.read_u32::<BE>()?;
        self.name = reader.read_fixed_string(self.length as usize)?;
        Ok(())
    }
}
