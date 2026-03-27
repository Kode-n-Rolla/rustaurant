use std::io;

mod restaurant;
use crate::restaurant::logic::{user_input_guest, create_rustaurant, find_table, tick};
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

        // @todo tick impl in tick() for actions in tick, like decrease tables` `remaining_ticks`
        tick(restaurant);

        if restaurant.tick >= working {
            restaurant.status = RestaurantStatus::Closed;
            println!("Rustaurant is closed");
            break;
        }

        println!("Somebody come."); // @todo but first ask come somebody or not
        let count: u8 = user_input_guest();

        // find table
        let table_id = find_table(restaurant, count);
        if table_id != 0 {
            println!("Guest sit at {} table", table_id);
        } else {
            println!("Suggest waiting"); // @todo give a choice waiting or leave
        }


        restaurant.tick += 1;
        println!("Current status: {:#?}", restaurant);
    }
}
