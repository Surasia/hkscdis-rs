use super::{hs_constant::HSValue, hs_header::HSHeader};
use byteorder::{ReadBytesExt, BE};
use std::io::{self};

pub fn read_number(reader: &mut impl ReadBytesExt, header: &HSHeader) -> io::Result<HSValue> {
    match header.number_size {
        4 => Ok(HSValue::Number(f64::from(reader.read_f32::<BE>()?))),
        8 => Ok(HSValue::Number(reader.read_f64::<BE>()?)),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid number size",
        )),
    }
}

pub fn read_string(reader: &mut impl ReadBytesExt, header: &HSHeader) -> io::Result<String> {
    let size = match header.t_size {
        4 => reader.read_u32::<BE>()? as usize,
        8 => reader.read_u64::<BE>()? as usize,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid string size",
            ))
        }
    };

    let mut buffer = vec![0; size];
    reader.read_exact(&mut buffer)?;

    String::from_utf8(buffer)
        .map(|s| s.trim_end_matches('\0').to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
