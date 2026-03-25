use std::io;

mod restaurant;
use crate::restaurant::logic::{user_input_guest, create_rustaurant, find_table};
use crate::restaurant::model::{Restaurant, RestaurantStatus};

fn main() {
    println!("Welcome to the Rustaurant");

    println!("Open Rustaurant? (1 - yes, 0 - no)");

    let mut choice: String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to understand");

    let choice: u8 = match choice.trim().parse() {
        Ok(ch) => ch,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    if choice == 1 {
        let work_until: u32 = 5; //@todo user input
        let mut restaurant = create_rustaurant();
        rustaurant_start(&mut restaurant, work_until);
    } else {
        println!("Rustaurant is closed");
    }
}

fn rustaurant_start(restaurant: &mut Restaurant, working: u32) {
    println!("Rustaurant is open");

    //println!("Tables {:?}", restaurant.tables);

    // Main work loop
    loop {
        println!("Working {} ticks", restaurant.tick);

        if restaurant.tick >= working {
            restaurant.status = RestaurantStatus::Closed;
            println!("Rustaurant is closed");
            break;
        }

        println!("Somebody come.");
        let count: u8 = user_input_guest();

        // find table
        find_table(restaurant, count);

        restaurant.tick += 1;
    }
}
