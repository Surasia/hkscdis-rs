use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, Readable},
};

use byteorder::{ByteOrder, ReadBytesExt};

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

impl Readable for HSEnum {
    fn read<T: ByteOrder>(&mut self, reader: &mut impl BufReaderExt) -> Result<(), HkscError> {
        self.value = reader.read_u32::<T>()?;
        self.length = reader.read_u32::<T>()?;
        self.name = reader.read_fixed_string::<T>(self.length as usize)?;
        Ok(())
    }
}
