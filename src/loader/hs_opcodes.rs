use num_enum::TryFromPrimitive;

/// Small helper macro to create a `HSMode` struct.
macro_rules! hs_mode {
    ($op:ident, $mode:ident, $a:ident, $b:ident, $c:ident) => {
        HSMode {
            op_code: HSOpCode::$op,
            op_mode: HSOpMode::$mode,
            op_mode_a: HSOpArgModeA::$a,
            op_mode_b: HSOpArgModeBC::$b,
            op_mode_c: HSOpArgModeBC::$c,
        }
    };
}

/// Enum representing `HavokScript` operation codes
#[derive(Debug, TryFromPrimitive, Clone, Default)]
#[repr(u8)]
pub enum HSOpCode {
    #[default]
    GetField,
    Test,
    CallI,
    CallC,
    Eq,
    EqBk,
    GetGlobal,
    Move,
    SelfOp,
    Return,
    GetTableS,
    GetTableN,
    GetTable,
    LoadBool,
    TForLoop,
    SetField,
    SetTableS,
    SetTableSBk,
    SetTableN,
    SetTableNBk,
    SetTable,
    SetTableBk,
    TailCallI,
    TailCallC,
    TailCallM,
    LoadK,
    LoadNil,
    SetGlobal,
    Jmp,
    CallM,
    Call,
    IntrinsicIndex,
    IntrinsicNewIndex,
    IntrinsicSelf,
    IntrinsicLiteral,
    IntrinsicNewIndexLiteral,
    IntrinsicSelfLiteral,
    TailCall,
    GetUpval,
    SetUpval,
    Add,
    AddBk,
    Sub,
    SubBk,
    Mul,
    MulBk,
    Div,
    DivBk,
    Mod,
    ModBk,
    Pow,
    PowBk,
    NewTable,
    Unm,
    Not,
    Len,
    Lt,
    LtBk,
    Le,
    LeBk,
    Concat,
    TestSet,
    ForPrep,
    ForLoop,
    SetList,
    Close,
    Closure,
    Vararg,
    TailCallIR1,
    CallIR1,
    SetUpvalR1,
    TestR1,
    NotR1,
    GetFieldR1,
    SetFieldR1,
    NewStruct,
    Data,
    SetSlotN,
    SetSlotI,
    SetSlot,
    SetSlotS,
    SetSlotMt,
    CheckType,
    CheckTypes,
    GetSlot,
    GetSlotMt,
    SelfSlot,
    SelfSlotMt,
    GetFieldMm,
    CheckTypeD,
    GetSlotD,
    GetGlobalMem,
    NumOpcodes,
}

impl std::fmt::Display for HSOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Enum representing `HavokScript` data types
#[derive(Debug, TryFromPrimitive, Clone, Default)]
#[repr(u8)]
pub enum HSType {
    #[default]
    TNIL,
    TBOOLEAN,
    TLIGHTUSERDATA,
    TNUMBER,
    TSTRING,
    TTABLE,
    TFUNCTION,
    TUSERDATA,
    TTHREAD,
    TIFUNCTION,
    TCFUNCTION,
    TUI64,
    TSTRUCT,
}

/// Enum representing argument modes for `HavokScript` operations
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum HSOpArgMode {
    #[default]
    NUMBER,
    REG,
    CONST,
}

impl std::fmt::Display for HSOpArgMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Enum representing argument modes for the A field in `HavokScript` operations
#[derive(Debug, PartialEq, Eq)]
pub enum HSOpArgModeA {
    UNUSED,
    REG,
}

/// Enum representing operation modes in `HavokScript`
#[derive(Debug, PartialEq, Eq)]
pub enum HSOpMode {
    ABC,  // Operation with three fields: A, B, and C
    ABX,  // Operation with two fields: A and BX
    ASBX, // Operation with two fields: A and signed BX
}

/// Enum representing argument modes for B and C fields in `HavokScript` operations
#[derive(Debug, PartialEq, Eq)]
pub enum HSOpArgModeBC {
    UNUSED,   // Unused argument
    NUMBER,   // Argument is a number
    OFFSET,   // Argument is an offset
    REG,      // Argument is a register
    REGCONST, // Argument is either a register or constant
    CONST,    // Argument is a constant
}

/// Struct representing a complete `HavokScript` operation mode
pub struct HSMode {
    pub op_code: HSOpCode,
    pub op_mode: HSOpMode,
    pub op_mode_a: HSOpArgModeA,
    pub op_mode_b: HSOpArgModeBC,
    pub op_mode_c: HSOpArgModeBC,
}

pub const OP_TABLE: [HSMode; 92] = [
    hs_mode!(GetField, ABC, REG, REG, CONST),
    hs_mode!(Test, ABC, REG, UNUSED, NUMBER),
    hs_mode!(CallI, ABC, REG, NUMBER, NUMBER),
    hs_mode!(CallC, ABC, REG, NUMBER, NUMBER),
    hs_mode!(Eq, ABC, UNUSED, REGCONST, REGCONST),
    hs_mode!(EqBk, ABC, UNUSED, REGCONST, REGCONST),
    hs_mode!(GetGlobal, ABX, REG, CONST, NUMBER),
    hs_mode!(Move, ABC, REG, REG, UNUSED),
    hs_mode!(SelfOp, ABC, REG, REG, REGCONST),
    hs_mode!(Return, ABC, REG, NUMBER, UNUSED),
    hs_mode!(GetTableS, ABC, REG, REG, REGCONST),
    hs_mode!(GetTableN, ABC, REG, REG, REGCONST),
    hs_mode!(GetTable, ABC, REG, REG, REGCONST),
    hs_mode!(LoadBool, ABC, REG, NUMBER, NUMBER),
    hs_mode!(TForLoop, ABC, REG, UNUSED, NUMBER),
    hs_mode!(SetField, ABC, REG, CONST, REGCONST),
    hs_mode!(SetTableS, ABC, REG, REGCONST, REGCONST),
    hs_mode!(SetTableSBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(SetTableN, ABC, REG, REGCONST, REGCONST),
    hs_mode!(SetTableNBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(SetTable, ABC, REG, REGCONST, REGCONST),
    hs_mode!(SetTableBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(TailCallI, ABC, REG, NUMBER, NUMBER),
    hs_mode!(TailCallC, ABC, REG, NUMBER, NUMBER),
    hs_mode!(TailCallM, ABC, REG, NUMBER, NUMBER),
    hs_mode!(LoadK, ABX, REG, CONST, UNUSED),
    hs_mode!(LoadNil, ABC, REG, REG, UNUSED),
    hs_mode!(SetGlobal, ABX, REG, CONST, UNUSED),
    hs_mode!(Jmp, ASBX, UNUSED, OFFSET, UNUSED),
    hs_mode!(CallM, ABC, REG, NUMBER, NUMBER),
    hs_mode!(Call, ABC, REG, NUMBER, NUMBER),
    hs_mode!(IntrinsicIndex, ABC, REG, NUMBER, NUMBER),
    hs_mode!(IntrinsicNewIndex, ABC, REG, NUMBER, NUMBER),
    hs_mode!(IntrinsicSelf, ABC, REG, NUMBER, NUMBER),
    hs_mode!(IntrinsicLiteral, ABC, REG, NUMBER, NUMBER),
    hs_mode!(IntrinsicNewIndexLiteral, ABC, REG, NUMBER, NUMBER),
    hs_mode!(IntrinsicSelfLiteral, ABC, REG, NUMBER, NUMBER),
    hs_mode!(TailCall, ABC, REG, NUMBER, NUMBER),
    hs_mode!(GetUpval, ABC, REG, NUMBER, UNUSED),
    hs_mode!(SetUpval, ABC, REG, NUMBER, UNUSED),
    hs_mode!(Add, ABC, REG, REGCONST, REGCONST),
    hs_mode!(AddBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(Sub, ABC, REG, REGCONST, REGCONST),
    hs_mode!(SubBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(Mul, ABC, REG, REGCONST, REGCONST),
    hs_mode!(MulBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(Div, ABC, REG, REGCONST, REGCONST),
    hs_mode!(DivBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(Mod, ABC, REG, REGCONST, REGCONST),
    hs_mode!(ModBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(Pow, ABC, REG, REGCONST, REGCONST),
    hs_mode!(PowBk, ABC, REG, REGCONST, REGCONST),
    hs_mode!(NewTable, ABC, REG, NUMBER, NUMBER),
    hs_mode!(Unm, ABC, REG, REG, UNUSED),
    hs_mode!(Not, ABC, REG, REG, UNUSED),
    hs_mode!(Len, ABC, REG, REG, UNUSED),
    hs_mode!(Lt, ABC, UNUSED, REGCONST, REGCONST),
    hs_mode!(LtBk, ABC, UNUSED, REGCONST, REGCONST),
    hs_mode!(Le, ABC, UNUSED, REGCONST, REGCONST),
    hs_mode!(LeBk, ABC, UNUSED, REGCONST, REGCONST),
    hs_mode!(Concat, ABC, REG, NUMBER, NUMBER),
    hs_mode!(TestSet, ABC, REG, REG, NUMBER),
    hs_mode!(ForPrep, ASBX, REG, OFFSET, UNUSED),
    hs_mode!(ForLoop, ASBX, REG, OFFSET, UNUSED),
    hs_mode!(SetList, ABC, REG, NUMBER, OFFSET),
    hs_mode!(Close, ABC, REG, UNUSED, UNUSED),
    hs_mode!(Closure, ABX, REG, NUMBER, UNUSED),
    hs_mode!(Vararg, ABC, REG, NUMBER, UNUSED),
    hs_mode!(TailCallIR1, ABC, UNUSED, NUMBER, NUMBER),
    hs_mode!(CallIR1, ABC, UNUSED, NUMBER, NUMBER),
    hs_mode!(SetUpvalR1, ABC, REG, NUMBER, UNUSED),
    hs_mode!(TestR1, ABC, REG, UNUSED, NUMBER),
    hs_mode!(NotR1, ABC, REG, REG, UNUSED),
    hs_mode!(GetFieldR1, ABC, REG, REG, CONST),
    hs_mode!(SetFieldR1, ABC, REG, CONST, REGCONST),
    hs_mode!(NewStruct, ABC, REG, NUMBER, NUMBER),
    hs_mode!(Data, ABX, UNUSED, OFFSET, UNUSED),
    hs_mode!(SetSlotN, ABC, REG, UNUSED, NUMBER),
    hs_mode!(SetSlotI, ABC, REG, NUMBER, REGCONST),
    hs_mode!(SetSlot, ABC, REG, NUMBER, REGCONST),
    hs_mode!(SetSlotS, ABC, REG, NUMBER, REG),
    hs_mode!(SetSlotMt, ABC, REG, NUMBER, REGCONST),
    hs_mode!(CheckType, ABX, REG, NUMBER, UNUSED),
    hs_mode!(CheckTypes, ABX, REG, NUMBER, UNUSED),
    hs_mode!(GetSlot, ABC, REG, REG, NUMBER),
    hs_mode!(GetSlotMt, ABC, REG, REG, NUMBER),
    hs_mode!(SelfSlot, ABC, REG, REG, NUMBER),
    hs_mode!(SelfSlotMt, ABC, REG, REG, NUMBER),
    hs_mode!(GetFieldMm, ABC, REG, REG, CONST),
    hs_mode!(CheckTypeD, ABX, REG, NUMBER, UNUSED),
    hs_mode!(GetSlotD, ABC, REG, REG, NUMBER),
    hs_mode!(GetGlobalMem, ABX, REG, CONST, NUMBER),
];
