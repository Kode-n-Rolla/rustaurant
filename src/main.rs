mod restaurant;
use crate::restaurant::ui::{
    is_someone_come,
    suggest_waiting,
    user_input_guest,
    user_start_interaction};
use crate::restaurant::logic::{
    create_rustaurant,
    find_table,
    tick,
};
use crate::restaurant::model::{Restaurant, RestaurantStatus};

fn main() {
    println!("Welcome to the Rustaurant");

    let choice = user_start_interaction();

    if choice == 1 {
        const WORK_UNTIL: u32 = 10; //@todo (?) user input
        let mut restaurant = create_rustaurant();
        rustaurant_start(&mut restaurant, WORK_UNTIL);
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

        tick(restaurant);

        if restaurant.tick >= working {
            restaurant.status = RestaurantStatus::Closed;
            println!("Rustaurant is closed");
            break;
        }

        match is_someone_come() {
            true => {
                println!("Somebody come.");
                let count: u8 = user_input_guest();
        
                // find table
                let table_id = find_table(restaurant, count);
                if table_id != 0 {
                    println!("Guest sit at {} table", table_id);
                } else {
                    suggest_waiting(restaurant, count);
                }
            },
            false => println!("No new guest(s)."),
        };

        restaurant.tick += 1;
        println!("Current status: {:#?}", restaurant);
    }
}
