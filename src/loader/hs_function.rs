use super::{
    hs_constant::HSConstant, hs_debug::HSFunctionDebugInfo, hs_header::HSHeader,
    hs_instruction::HSInstruction, hs_opcodes::HSOpArgMode,
};
use crate::{
    common::errors::HkscError,
    common::extensions::{BufReaderExt, HeaderReadable},
};

use bitflags::bitflags;
use byteorder::{ByteOrder, ReadBytesExt};
use colored::Colorize;
use std::{fmt::Display, io::SeekFrom};

bitflags! {
    #[derive(Default)]
    /// Flag that sets if a function supports variadic arguments.
    pub struct HSVarArg : u8 {
        /// Function has no variadic arguments.
        const NONE = 0;
        /// Function has variadic arguments.
        const HASARG = 1;
        /// Function only has variadic arguments.
        const ISVARARG = 2;
        /// Function needs variadic arguments.
        const NEEDSARG = 4;
    }
}

impl Display for HSVarArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.0 == 0 {
            return write!(f, "NONE");
        }
        write!(f, "{:?}", self.0)
    }
}

#[derive(Default)]
/// Function definition in a `HavokScript` file.
pub struct HSFunction {
    /// Number of up values in the function.
    /// Important: These may not have a name in debug info.
    pub up_value_count: u32,
    /// Number of parameters in the function.
    pub param_count: u32,
    /// Flags for variadic arguments.
    pub var_arg: HSVarArg,
    /// Number of slots (registers) required for the function.
    pub slot_count: u32,
    /// Number of instructions in the function.
    pub instruction_count: u64,
    /// Instructions in the function.
    pub instructions: Vec<HSInstruction>,
    /// Number of constants in the function.
    pub constant_count: u32,
    /// Constants in the function.
    pub constants: Vec<HSConstant>,
    /// Flag that determines if the function has debug information.
    pub has_debug_info: bool,
    /// Debug information for the function.
    pub debug_info: HSFunctionDebugInfo,
    /// Number of child functions in the function.
    pub function_count: u32,
    /// Child functions in the function.
    pub child_functions: Vec<HSFunction>,
    /// Offset of the function in the file.
    pub function_offset: u64,
}

impl HeaderReadable for HSFunction {
    fn read<T: ByteOrder>(
        &mut self,
        reader: &mut impl BufReaderExt,
        header: &HSHeader,
    ) -> Result<(), HkscError> {
        self.up_value_count = reader.read_u32::<T>()?;
        self.param_count = reader.read_u32::<T>()?;
        self.var_arg = HSVarArg::from_bits_truncate(reader.read_u8()?);
        self.slot_count = reader.read_u32::<T>()?;
        self.instruction_count = reader.read_u64::<T>()?;

        // This aligns the reader to the next 4 byte boundary.
        let current_pos = reader.stream_position()?;
        let instruction_size = u64::from(header.instruction_size); // I hate rust.
        let aligned_pos = (current_pos + (instruction_size - 1)) & !(instruction_size - 1);
        reader.seek(SeekFrom::Start(aligned_pos))?;

        self.instructions = reader.read_enumerable::<HSInstruction, T>(self.instruction_count)?;
        self.constant_count = reader.read_u32::<T>()?;
        self.constants =
            reader.read_header_enumerable::<HSConstant, T>(self.constant_count.into(), header)?;
        self.has_debug_info = reader.read_u32::<T>()? != 0;
        if self.has_debug_info {
            self.debug_info.read::<T>(reader, header)?;
        };
        self.function_count = reader.read_u32::<T>()?;
        self.child_functions =
            reader.read_header_enumerable::<HSFunction, T>(self.function_count.into(), header)?;
        self.function_offset = reader.stream_position()?;
        Ok(())
    }
}

impl Display for HSFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_debug_info && !self.debug_info.function_name.is_empty() {
            writeln!(
                f,
                "{} {}{}",
                "[Function:".green(),
                self.debug_info.function_name.bright_cyan(),
                "]".green()
            )?;
        } else {
            writeln!(
                f,
                "{} {}{}",
                "[Function:".green(),
                self.function_offset.to_string().bright_cyan(),
                "]".green()
            )?;
        }
        writeln!(
            f,
            "{} {}",
            "- UpValue Count:".yellow(),
            self.up_value_count.to_string().bright_cyan()
        )?;
        writeln!(
            f,
            "{} {}",
            "- Parameter Count:".yellow(),
            self.param_count.to_string().bright_cyan()
        )?;
        writeln!(
            f,
            "{} {}",
            "- Variadic Argument Type:".yellow(),
            self.var_arg.to_string().bright_cyan()
        )?;
        writeln!(
            f,
            "{} {}",
            "- Slot Count:".yellow(),
            self.slot_count.to_string().bright_cyan()
        )?;

        writeln!(f, "{}", "Instructions:".bright_blue())?;
        for inst in &self.instructions {
            write!(
                f,
                "{} {}{} ",
                "-".yellow(),
                inst.mode.to_string().yellow(),
                ":".yellow()
            )?;
            for (i, arg) in inst.args.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                match arg.mode {
                    HSOpArgMode::CONST => {
                        write!(f, "{}", "CONST(".bright_cyan())?;
                        #[allow(clippy::cast_sign_loss)]
                        if let Some(constant) = self.constants.get(arg.value as usize) {
                            write!(f, "{constant}")?;
                        }
                        write!(f, "{}", ")".bright_cyan())?;
                    }
                    _ => write!(
                        f,
                        "{}{}{}{}",
                        arg.mode.to_string().bright_cyan(),
                        "(".bright_cyan(),
                        arg.value.to_string().bright_blue(),
                        ")".bright_cyan()
                    )?,
                }
            }
            writeln!(f)?;
        }

        if self.has_debug_info {
            writeln!(f, "{}", "Debug Info:".bright_blue())?;
            write!(f, "{}", self.debug_info)?;
        }

        writeln!(f)?;
        for func in &self.child_functions {
            write!(f, "{func}")?;
        }
        Ok(())
    }
}
