//! # Code Object
//!
//! This crate contain the definition for the code object struct.
//! and his methods.
//! Code Object are the executable form that interpreter execute.

/// Representation of executable program
pub struct CodeObject {

    /// it is a pointer to an array of byte with the op codes.
    pub instructions: *const u8,

    /// number of array's opcode 
    pub count : isize,
    
    //constants: Vec<u8>,
    
}

pub fn buil_co(code:*const u8, op_count : isize) -> CodeObject {
    CodeObject { 
        instructions: code,
        count: op_count,
        //constants: constants,
    }
}