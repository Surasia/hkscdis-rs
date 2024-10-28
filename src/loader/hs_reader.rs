use crate::{common::extensions::BufReaderExt, errors::HkscError};

use super::{hs_constant::HSValue, hs_header::HSHeader};
use byteorder::{ReadBytesExt, BE};
use std::io::BufRead;

pub fn read_number(reader: &mut impl ReadBytesExt, header: &HSHeader) -> Result<HSValue, HkscError> {
    match header.number_size {
        4 => Ok(HSValue::Number(f64::from(reader.read_f32::<BE>()?))),
        8 => Ok(HSValue::Number(reader.read_f64::<BE>()?)),
        _ => Err(HkscError::InvalidNumberSize(header.number_size)),
    }
}

pub fn read_string<R>(reader: &mut R, header: &HSHeader) -> Result<String, HkscError>
where
    R: BufRead + BufReaderExt
{
    let size = match header.t_size {
        4 => reader.read_u32::<BE>()? as usize,
        8 => usize::try_from(reader.read_u64::<BE>()?)?,
        _ => return Err(HkscError::InvalidStringSize(header.t_size)) 
    };

    reader.read_fixed_string(size)
}
