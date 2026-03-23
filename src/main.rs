use std::io;
use restaurant::model;

fn main() {
    println!("Welcome to the Rustaurant");

    println!("Open Rustaurant? (1 - yes, 0 - no)");

    let mut choice: String = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to understand");

    let choice: u8 = match choice.trim().parse() {
        Ok(ch) => ch,
        Err(_) => 255, //@todo imlp correct out value
    };

    match choice {
        1 => println!("Rustaurant is open"),
        _ => println!("Rustaurant is closed")
    }
}
