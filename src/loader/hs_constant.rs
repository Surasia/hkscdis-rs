use super::{
    hs_header::HSHeader,
    hs_opcodes::HSType,
    hs_reader::{read_number, read_string},
};
use byteorder::{ReadBytesExt, BE};
use std::{
    fs::File,
    io::{self, BufReader},
};

#[derive(Default, Debug, Clone)]
pub enum HSValue {
    #[default]
    Nil,
    Boolean(bool),
    LightUserData(u64),
    Number(f64),
    String(String),
    Ui64(u64),
}

impl HSValue {
    pub fn format_value(&self) -> String {
        match self {
            HSValue::String(s) => format!("\"{}\"", s),
            HSValue::Number(n) => format!("{}", n),
            HSValue::LightUserData(n) => format!("{}", n),
            HSValue::Ui64(n) => format!("{}", n),
            HSValue::Boolean(b) => format!("{}", b),
            HSValue::Nil => "Nil".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HSConstant {
    pub type_: HSType,
    pub value: Option<HSValue>,
}

impl HSConstant {
    pub fn read(&mut self, reader: &mut BufReader<File>, header: &HSHeader) -> io::Result<()> {
        let type_byte = reader.read_u8()?;
        self.type_ = HSType::try_from(type_byte)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unknown type"))?;

        self.value = match self.type_ {
            HSType::TNIL => Some(HSValue::Nil),
            HSType::TLIGHTUSERDATA => match header.t_size {
                4 => Some(HSValue::LightUserData(reader.read_u32::<BE>()? as u64)),
                8 => Some(HSValue::LightUserData(reader.read_u64::<BE>()?)),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid LIGHTUSERDATA type!",
                    ))
                }
            },
            HSType::TBOOLEAN => Some(HSValue::Boolean(reader.read_u8()? != 0)),
            HSType::TSTRING => Some(HSValue::String(read_string(reader, header)?)),
            HSType::TNUMBER => Some(read_number(reader, header)?),
            HSType::TUI64 => Some(HSValue::Ui64(reader.read_u64::<BE>()?)),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown constant type!",
                ))
            }
        };

        Ok(())
    }
}
