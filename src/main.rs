#[derive(Default)]
struct Adder;

impl Adder {
    pub fn new() -> Self {
        Self
    }
}

fn main() {
    let _a = Adder::new();
    println!("Hello, world!");
}
