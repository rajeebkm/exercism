fn main() {
    println!("{}", hello());
}

pub fn hello() -> &'static str {
    "Hello, world!"
}