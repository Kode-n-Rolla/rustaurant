use std::io::{self, Write};
use crate::restaurant::{model::{GuestGroup, Restaurant}};

pub fn user_start_interaction() -> u8{
    print!("Open Rustaurant? (1 - yes, 0 - no)\n> ");

    io::stdout().flush().unwrap();

    let mut choice: String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to understand");

    let choice: u8 = match choice.trim().parse() {
        Ok(ch) => ch,
        Err(_) => {
            println!("Invalid input");
            return 0;
        }
    };

    choice
}

pub fn is_someone_come() -> bool {
    print!("Is someone come? 1 - yes, 0 - no\n> ");

    io::stdout().flush().unwrap();

    let mut status = String::new();

    io::stdin()
        .read_line(&mut status)
        .expect("Failed input");

    let status: bool = status.trim().parse::<i32>().unwrap_or(0) == 1;

    status
}

pub fn user_input_guest() -> u8 {

    print!("Input count of guest(s)\n> ");

    io::stdout().flush().unwrap();

    let mut count = String::new();

    io::stdin()
        .read_line(&mut count)
        .expect("Failed input");

    let count: u8 = match count.trim().parse() {
        Ok(c) => c,
        Err(_) => {
            return 0; // @todo imlp error handling
        }
    };

    //@todo add check for max 5 guests per group

    count
}

pub fn suggest_waiting(restaurant: &mut Restaurant, count: u8) {
    print!("No free tables. Would you like to wait? (1 - yes, 0 - no)\n> ");

    io::stdout().flush().unwrap();

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Invalid input");

    let choice: u8 = match choice.trim().parse() {
        Ok(ch) => ch,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    if choice == 1 {
        let id: u32 = restaurant.waiting_queue.len() as u32;
        restaurant.waiting_queue.push(
            GuestGroup{id: id + 1, size: count});
            println!("You are the {} in queue", id + 1);
    } else {
        println!("It`s a pity. Have a nice day");
    }
}