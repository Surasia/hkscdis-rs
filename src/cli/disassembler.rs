use crate::loader::{
    hs::HavokScriptFile, hs_debug::HSFunctionDebugInfo, hs_enums::HSEnum, hs_function::HSFunction,
    hs_header::HSHeader, hs_opcodes::HSOpArgMode, hs_structure::HSStructBlock,
};

use color_print::{cprint, cprintln};

fn print_header(header: &HSHeader) {
    cprintln!(
        "<yellow>- Is Little Endian:<yellow> <bright-cyan>{}<bright-cyan>",
        header.is_little_endian
    );
    cprintln!(
        "<yellow>- Integer Size:<yellow> <bright-cyan>{}<bright-cyan>",
        header.int_size
    );
    cprintln!(
        "<yellow>- Type Size:<yellow> <bright-cyan>{}<bright-cyan>",
        header.t_size
    );
    cprintln!(
        "<yellow>- Instruction Size:<yellow> <bright-cyan>{}<bright-cyan>",
        header.instruction_size
    );
    cprintln!(
        "<yellow>- Number Size:<yellow> <bright-cyan>{}<bright-cyan>",
        header.number_size
    );
    cprintln!(
        "<yellow>- Is Using Integer:<yellow> <bright-cyan>{}<bright-cyan>",
        header.is_integer
    );
    cprintln!(
        "<yellow>- Extensions:<yellow> <bright-cyan>{}<bright-cyan>",
        header.compatability
    );
    println!();
}

fn print_enums(enums: &Vec<HSEnum>) {
    for item in enums {
        cprintln!(
            "<yellow>- {}:<yellow> <bright-cyan>{}<bright-cyan>",
            item.name,
            item.value
        );
    }
    println!();
}

#[allow(clippy::cast_sign_loss)]
fn print_instruction(function: &HSFunction) {
    for i in &function.instructions {
        cprint!("<yellow>   - {:?}: <yellow>", i.mode);
        for arg in &i.args {
            if arg.mode == HSOpArgMode::CONST {
                let val = &function.constants[arg.value as usize];
                cprint!("<bright-cyan>CONST(<bright-cyan><bright-blue>{}<bright-blue><bright-cyan>)<bright-cyan> ", val);
            } else {
                cprint!(
                    "<bright-cyan>{:?}(<bright-cyan><bright-blue>{}<bright-blue><bright-cyan>) <bright-cyan>",
                    arg.mode,
                    arg.value
                );
            }
        }
        println!();
    }
}

fn print_constants(function: &HSFunction) {
    for c in &function.constants {
        cprintln!(
            "<yellow>   - {:?}<yellow> <bright-cyan>{}<bright-cyan>",
            c.type_,
            c,
        );
    }
}

fn print_debug(debug_info: &HSFunctionDebugInfo) {
    cprintln!(
        "<yellow>   - Line Count: <yellow><bright-cyan>{}<bright-cyan>",
        debug_info.line_count
    );
    cprintln!(
        "<yellow>   - Locals Count: <yellow><bright-cyan>{}<bright-cyan>",
        debug_info.locals_count
    );
    cprintln!(
        "<yellow>   - Up Value Count: <yellow><bright-cyan>{}<bright-cyan>",
        debug_info.up_value_count
    );
    cprintln!(
        "<yellow>   - Line Begin: <yellow><bright-cyan>{}<bright-cyan>",
        debug_info.line_begin
    );
    cprintln!(
        "<yellow>   - Line End: <yellow><bright-cyan>{}<bright-cyan>",
        debug_info.line_end
    );
    cprintln!(
        "<yellow>   - Path: <yellow><bright-cyan>{}<bright-cyan>",
        debug_info.path
    );
    cprintln!(
        "<yellow>   - Function Name: <yellow><bright-cyan>{}<bright-cyan>",
        debug_info.function_name
    );

    if debug_info.locals_count != 0 {
        cprintln!("<yellow>   - Locals: <yellow>");
        for local in &debug_info.locals {
            cprintln!(
                "<yellow>      - <yellow><bright-blue>{}<bright-blue>",
                local.local_name
            );
        }
    }

    if debug_info.up_value_count != 0 {
        cprintln!("<yellow>   - Up Values: <yellow>");
        for upvalue in &debug_info.up_values {
            cprintln!(
                "<yellow>      - <yellow><bright-blue>{}<bright-blue>",
                upvalue
            );
        }
    }
}

fn print_function(function: &HSFunction) {
    if function.has_debug_info && !function.debug_info.function_name.is_empty() {
        cprintln!(
            "<bright-blue>- Function: {}<bright-blue>",
            function.debug_info.function_name
        );
    } else {
        cprintln!(
            "<yellow>- Function:<yellow> <bright-cyan>0x{:X}<bright-cyan>:",
            function.function_offset
        );
    }
    cprintln!(
        "<yellow>   - UpValue Count: <bright-cyan>{}<bright-cyan>",
        function.up_value_count
    );
    cprintln!(
        "<yellow>   - Parameter Count: <bright-cyan>{}<bright-cyan>",
        function.param_count
    );
    cprintln!(
        "<yellow>   - Variadic Argument Type: <bright-cyan>{}<bright-cyan>",
        function.var_arg
    );
    cprintln!(
        "<yellow>   - Slot Count: <bright-cyan>{}<bright-cyan>",
        function.slot_count
    );

    if function.instruction_count != 0 {
        cprintln!("<bright-blue>  Instructions:<bright-blue>");
        print_instruction(function);
    }

    if function.constant_count != 0 {
        cprintln!("<bright-blue>  Constants:<bright-blue>");
        print_constants(function);
    }

    if function.has_debug_info {
        cprintln!("<bright-blue>  Debug Info:<bright-blue>");
        print_debug(&function.debug_info);
    }

    println!();
}

fn print_structures(structs: &Vec<HSStructBlock>) {
    for struc in structs {
        cprint!(
            "<yellow>-<yellow> <bright-blue>{}<bright-blue>",
            struc.header.name
        );
        if struc.extended_structs.is_empty() {
            cprintln!("<bright-blue>:<bright-blue>");
        }
        for extend in &struc.extended_structs {
            cprintln!(
                "<green> EXTENDS<green> <bright-blue>{}<bright-blue>:",
                extend
            );
        }
        for member in &struc.members {
            cprintln!(
                "<yellow>   - {:?}<yellow> <bright-cyan>{}<bright-cyan>",
                member.header.type_,
                member.header.name
            );
        }
    }
}

pub fn print_disassembly(file: &HavokScriptFile) {
    cprintln!("<green>{}<green>", "[Header]");
    print_header(&file.header);
    cprintln!("<green>{}<green>", "[Enums]");
    print_enums(&file.enums);
    cprintln!("<green>{}<green>", "[Functions]");
    print_function(&file.main_function);
    for function in &file.main_function.child_functions {
        print_function(function);
    }
    if !file.structs.is_empty() {
        cprintln!("<green>{}<green>", "[Structures]");
    }
    print_structures(&file.structs);
}
