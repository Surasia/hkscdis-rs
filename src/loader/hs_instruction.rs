use super::hs_opcodes::{
    HSMode, HSOpArgMode, HSOpArgModeA, HSOpArgModeBC, HSOpCode, HSOpMode, OP_TABLE,
};
use byteorder::{ReadBytesExt, BE};
use std::io;
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone)]
pub struct HSInstructionArg {
    pub mode: HSOpArgMode,
    pub value: u32,
}

#[derive(Debug, Clone, Default)]
pub struct HSInstruction {
    pub mode: HSOpCode,
    pub args: Vec<HSInstructionArg>,
}

impl HSInstruction {
    pub fn read(&mut self, reader: &mut BufReader<File>) -> io::Result<()> {
        let raw = reader.read_u32::<BE>()?;
        let op_entry = &OP_TABLE[(raw >> 25) as usize];

        self.mode = op_entry.op_code.clone();
        self.read_op_a(raw, op_entry)?;
        self.read_op_bc(raw, op_entry)?;

        Ok(())
    }

    fn read_op_a(&mut self, raw: u32, modes: &HSMode) -> io::Result<()> {
        let mode = if modes.op_mode_a == HSOpArgModeA::UNUSED {
            HSOpArgMode::NUMBER
        } else {
            HSOpArgMode::REG
        };

        let value = raw & 0xFF;
        self.args.push(HSInstructionArg { mode, value });
        Ok(())
    }

    fn read_op_abc_b(&mut self, raw: u32, modes: &HSMode) -> io::Result<()> {
        let (mode, value) = match modes.op_mode_b {
            HSOpArgModeBC::NUMBER => (HSOpArgMode::NUMBER, (raw >> 17) & 0xFF),
            HSOpArgModeBC::OFFSET => (HSOpArgMode::NUMBER, (raw >> 17) & 0x1FF),
            HSOpArgModeBC::REG => (HSOpArgMode::REG, (raw >> 17) & 0xFF),
            HSOpArgModeBC::REGCONST => {
                let value = (raw >> 17) & 0x1FF;
                if value < 0x100 {
                    (HSOpArgMode::REG, value)
                } else {
                    (HSOpArgMode::CONST, value & 0xFF)
                }
            }
            HSOpArgModeBC::CONST => (HSOpArgMode::CONST, (raw >> 17) & 0xFF),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown Arg Mode type!",
                ))
            }
        };
        self.args.push(HSInstructionArg { mode, value });
        Ok(())
    }

    fn read_op_abc_c(&mut self, raw: u32, modes: &HSMode) -> io::Result<()> {
        let (mode, value) = match modes.op_mode_c {
            HSOpArgModeBC::NUMBER => (HSOpArgMode::NUMBER, (raw >> 8) & 0xFF),
            HSOpArgModeBC::OFFSET => (HSOpArgMode::NUMBER, (raw >> 8) & 0x1FF),
            HSOpArgModeBC::REG => (HSOpArgMode::REG, (raw >> 8) & 0xFF),
            HSOpArgModeBC::REGCONST => {
                let value = (raw >> 8) & 0x1FF;
                if value < 0x100 {
                    (HSOpArgMode::REG, value)
                } else {
                    (HSOpArgMode::CONST, value & 0xFF)
                }
            }
            HSOpArgModeBC::CONST => (HSOpArgMode::CONST, (raw >> 8) & 0xFF),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown Arg Mode type!",
                ))
            }
        };
        self.args.push(HSInstructionArg { mode, value });
        Ok(())
    }

    fn read_op_non_abc_b(&mut self, raw: u32, modes: &HSMode) -> io::Result<()> {
        let mut value = (raw >> 8) & 0x1FFFF;
        let mode = match modes.op_mode_b {
            HSOpArgModeBC::NUMBER | HSOpArgModeBC::OFFSET => HSOpArgMode::NUMBER,
            HSOpArgModeBC::CONST => HSOpArgMode::CONST,
            _ => HSOpArgMode::default(),
        };

        if modes.op_mode == HSOpMode::ASBX {
            value = value.wrapping_sub(0xFFFF);
        }
        self.args.push(HSInstructionArg { mode, value });
        Ok(())
    }

    fn read_op_bc(&mut self, raw: u32, modes: &HSMode) -> io::Result<()> {
        if modes.op_mode_b != HSOpArgModeBC::UNUSED {
            if modes.op_mode == HSOpMode::ABC {
                self.read_op_abc_b(raw, modes)?;
            } else {
                self.read_op_non_abc_b(raw, modes)?;
            }
        }

        if modes.op_mode == HSOpMode::ABC && modes.op_mode_c != HSOpArgModeBC::UNUSED {
            self.read_op_abc_c(raw, modes)?;
        }
        Ok(())
    }
}
