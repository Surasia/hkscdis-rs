use super::{hs_header::HSHeader, hs_opcodes::HSType, hs_reader::read_string};
use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, HeaderReadable},
};

use byteorder::{ByteOrder, ReadBytesExt};
use colored::Colorize;
use std::fmt::Display;

#[derive(Default, Debug)]
/// Information about a slot in a structure, containing type and metadata
pub struct HSStructSlot {
    /// Name of the struct slot
    pub name: String,
    /// Unique identifier for this struct within the file
    pub struct_id: u64,
    /// Type of the struct slot
    pub type_: HSType,
    /// Reserved for VM
    pub reserved: u32,
    /// Position/offset within the structure
    pub position: u64,
}

impl HeaderReadable for HSStructSlot {
    fn read<T: ByteOrder>(
        &mut self,
        reader: &mut impl BufReaderExt,
        header: &HSHeader,
    ) -> Result<(), HkscError> {
        self.name = read_string::<T>(reader, header)?;
        self.struct_id = reader.read_u64::<T>()?;
        let type_byte = u8::try_from(reader.read_u32::<T>()?)?;
        self.type_ = HSType::try_from(type_byte).map_err(|_| HkscError::UnknownType(type_byte))?;
        self.reserved = reader.read_u32::<T>()?;
        self.position = reader.read_u64::<T>()?;
        Ok(())
    }
}

impl Display for HSStructSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            format!("{:?}", self.type_).yellow(),
            self.name.bright_cyan()
        )
    }
}

#[derive(Default, Debug)]
/// A block containing structure information including slots and inheritance
pub struct HSStructPrototype {
    /// Name of the structure
    pub name: String,
    /// Unique identifier for this structure
    pub id: u64,
    /// Whether this structure has metadata
    pub has_meta: bool,
    /// Whether this structure has a proxy
    pub has_proxy: bool,
    /// Number of slots in this structure
    pub slot_count: u64,
    /// List of structure slots
    pub slots: Vec<HSStructSlot>,
    /// Number of structures this structure extends/inherits from
    pub inherited_count: u32,
    /// Names of structures that this structure extends/inherits from
    pub inherited_structs: Vec<String>,
}

impl HSStructPrototype {
    pub fn read<T: ByteOrder>(
        &mut self,
        reader: &mut impl BufReaderExt,
        header: &HSHeader,
        enable_inheritance: bool,
    ) -> Result<(), HkscError> {
        self.id = reader.read_u64::<T>()?;
        self.has_meta = reader.read_u32::<T>()? != 0;
        self.has_proxy = reader.read_u32::<T>()? != 0;
        self.slot_count = reader.read_u64::<T>()?;
        if enable_inheritance {
            self.inherited_count = reader.read_u32::<T>()?;
            self.inherited_structs = (0..self.inherited_count)
                .map(|_| read_string::<T>(reader, header))
                .collect::<Result<Vec<_>, HkscError>>()?;
        }
        self.slots = reader.read_header_enumerable::<HSStructSlot, T>(self.slot_count, header)?;
        Ok(())
    }
}

impl Display for HSStructPrototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "[Structure: ".green(), self.name.bright_cyan(),)?;
        if !self.inherited_structs.is_empty() {
            write!(
                f,
                "{} {}",
                "INHERITED FROM".green(),
                self.inherited_structs.join(", ").bright_cyan()
            )?;
        }
        write!(f, "{}", "]".green())?;
        Ok(())
    }
}
