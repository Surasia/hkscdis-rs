use super::{hs_header::HSHeader, hs_reader::read_string};
use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, HeaderReadable},
};

use byteorder::{ByteOrder, ReadBytesExt};

#[derive(Debug, Default)]
/// Local variable information for a function.
pub struct HSFunctionDebugInfoLocals {
    /// Name of the local variable.
    pub local_name: String,
    /// Start line of the local variable.
    start: u32,
    /// End line of the local variable.
    end: u32,
}

impl HeaderReadable for HSFunctionDebugInfoLocals {
    fn read<T: ByteOrder>(
        &mut self,
        reader: &mut impl BufReaderExt,
        header: &HSHeader,
    ) -> Result<(), HkscError> {
        self.local_name = read_string::<T>(reader, header)?;
        self.start = reader.read_u32::<T>()?;
        self.end = reader.read_u32::<T>()?;
        Ok(())
    }
}

#[derive(Default)]
/// Debug information for a function, containing data to read local variables and up values.
pub struct HSFunctionDebugInfo {
    /// Number of lines in the function.
    pub line_count: u32,
    /// Number of local variables in the function.
    pub locals_count: u32,
    /// Number of up values in the function.
    pub up_value_count: u32,
    /// Start line of the function.
    pub line_begin: u32,
    /// End line of the function.
    pub line_end: u32,
    /// Path to the file containing the function.
    pub path: String,
    /// Name of the function.
    pub function_name: String,
    /// Lines in the function.
    pub lines: Vec<u32>,
    /// Local variables in the function.
    pub locals: Vec<HSFunctionDebugInfoLocals>,
    /// Up values in the function.
    pub up_values: Vec<String>,
}

impl HeaderReadable for HSFunctionDebugInfo {
    fn read<T: ByteOrder>(
        &mut self,
        reader: &mut impl BufReaderExt,
        header: &HSHeader,
    ) -> Result<(), HkscError> {
        self.line_count = reader.read_u32::<T>()?;
        self.locals_count = reader.read_u32::<T>()?;
        self.up_value_count = reader.read_u32::<T>()?;
        self.line_begin = reader.read_u32::<T>()?;
        self.line_end = reader.read_u32::<T>()?;
        self.path = read_string::<T>(reader, header)?;
        self.function_name = read_string::<T>(reader, header)?;

        self.lines = (0..self.line_count)
            .map(|_| reader.read_u32::<T>())
            .collect::<Result<_, _>>()?;

        self.locals = reader.read_header_enumerable::<HSFunctionDebugInfoLocals, T>(
            self.locals_count.into(),
            header,
        )?;

        self.up_values = (0..self.up_value_count)
            .map(|_| read_string::<T>(reader, header))
            .collect::<Result<_, _>>()?;

        Ok(())
    }
}
