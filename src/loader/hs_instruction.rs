use super::hs_opcodes::{
    HSMode, HSOpArgMode, HSOpArgModeA, HSOpArgModeBC, HSOpCode, HSOpMode, OP_TABLE,
};
use crate::common::{errors::HkscError, extensions::BufReaderExt, extensions::Readable};

use byteorder::{ByteOrder, ReadBytesExt};

// Bit masks for instruction parsing
const OPCODE_MASK: u32 = 0x7F << 25; // Highest 7 bits
const A_ARG_MASK: i32 = 0xFF; // Lowest 8 bits
const B_ARG_MASK: i32 = 0xFF << 17; // Bits 17-24
const B_ARG_EXTMASK: i32 = 0x1FF << 17; // Bits 17-25 (including extension)
const C_ARG_MASK: i32 = 0xFF << 8; // Bits 8-15
const C_ARG_EXTMASK: i32 = 0x1FF << 8; // Bits 8-16 (including extension)
const NON_ABC_B_MASK: i32 = 0x1FFFF << 8; // Bits 8-25 for non-ABC formats
const REG_CONST_THRESHOLD: i32 = 0x100; // Values >= 256 indicate constants

#[derive(Debug)]
/// Represents a single argument for a `HavokScript` instruction. Each argument has both
/// a mode (indicating how it should be interpreted) and a raw value.
pub struct HSInstructionArg {
    /// The mode determines how the value should be interpreted.
    pub mode: HSOpArgMode,
    /// The raw value of the argument. Interpretation depends on the mode.
    pub value: i32,
}

#[derive(Default)]
/// Represents a single instruction in the `HavokScript` bytecode.
/// Each instruction consists of an opcode and up to three arguments.
pub struct HSInstruction {
    /// The operation code that determines what this instruction does.
    /// This defines the basic operation being performed.
    pub mode: HSOpCode,
    /// Vector containing the instruction's arguments (typically 0-3 arguments).
    /// The meaning and number of arguments depends on the opcode.
    pub args: Vec<HSInstructionArg>,
}

impl Readable for HSInstruction {
    /// Reads and decodes a single instruction from the bytecode stream.
    /// `HavokScript` instructions are encoded as 32-bit integers in big-endian format.
    fn read<T: ByteOrder>(&mut self, reader: &mut impl BufReaderExt) -> Result<(), HkscError> {
        let raw = reader.read_i32::<T>()?;
        #[allow(clippy::cast_sign_loss)]
        let op_entry = &OP_TABLE[((raw as u32 & OPCODE_MASK) >> 25) as usize];

        self.mode = op_entry.op_code.clone();
        self.read_op_a(raw, op_entry);
        self.read_op_bc(raw, op_entry);
        Ok(())
    }
}

impl HSInstruction {
    /// Reads the 'A' argument from the raw instruction data.
    /// The A argument is always stored in the lowest 8 bits of the instruction.
    /// This argument typically represents the destination register for operations.
    fn read_op_a(&mut self, raw: i32, modes: &HSMode) {
        let mode = if modes.op_mode_a == HSOpArgModeA::UNUSED {
            HSOpArgMode::NUMBER
        } else {
            HSOpArgMode::REG
        };

        let value = raw & A_ARG_MASK;
        self.args.push(HSInstructionArg { mode, value });
    }

    /// Reads the 'B' argument for instructions that use the ABC instruction format.
    /// The B argument is located in bits 17-25 of the instruction.
    ///
    /// Argument interpretation varies based on mode:
    /// * REGCONST: Value can represent either a register (<256) or constant (≥256)
    /// * OFFSET: Used for relative jumps, sign extended
    /// * REG: Represents a register index
    /// * CONST: Represents a constant pool index
    /// * NUMBER: Raw numeric value
    fn read_op_abc_b(&mut self, raw: i32, modes: &HSMode) {
        let (mode, value) = match modes.op_mode_b {
            HSOpArgModeBC::NUMBER => (HSOpArgMode::NUMBER, (raw & B_ARG_MASK) >> 17),
            HSOpArgModeBC::OFFSET => (HSOpArgMode::NUMBER, (raw & B_ARG_EXTMASK) >> 17),
            HSOpArgModeBC::REG => (HSOpArgMode::REG, (raw & B_ARG_MASK) >> 17),
            HSOpArgModeBC::REGCONST => {
                let value = (raw & B_ARG_EXTMASK) >> 17;
                if value < REG_CONST_THRESHOLD {
                    // Values below 256 indicate a register reference
                    (HSOpArgMode::REG, value)
                } else {
                    // Values 256 and above indicate a constant reference
                    (HSOpArgMode::CONST, value & 0xFF)
                }
            }
            HSOpArgModeBC::CONST => (HSOpArgMode::CONST, (raw >> 17) & 0xFF),
            HSOpArgModeBC::UNUSED => (HSOpArgMode::CONST, 0),
        };
        self.args.push(HSInstructionArg { mode, value });
    }

    /// Reads the 'C' argument for instructions using the ABC instruction format.
    /// The C argument is located in bits 8-16 of the instruction.
    ///
    /// Interpretation follows the same rules as the B argument:
    /// * REGCONST: Value represents register (<256) or constant (≥256)
    /// * OFFSET: Used for relative jumps
    /// * REG: Register index
    /// * CONST: Constant pool index
    /// * NUMBER: Raw numeric value
    fn read_op_abc_c(&mut self, raw: i32, modes: &HSMode) {
        let (mode, value) = match modes.op_mode_c {
            HSOpArgModeBC::NUMBER => (HSOpArgMode::NUMBER, (raw & C_ARG_MASK) >> 8),
            HSOpArgModeBC::OFFSET => (HSOpArgMode::NUMBER, (raw & C_ARG_EXTMASK) >> 8),
            HSOpArgModeBC::REG => (HSOpArgMode::REG, (raw & C_ARG_MASK) >> 8),
            HSOpArgModeBC::REGCONST => {
                let value = (raw & C_ARG_EXTMASK) >> 8;
                if value < REG_CONST_THRESHOLD {
                    (HSOpArgMode::REG, value)
                } else {
                    (HSOpArgMode::CONST, value & 0xFF)
                }
            }
            HSOpArgModeBC::CONST => (HSOpArgMode::CONST, (raw >> 8) & 0xFF),
            HSOpArgModeBC::UNUSED => (HSOpArgMode::CONST, 0),
        };
        self.args.push(HSInstructionArg { mode, value });
    }

    /// Reads the 'B' argument for non-ABC instruction formats (like `AsBx`).
    /// For these formats, the B argument spans bits 8-25 and is treated as
    /// a single field rather than two separate arguments.
    fn read_op_non_abc_b(&mut self, raw: i32, modes: &HSMode) {
        let mut value = (raw & NON_ABC_B_MASK) >> 8;
        let mode = match modes.op_mode_b {
            HSOpArgModeBC::NUMBER | HSOpArgModeBC::OFFSET => HSOpArgMode::NUMBER,
            HSOpArgModeBC::CONST => HSOpArgMode::CONST,
            _ => HSOpArgMode::default(),
        };

        // For AsBx format instructions, adjust the value by subtracting 0xFFFF
        // to support negative offsets
        if modes.op_mode == HSOpMode::ASBX {
            value = value.wrapping_sub(0xFFFF);
        }
        self.args.push(HSInstructionArg { mode, value });
    }

    /// Handles reading both B and C arguments based on the instruction's mode.
    /// Different instruction formats interpret these bits differently:
    /// * ABC format: Bits are split into two separate arguments (B and C)
    /// * Other formats: Bits are treated as a single larger argument (B only)
    fn read_op_bc(&mut self, raw: i32, modes: &HSMode) {
        if modes.op_mode_b != HSOpArgModeBC::UNUSED {
            if modes.op_mode == HSOpMode::ABC {
                self.read_op_abc_b(raw, modes);
            } else {
                self.read_op_non_abc_b(raw, modes);
            }
        }

        if modes.op_mode == HSOpMode::ABC && modes.op_mode_c != HSOpArgModeBC::UNUSED {
            self.read_op_abc_c(raw, modes);
        }
    }
}
