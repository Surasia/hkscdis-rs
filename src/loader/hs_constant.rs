use super::{
    hs_header::HSHeader,
    hs_opcodes::HSType,
    hs_reader::{read_number, read_string},
};
use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, HeaderReadable},
};

use byteorder::{ByteOrder, ReadBytesExt};
use std::fmt::{Display, Formatter};

/// Possible values for a (valid) `HavokScript` constant.
pub enum HSValue {
    /// Null pointer.
    Nil,
    /// Boolean, usually stored in 32 bits.
    Boolean(bool),
    /// Data that interacts with engine. Can be either 32 bit or 64 bit.
    LightUserData(u64),
    /// Regular lua number, stored as a 64 bit float.
    Number(f64),
    /// Null terminated string.
    String(String),
    /// 64 bit unsigned integer.
    Ui64(u64),
}

#[derive(Default)]
/// Represents a constant, containing a type and value.
pub struct HSConstant {
    /// Type of constant, dictates how it is read
    pub type_: HSType,
    /// Value of the constant, mapping to a `HSValue` enum.
    pub value: Option<HSValue>,
}

impl Display for HSConstant {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.value {
            Some(HSValue::String(s)) => write!(f, "\"{s}\""),
            Some(HSValue::Number(n)) => write!(f, "{n}"),
            Some(HSValue::LightUserData(n) | HSValue::Ui64(n)) => write!(f, "{n}"),
            Some(HSValue::Boolean(b)) => write!(f, "{b}"),
            Some(HSValue::Nil) => write!(f, "nil"),
            None => write!(f, ""),
        }
    }
}

impl HeaderReadable for HSConstant {
    fn read<T: ByteOrder>(
        &mut self,
        reader: &mut impl BufReaderExt,
        header: &HSHeader,
    ) -> Result<(), HkscError> {
        let type_byte = reader.read_u8()?;
        self.type_ = HSType::try_from(type_byte).map_err(|_| HkscError::UnknownType(type_byte))?;

        self.value = match self.type_ {
            HSType::TNIL => Some(HSValue::Nil),
            HSType::TLIGHTUSERDATA => match header.t_size {
                4 => Some(HSValue::LightUserData(reader.read_u32::<T>()?.into())),
                8 => Some(HSValue::LightUserData(reader.read_u64::<T>()?)),
                _ => return Err(HkscError::InvalidLightUserDataSize(header.t_size)),
            },
            HSType::TBOOLEAN => Some(HSValue::Boolean(reader.read_u8()? != 0)),
            HSType::TSTRING => Some(HSValue::String(read_string::<T>(reader, header)?)),
            HSType::TNUMBER => Some(read_number::<T>(reader, header)?),
            HSType::TUI64 => Some(HSValue::Ui64(reader.read_u64::<T>()?)),
            _ => return Err(HkscError::UnsupportedConstantType(type_byte)),
        };

        Ok(())
    }
}
