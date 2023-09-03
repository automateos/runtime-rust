//#![no_std]

pub mod code_object;
mod opcode;

use crate::code_object::CodeObject;
use crate::opcode::*;

use tracer::console_log;


pub enum ExecResult{
    EXEC_OK,
    EXEC_FAIL,
}

#[repr(C)]
#[derive(Clone,Copy)]
union Value {
    DUINT: u32,
    DINT: i32,
    LUINT: u64,
    LINT: i64,
    REAL: f32,
    LREAL: f64,
}

impl Value {
    pub fn new() -> Self{
        Self{
            LUINT: 0,
        }
    }
}

fn stack_instpect(stack: [Value; 256], top: usize){
    println!("Stack Content************");
    for i in 0..top {
        println!("[{}]:{}", i, unsafe {
            stack[i].DUINT
        });
    }
}

pub struct VM {    
    ip: *const u8,
    instruction_offset: isize,
    code_object: CodeObject,

    value_stack: [Value; 256],
    stack_pointer: usize, //if 0 means stack empty
}

impl VM {

    pub fn new (co: CodeObject) -> Self {

        Self { 
                ip: co.instructions,
                instruction_offset: 0,
                code_object: co,

                value_stack: [Value::new(); 256],
                stack_pointer: 0,
        }
    }

    fn push (&mut self, val: Value) {
        if self.stack_pointer < 255 {
            self.value_stack[self.stack_pointer] = val;
            self.stack_pointer += 1;
        }
        else{
            //stack overflow
        }
        
    }

    fn pop(&mut self) -> Value{
        if self.stack_pointer > 0{
            self.stack_pointer -= 1;
            return self.value_stack[self.stack_pointer]
        }
        else{
            //here must handle stack overflow
            return self.value_stack[self.stack_pointer]
        }
        
    }

    pub fn run(&mut self) -> ExecResult {

        
        ///Macro to read a single byte from instruction array
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

        //macro used to read arguments in format of u16 litle endiand
        macro_rules! read_word {
            () => {
                {
                    let mut byte_array: [u8; 2] = [0, 0];
                    byte_array[0] = read_byte!();
                    byte_array[1] = read_byte!();
                    let word = u16::from_le_bytes(byte_array);
                    word
                }
            };
        }

        macro_rules! read_constants_4bytes {
            ($x: expr) => {
                unsafe{
                    let mut byte_array: [u8; 4] = [0, 0, 0, 0];
                    byte_array[0] = *self.code_object.constants.offset($x);
                    byte_array[1] = *self.code_object.constants.offset($x+1);
                    byte_array[2] = *self.code_object.constants.offset($x+2);
                    byte_array[3] = *self.code_object.constants.offset($x+3);
                    byte_array
                }
            };
        }
        
        let mut instruction: u8;
        
        /// Main evaluation loop
        loop{

           
            instruction = read_byte!();                      
            
            match instruction{
                OP_NOP =>{
                    console_log!("NOP");
                    continue;
                }
                OP_RETURN =>{
                    console_log!("Return");
                    break;
                }
                OP_U32_CONST =>{
                    console_log!("Constant");                     
                    let constant_offset = read_word!() as isize;                    
                    let constant_bytes = read_constants_4bytes!(constant_offset);
                    let constant = Value{ DUINT: u32::from_le_bytes(constant_bytes)} ; 
                    self.push(constant);                    
                    //console_log!("{}", unsafe {constant.DUINT});
                    stack_instpect(self.value_stack, self.stack_pointer);
                }
                _ =>{
                    return ExecResult::EXEC_FAIL;
                }
            }
                        
            if self.instruction_offset >= self.code_object.inststructions_size{
                console_log!("Wrong");
                break;
            }
            
        }
         return ExecResult::EXEC_OK;
    }
}

