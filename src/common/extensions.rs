//! Extensions to `BufReader`.
//!
//! Implements additional reading methods for `BufReader`:
//! * `read_fixed_string`: Reads a fixed-length string from a buffer.
//! * `read_enumerable`: Reads multiple instances of a type that implements `Readable` into a `Vec`.
//! * `read_header_enumerable`: Reads multiple instances of a type that implements `HeaderReadable` into a `Vec`.
//!
//! These extensions require `Read + Seek` bounds.

use crate::{common::errors::HkscError, loader::hs_header::HSHeader};
use byteorder::ByteOrder;
use std::io::{BufRead, BufReader, Read, Seek};

/// `Readable` trait that ensures a `read` method is declared.
pub trait Readable {
    /// Reads data from a reader implementing `BufRead`, `BufReaderExt`, and `Seek`.
    fn read<T: ByteOrder>(&mut self, reader: &mut impl BufReaderExt) -> Result<(), HkscError>;
}

/// `HeaderReadable` trait that ensures a `read` method is declared with a `HSHeader` argument.
pub trait HeaderReadable {
    /// Reads data from a reader implementing `BufRead`, `BufReaderExt`, and `Seek`, using header information.
    fn read<T: ByteOrder>(
        &mut self,
        reader: &mut impl BufReaderExt,
        header: &HSHeader,
    ) -> Result<(), HkscError>;
}

/// Extension trait for `BufReader` to add custom reading methods.
pub trait BufReaderExt: BufRead
where
    Self: Seek,
{
    /// Reads a fixed-length UTF-8 encoded string.
    ///
    /// # Arguments
    /// * `length` - Number of bytes to read.
    ///
    /// # Returns
    /// The read string on success, or an error on failure.
    fn read_fixed_string<T: ByteOrder>(&mut self, length: usize) -> Result<String, HkscError> {
        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer)?;

        if buffer == [255, 255, 255, 255] {
            return Ok(String::new()); // Return empty string if all bytes are 0xFF.
        }

        let string = String::from_utf8(buffer)?;

        Ok(string)
    }

    /// Reads multiple instances of a type into a `Vec`.
    ///
    /// # Arguments
    /// * `count` - Number of instances to read.
    ///
    /// # Returns
    /// `Vec` of read instances on success, or an error on failure.
    fn read_enumerable<T: Default + Readable, R: ByteOrder>(
        &mut self,
        count: u64,
    ) -> Result<Vec<T>, HkscError>
    where
        Self: Sized,
        Vec<T>: FromIterator<T>,
    {
        let enumerables = (0..count)
            .map(|_| -> Result<T, HkscError> {
                let mut enumerable = T::default();
                enumerable.read::<R>(self)?;
                Ok(enumerable)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(enumerables)
    }

    /// Reads multiple instances of a type into a `Vec`, using header information.
    ///
    /// # Arguments
    /// * `count` - Number of instances to read.
    /// * `header` - The `HSHeader` containing format information.
    ///
    /// # Returns
    /// `Vec` of read instances on success, or an error on failure.
    fn read_header_enumerable<T: Default + HeaderReadable, R: ByteOrder>(
        &mut self,
        count: u64,
        header: &HSHeader,
    ) -> Result<Vec<T>, HkscError>
    where
        Self: Sized,
        Vec<T>: FromIterator<T>,
    {
        let enumerables = (0..count)
            .map(|_| -> Result<T, HkscError> {
                let mut enumerable = T::default();
                enumerable.read::<R>(self, header)?;
                Ok(enumerable)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(enumerables)
    }
}

impl<R: Read + Seek> BufReaderExt for BufReader<R> {}
