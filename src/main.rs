use std::io;

pub mod fibonacci;

fn main() {
    println!("Fibonacci number calculated in 1 second: {}", fibonacci::fibonacci_rebounded());
    println!("Enter any character to exit");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
        }
        Err(error) => println!("error: {error}"),
    }
}
