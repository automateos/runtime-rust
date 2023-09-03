//! # Code Object
//!
//! This crate contain the definition for the code object struct.
//! and his methods.
//! Code Object are the executable form that interpreter execute.

/// Representation of executable program
pub struct CodeObject {

    /// it is a pointer to an array of byte with the op codes.
    pub instructions: *const u8,
    /// instruction array's length 
    pub inststructions_size : isize,
    
    /// pointer to a byte array with constant values
    pub constants: *const u8,
    //constatn array's length in byte
    pub constants_size : isize,
    
}

