use crate::common::errors::HkscError;

use bitflags::bitflags;
use byteorder::{ReadBytesExt, BE, LE};
use colored::Colorize;
use std::{fmt::Display, fs::File, io::BufReader};

bitflags! {
    #[derive(Default)]
    /// Flags for enabling `HavokScript` features, such as global memoization.
    pub struct HSFeatures: u8 {
        /// Enable memoization.
        const MEMOIZATION = 1 << 0;
        /// Enables havok structures, creating in lua using `hmake`.
        const STRUCTURES = 1 << 1;
        /// Enable self references.
        const SELF = 1 << 2;
        /// Enable double precision numbers.
        const DOUBLES = 1 << 3;
        /// Enable native integers. (Does not respect t_size)
        const NATIVEINT = 1 << 4;
    }
}

impl Display for HSFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "[".bright_cyan())?;
        let mut first = true;
        for (name, _) in self.iter_names() {
            if !first {
                write!(f, "{} ", ",".bright_cyan())?;
            }
            write!(f, "{}", name.bright_cyan())?;
            first = false;
        }
        write!(f, "{}", "]".bright_cyan())
    }
}

#[derive(Default)]
/// Header of a `HavokScript` file.
pub struct HSHeader {
    /// Magic number for script files. (0x61754C1B / "\1BLua")
    pub magic: u32,
    /// Version of the lua file. (0x51 is 5.1)
    pub version: u8,
    /// Format version of the file. (should be 14)
    pub fmt: u8,
    /// Endianness of the file. (0 = big endian, 1 = little endian)
    pub is_little_endian: bool,
    /// WORD size of target system.
    pub int_size: u8,
    /// Type size of target system.
    pub t_size: u8,
    /// Instruction size to extract `OpCodes` from.
    pub instruction_size: u8,
    /// Number (long long) size of target system.
    pub number_size: u8,
    /// Whether to use integer or floats for numbers.
    pub is_integer: bool,
    /// Feature flags for the file.
    pub features: HSFeatures,
    /// Unknown?
    pub shared: u8,
    /// Count of enums to read.
    pub enum_count: u32,
}

impl HSHeader {
    pub fn read(&mut self, reader: &mut BufReader<File>) -> Result<(), HkscError> {
        self.magic = reader.read_u32::<LE>()?;
        if self.magic != 1_635_077_147 {
            return Err(HkscError::IncorrectMagicNumber(self.magic));
        }
        self.version = reader.read_u8()?;
        if self.version != 0x51 {
            return Err(HkscError::IncorrectVersionNumber(self.version));
        }
        self.fmt = reader.read_u8()?;
        if self.fmt != 14 {
            return Err(HkscError::IncorrectFormatNumber(self.fmt));
        }
        self.is_little_endian = reader.read_u8()? != 0;
        self.int_size = reader.read_u8()?;
        self.t_size = reader.read_u8()?;
        self.instruction_size = reader.read_u8()?;
        self.number_size = reader.read_u8()?;
        self.is_integer = reader.read_u8()? != 0;
        self.features = HSFeatures::from_bits_truncate(reader.read_u8()?);
        self.shared = reader.read_u8()?;
        if self.is_little_endian {
            self.enum_count = reader.read_u32::<LE>()?;
        } else {
            self.enum_count = reader.read_u32::<BE>()?;
        }
        Ok(())
    }
}

impl Display for HSHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}",
            "- Is Little Endian:".yellow(),
            self.is_little_endian.to_string().bright_cyan()
        )?;
        writeln!(
            f,
            "{} {}",
            "- Integer Size:".yellow(),
            self.int_size.to_string().bright_cyan()
        )?;
        writeln!(
            f,
            "{} {}",
            "- Type Size:".yellow(),
            self.t_size.to_string().bright_cyan()
        )?;
        writeln!(
            f,
            "{} {}",
            "- Instruction Size:".yellow(),
            self.instruction_size.to_string().bright_cyan()
        )?;
        writeln!(
            f,
            "{} {}",
            "- Number Size:".yellow(),
            self.number_size.to_string().bright_cyan()
        )?;
        writeln!(
            f,
            "{} {}",
            "- Is Using Integer:".yellow(),
            self.is_integer.to_string().bright_cyan()
        )?;
        writeln!(f, "{} {}", "- Extensions:".yellow(), self.features)
    }
}
