use std::io::{self, Write};
use crate::restaurant::{model::{GuestGroup, Restaurant}};

const MAX_GROUP_VALUE: u8 = 5;

pub fn user_start_interaction() -> bool {
    loop {
        print!("Open Rustaurant? (1 - yes, 0 - no)\n> ");
    
        io::stdout().flush().unwrap();
    
        let mut choice_input: String = String::new();
    
        io::stdin()
            .read_line(&mut choice_input)
            .expect("Failed to understand");
    
        match choice_input.trim().parse::<u8>() {
            Ok(status) => {
                if status == 1 {
                    return true;
                } else if status == 0{
                    return false;
                } else {
                    println!("Please enter a valid choice.");
                    continue;
                }
            },
            Err(_) => {
                println!("Please enter a valid choice.");
                continue;
            }
        }
    }
}

pub fn is_someone_come() -> bool {
    loop {
        print!("Has anyone arrived? 1 - yes, 0 - no\n> ");
    
        io::stdout().flush().unwrap();
    
        let mut status_input = String::new();
    
        io::stdin()
            .read_line(&mut status_input)
            .expect("Failed input");
    
        match status_input.trim().parse::<u8>() {
            Ok(status) => {
                if status == 1 {
                    return true;
                } else if status == 0 {
                    return false;
                } else {
                    println!("Please enter a valid choice.");
                    continue;
                }
            },
            Err(_) => {
                println!("Please enter a valid choice.");
                continue;
            }
        }
    }
}

pub fn user_input_guest() -> u8 {
    loop {
        print!("Enter the number of guest(s)\n> ");
    
        io::stdout().flush().unwrap();
    
        let mut count_input = String::new();
    
        io::stdin()
            .read_line(&mut count_input)
            .expect("Failed input");
    
        match count_input.trim().parse() {
            Ok(count) => {
                if count == 0 {
                    println!("The group size must be at least 1.");
                    continue;
                } else if count > MAX_GROUP_VALUE {
                    println!("The maximum group size is {}", MAX_GROUP_VALUE);
                    continue;
                } else {
                    return count;
                }
            },
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
    }
}

pub fn suggest_waiting(restaurant: &mut Restaurant, count: u8) {
    println!("No free tables.");

    loop {
        print!("Would you like to wait? (1 - yes, 0 - no)\n> ");

        io::stdout().flush().unwrap();
    
        let mut choice_input = String::new();
    
        io::stdin()
            .read_line(&mut choice_input)
            .expect("Invalid input");

        match choice_input.trim().parse::<u8>() {
            Ok(choice) => {
                if choice == 1 {
                    let id: u32 = restaurant.waiting_queue.len() as u32;
                    restaurant.waiting_queue.push(
                        GuestGroup{id: id + 1, size: count});
                        println!("You are number {} in the queue", id + 1);
                        break;
                } else if choice == 0 {
                    println!("We are sorry to hear that. Have a nice day!");
                    break;
                } else {
                    println!("Please enter a valid choice");
                    continue;
                }
            },
            Err(_) => {
                println!("Please enter a valid choice");
                continue;
            }
        }
    }
}