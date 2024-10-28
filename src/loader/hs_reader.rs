use crate::{common::errors::HkscError, common::extensions::BufReaderExt};

use super::{hs_constant::HSValue, hs_header::HSHeader};
use byteorder::{ReadBytesExt, BE};
use std::io::BufRead;

/// Reads a number from the given reader based on the number size specified in the header.
///
/// # Arguments
///
/// * `reader` - A mutable reference to an object that implements the `ReadBytesExt` trait.
/// * `header` - A reference to the `HSHeader` which contains metadata about the number size.
///
/// # Returns
///
/// * `Ok(HSValue::Number)` - If the number is successfully read and converted to `HSValue`.
/// * `Err(HkscError::InvalidNumberSize)` - If the number size specified in the header is invalid.
pub fn read_number(
    reader: &mut impl ReadBytesExt,
    header: &HSHeader,
) -> Result<HSValue, HkscError> {
    match header.number_size {
        4 => Ok(HSValue::Number(f64::from(reader.read_f32::<BE>()?))),
        8 => Ok(HSValue::Number(reader.read_f64::<BE>()?)),
        _ => Err(HkscError::InvalidNumberSize(header.number_size)),
    }
}

/// Reads a string from the given reader based on the string size specified in the header.
///
/// # Arguments
///
/// * `reader` - A mutable reference to an object that implements both the `BufRead` and `BufReaderExt` traits.
/// * `header` - A reference to the `HSHeader` which contains metadata about the string size.
///
/// # Returns
///
/// * `Ok(String)` - If the string is successfully read.
/// * `Err(HkscError::InvalidStringSize)` - If the string size specified in the header is invalid.
pub fn read_string<R>(reader: &mut R, header: &HSHeader) -> Result<String, HkscError>
where
    R: BufRead + BufReaderExt,
{
    let size = match header.t_size {
        4 => reader.read_u32::<BE>()? as usize,
        8 => usize::try_from(reader.read_u64::<BE>()?)?,
        _ => return Err(HkscError::InvalidStringSize(header.t_size)),
    };

    reader.read_fixed_string(size)
}
