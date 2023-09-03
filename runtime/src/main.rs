
use vm::{VM, code_object::CodeObject};

fn main() {
        
    let code: [u8; 5] = [0x00, 0x40, 0x00, 0x00, 0x01];
    let mut constants: [u8; 4] = [0, 0, 0, 0];
    constants[0]=0x88;

    let test_code = CodeObject{
        instructions: code.as_ptr(),
        inststructions_size: 5,
        constants: constants.as_ptr(),
        constants_size: 4
    };
    
    let mut vm1 = VM::new(test_code);

    vm1.run();
    println!("Hello, Automate OS");
}
