use vm;
use vm::VM;

fn main() {
        
    let code: [u8; 1] = [0];
    //let constants: Vec<u8> = vec![1, 2, 3];
    
    let test_code = vm::code_object::buil_co(code.as_ptr(), 1);
    let mut vm1 = VM{
        ip: 0,
    };
    vm1.run(test_code);
    println!("Hello, Automate OS");
}
