use super::{
    hs_constant::HSConstant, hs_debug::HSFunctionDebugInfo, hs_header::HSHeader,
    hs_instruction::HSInstruction,
};
use bitflags::bitflags;
use byteorder::{ReadBytesExt, BE};
use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
};

bitflags! {
    #[derive(Debug, Clone, Default)]
    pub struct HSVarArg : u8 {
        const NONE = 0;
        const HASARG = 1;
        const ISVARARG = 2;
        const NEEDSARG = 4;
    }
}

fn align_to_4_bytes<R: Seek>(reader: &mut R) -> std::io::Result<u64> {
    let current_pos = reader.stream_position()?;
    let aligned_pos = (current_pos + 3) & !3;
    reader.seek(SeekFrom::Start(aligned_pos))
}

#[derive(Debug, Clone, Default)]
pub struct HSFunction {
    pub up_value_count: u32,
    pub param_count: u32,
    pub var_arg: HSVarArg,
    pub slot_count: u32,
    pub unknown: u32,
    pub instruction_count: u32,
    pub instructions: Vec<HSInstruction>,
    pub constant_count: u32,
    pub constants: Vec<HSConstant>,
    pub has_debug_info: bool,
    pub debug_info: HSFunctionDebugInfo,
    pub function_count: u32,
    pub child_functions: Vec<HSFunction>,
    pub function_offset: u64,
}

impl HSFunction {
    pub fn read(&mut self, reader: &mut BufReader<File>, header: &HSHeader) -> std::io::Result<()> {
        self.up_value_count = reader.read_u32::<BE>()?;
        self.param_count = reader.read_u32::<BE>()?;
        self.var_arg = HSVarArg::from_bits_truncate(reader.read_u8()?);
        self.slot_count = reader.read_u32::<BE>()?;
        self.unknown = reader.read_u32::<BE>()?;
        self.instruction_count = reader.read_u32::<BE>()?;

        align_to_4_bytes(reader)?;

        self.instructions = (0..self.instruction_count)
            .map(|_| {
                let mut instruction = HSInstruction::default();
                instruction.read(reader)?;
                Ok(instruction)
            })
            .collect::<std::io::Result<Vec<_>>>()?;

        self.constant_count = reader.read_u32::<BE>()?;
        self.constants = (0..self.constant_count)
            .map(|_| {
                let mut constant = HSConstant::default();
                constant.read(reader, header)?;
                Ok(constant)
            })
            .collect::<std::io::Result<Vec<_>>>()?;

        self.has_debug_info = reader.read_u32::<BE>()? != 0;
        self.debug_info = if self.has_debug_info {
            let mut debug_info = HSFunctionDebugInfo::default();
            debug_info.read(reader, header)?;
            debug_info
        } else {
            HSFunctionDebugInfo::default()
        };

        self.function_count = reader.read_u32::<BE>()?;
        self.child_functions = (0..self.function_count)
            .map(|_| {
                let mut child_function = HSFunction::default();
                child_function.read(reader, header)?;
                Ok(child_function)
            })
            .collect::<std::io::Result<Vec<_>>>()?;

        self.function_offset = reader.stream_position()?;

        Ok(())
    }
}
