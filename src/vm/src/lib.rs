#![no_std]

pub mod code_object;
mod opcode;

use crate::code_object::CodeObject;
use crate::opcode::*;


pub struct VM {    
    pub ip: isize,
}

impl VM {
    pub fn run(&mut self, co: CodeObject) -> u8 {
        
        let mut instruction: u8;
        self.ip = 0;

        loop{

            self.ip += 1;

            if self.ip > co.count {
                break 0;
            }
            unsafe {
                instruction = *co.instructions.offset(self.ip-1);
            }           
            
            if instruction == OP_RETURN{
                break 0;
            }
                        

            
        }
    }
}

