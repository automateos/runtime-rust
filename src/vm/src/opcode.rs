//! # Op Code
//!
//! This crate contain the definition for all op codes.
//! Are strongly inspired on webassembly instructions


//Control Instruction
pub const OP_NOP: u8 = 0x00;        //No operation
pub const OP_RETURN: u8 = 0x01;        //Return
pub const OP_CALL: u8 = 0x02;          // call a function
pub const OP_JMP: u8 = 0x03;           //Unconditional jump
pub const OP_CJMP: u8 = 0x04;          //Condtional Jump

//Memory Instruction
pub const OP_U32_LOAD: u8 = 40;     //Load a u32 to value stack.
pub const OP_I32_LOAD: u8 = 40;     //Load a i32 to value stack.
pub const OP_U64_LOAD: u8 = 40;     //Load a u64 to value stack.
pub const OP_I64_LOAD: u8 = 40;     //Load a i64 to value stack.
pub const OP_F32_LOAD: u8 = 40;     //Load a f32 to value stack.
pub const OP_F64_LOAD: u8 = 40;     //Load a f64 to value stack.
pub const OP_U32_STORE: u8 = 40;    //Store a u32 from value stack to memory.
pub const OP_I32_STORE: u8 = 40;    //Store a i32 from value stack to memory.
pub const OP_U64_STORE: u8 = 40;    //Store a u64 from value stack to memory.
pub const OP_I64_STORE: u8 = 40;    //Store a i64 from value stack to memory.
pub const OP_F32_STORE: u8 = 40;    //Store a f32 from value stack to memory.
pub const OP_F64_STORE: u8 = 40;    //Store a f64 from value stack to memory.


//Numeric Instructions
pub const OP_U32_CONST: u8 = 40;    //Load u32 constant to value stack
pub const OP_I32_CONST: u8 = 40;    //Load i32 constant to value stack
pub const OP_U64_CONST: u8 = 40;    //Load u64 constant to value stack
pub const OP_I64_CONST: u8 = 40;    //Load i64 constant to value stack
pub const OP_F32_CONST: u8 = 40;    //Load f32 constant to value stack
pub const OP_F64_CONST: u8 = 40;    //Load f64 constant to value stack

pub const OP_U32_EQ: u8 = 45;       //Binary operation. compare two u32 and return true if equal
pub const OP_U32_NE: u8 = 45;       //Binary operation. compare two u32 and return true if not equal
pub const OP_U32_LT: u8 = 45;       //Binary operation. compare two u32 and return true if is less than
pub const OP_U32_GT: u8 = 45;       //Binary operation. compare two u32 and return true if is Greater than
pub const OP_U32_LE: u8 = 45;       //Binary operation. compare two u32 and return true if  less or equal
pub const OP_U32_GE: u8 = 45;       //Binary operation. compare two u32 and return true if greater equal

pub const OP_I32_EQ: u8 = 46;       //Binary operation. compare two i32 and return true if equal
pub const OP_I32_NE: u8 = 45;       //Binary operation. compare two i32 and return true if not equal
pub const OP_I32_LT: u8 = 45;       //Binary operation. compare two i32 and return true if is less than
pub const OP_I32_GT: u8 = 45;       //Binary operation. compare two i32 and return true if is Greater than
pub const OP_I32_LE: u8 = 45;       //Binary operation. compare two i32 and return true if  less or equal
pub const OP_I32_GE: u8 = 45;       //Binary operation. compare two i32 and return true if greater equal

pub const OP_U64_EQ: u8 = 47;       //Binary operation. compare two u64 and return true if equal
pub const OP_U64_NE: u8 = 45;       //Binary operation. compare two u64 and return true if not equal
pub const OP_U64_LT: u8 = 45;       //Binary operation. compare two u64 and return true if is less than
pub const OP_U64_GT: u8 = 45;       //Binary operation. compare two u64 and return true if is Greater than
pub const OP_U64_LE: u8 = 45;       //Binary operation. compare two u64 and return true if  less or equal
pub const OP_U64_GE: u8 = 45;       //Binary operation. compare two u64 and return true if greater equal

pub const OP_I64_EQ: u8 = 48;       //Binary operation. compare two i64 and return true if equal
pub const OP_I64_NE: u8 = 45;       //Binary operation. compare two i64 and return true if not equal
pub const OP_I64_LT: u8 = 45;       //Binary operation. compare two i64 and return true if is less than
pub const OP_I64_GT: u8 = 45;       //Binary operation. compare two i64 and return true if is Greater than
pub const OP_I64_LE: u8 = 45;       //Binary operation. compare two i64 and return true if  less or equal
pub const OP_I64_GE: u8 = 45;       //Binary operation. compare two i64 and return true if greater equal

pub const OP_F32_EQ: u8 = 49;       //Binary operation. compare two f32 and return true if equal
pub const OP_F32_NE: u8 = 45;       //Binary operation. compare two f32 and return true if not equal
pub const OP_F32_LT: u8 = 45;       //Binary operation. compare two f32 and return true if is less than
pub const OP_F32_GT: u8 = 45;       //Binary operation. compare two f32 and return true if is Greater than
pub const OP_F32_LE: u8 = 45;       //Binary operation. compare two f32 and return true if  less or equal
pub const OP_F32_GE: u8 = 45;       //Binary operation. compare two f32 and return true if greater equal

pub const OP_F64_EQ: u8 = 50;       //Binary operation. compare two f64 and return true if equal
pub const OP_F64_NE: u8 = 45;       //Binary operation. compare two f64 and return true if not equal
pub const OP_F64_LT: u8 = 45;       //Binary operation. compare two f64 and return true if is less than
pub const OP_F64_GT: u8 = 45;       //Binary operation. compare two f64 and return true if is Greater than
pub const OP_F64_LE: u8 = 45;       //Binary operation. compare two f64 and return true if  less or equal
pub const OP_F64_GE: u8 = 45;       //Binary operation. compare two f64 and return true if greater equal

pub const OP_U32_ADD: u8 = 45;      //Binary operation add two u32
pub const OP_U32_SUB: u8 = 45;      //Binary operation substraction two u32
pub const OP_U32_MUL: u8 = 45;      //Binary operation multiplication two u32
pub const OP_U32_DIV: u8 = 45;      //Binary operation division two u32
pub const OP_U32_REM: u8 = 45;      //Binary operation return the remainder of division
pub const OP_U32_NOT: u8 = 45;      //Unanary operation logical NOT
pub const OP_U32_AND: u8 = 45;      //Binary operation logical AND
pub const OP_U32_OR: u8 = 45;       //Binary operation logical OR
pub const OP_U32_XOR: u8 = 45;      //Binary operation logical XOR

pub const OP_I32_ADD: u8 = 45;      //Binary operation add two i32
pub const OP_I32_SUB: u8 = 45;      //Binary operation substraction two i32
pub const OP_I32_MUL: u8 = 45;      //Binary operation multiplication two i32
pub const OP_I32_DIV: u8 = 45;      //Binary operation division two i32
pub const OP_I32_REM: u8 = 45;      //Binary operation return the remainder of division
pub const OP_I32_NOT: u8 = 45;      //Unanary operation logical NOT
pub const OP_I32_AND: u8 = 45;      //Binary operation logical AND
pub const OP_I32_OR: u8 = 45;       //Binary operation logical OR
pub const OP_I32_XOR: u8 = 45;      //Binary operation logical XOR


pub const OP_U64_ADD: u8 = 45;      //Binary operation add two u64
pub const OP_U64_SUB: u8 = 45;      //Binary operation substraction two u64
pub const OP_U64_MUL: u8 = 45;      //Binary operation multiplication two u64
pub const OP_U64_DIV: u8 = 45;      //Binary operation division two u64
pub const OP_U64_REM: u8 = 45;      //Binary operation return the remainder of division
pub const OP_U64_NOT: u8 = 45;      //Unanary operation logical NOT
pub const OP_U64_AND: u8 = 45;      //Binary operation logical AND
pub const OP_U64_OR: u8 = 45;       //Binary operation logical OR
pub const OP_U64_XOR: u8 = 45;      //Binary operation logical XOR


pub const OP_I64_ADD: u8 = 45;      //Binary operation add two i64
pub const OP_I64_SUB: u8 = 45;      //Binary operation substraction two i64
pub const OP_I64_MUL: u8 = 45;      //Binary operation multiplication two i64
pub const OP_I64_DIV: u8 = 45;      //Binary operation division two i64
pub const OP_I64_REM: u8 = 45;      //Binary operation return the remainder of division
pub const OP_I64_NOT: u8 = 45;      //Unanary operation logical NOT
pub const OP_I64_AND: u8 = 45;      //Binary operation logical AND
pub const OP_I64_OR: u8 = 45;       //Binary operation logical OR
pub const OP_I64_XOR: u8 = 45;      //Binary operation logical XOR

pub const OP_F32_ADD: u8 = 45;      //Binary operation. Add to float
pub const OP_F32_SUB: u8 = 45;      //Binary operation. Substract to float
pub const OP_F32_MUL: u8 = 45;      //Binary operation. Mul to float
pub const OP_F32_DIV: u8 = 45;      //Binary operation. DIV to float
pub const OP_F32_ABS: u8 = 45;      //Unary operation. Absolute value ej -1.2 -> 1.2
pub const OP_F32_NEG: u8 = 45;      //Unary operation. change sign.
pub const OP_F32_CEIL: u8 = 45;     //Unary operation. Round to higest natural value. Ex 1.3 -> 2.0
pub const OP_F32_FlOOR: u8 = 45;    //Unary operation. Round to lowest natural value. Ex 1.7 -> 1.0

pub const OP_F64_ADD: u8 = 45;      //Binary operation. Add to float
pub const OP_F64_SUB: u8 = 45;      //Binary operation. Substract to float
pub const OP_F64_MUL: u8 = 45;      //Binary operation. Mul to float
pub const OP_F64_DIV: u8 = 45;      //Binary operation. DIV to float
pub const OP_F64_ABS: u8 = 45;      //Unary operation. Absolute value ej -1.2 -> 1.2
pub const OP_F64_NEG: u8 = 45;      //Unary operation. change sign.
pub const OP_F64_CEIL: u8 = 45;     //Unary operation. Round to higest natural value. Ex 1.3 -> 2.0
pub const OP_F64_FlOOR: u8 = 45;    //Unary operation. Round to lowest natural value. Ex 1.7 -> 1.0

//bit displasement

pub const OP_DW_SHL: u8 = 45;      //Binary operation shift bits to left
pub const OP_DW_SHR: u8 = 45;      //Binary operation shift bits to right
pub const OP_DW_ROTR:u8 = 45;      //Binary operation rotation of bit to right.
pub const OP_DW_ROTL:u8 = 45;      //Binary operation rotation of bit to left.
pub const OP_LW_SHL: u8 = 45;      //Binary operation shift bits to left
pub const OP_LW_SHR: u8 = 45;      //Binary operation shift bits to right
pub const OP_LW_ROTR:u8 = 45;      //Binary operation rotation of bit to right.
pub const OP_LW_ROTL:u8 = 45;      //Binary operation rotation of bit to left.
