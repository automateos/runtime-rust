use vm;
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", vm::add_one(num));
}
