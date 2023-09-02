#![no_std]

pub mod code_object;
mod opcode;

use crate::code_object::CodeObject;
use crate::opcode::*;

pub enum ExecResult{
    EXEC_OK,
    EXEC_FAIL,
}


pub struct VM {    
    ip: *const u8,
    instruction_offset: isize,
}

impl VM {

    pub fn new (co: CodeObject) -> Self {

        Self { 
                ip: co.instructions,
                instruction_offset: 0,
        }
    }

    pub fn run(&mut self) -> ExecResult {

        

        macro_rules! read_byte {
            () => {
                unsafe {
                    let byte_content: u8;
                    byte_content = *self.ip.offset(self.instruction_offset);
                    self.instruction_offset+= 1;
                    byte_content
                }                
            };
        }
        
        let mut instruction: u8;
        

        loop{

           
            instruction = read_byte!();                      
            
            match instruction{
                OP_NOP =>{
                    continue;
                }
                OP_RETURN =>{
                    break;
                }
                _ =>{
                    break;
                }
            }
                        

            
        }
         return ExecResult::EXEC_OK;
    }
}

