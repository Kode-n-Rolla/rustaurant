use std::io::{self, Write};

use crate::restaurant::{model::{GuestGroup, Restaurant, RestaurantStatus, Table}};

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

pub fn create_rustaurant() -> Restaurant {
    let tables = create_tables();
    let rustaurant = Restaurant {
        tick: 0,
        status: RestaurantStatus::Open,
        tables,
        waiting_queue: vec![],
    };

    rustaurant
}

pub fn find_table(restaurant: &mut Restaurant, count_of_guests: u8) -> u32 { 
    for table in restaurant.tables.iter_mut() {
        if table.capacity >= count_of_guests && table.remaining_ticks == 0 {
            table.remaining_ticks = 4; //@todo move to restaurant config
            return table.id;
        }
    }
    
    0 // @todo think about return Option
}

pub fn tick(restaurant: &mut Restaurant) {
for table in restaurant.tables.iter_mut() {
        if table.remaining_ticks > 0 {
            table.remaining_ticks -= 1;
            if table.remaining_ticks == 0 {
                //println!("{} is free.", table.id);
            }
        }
    }

    if !restaurant.waiting_queue.is_empty() {
        let table = find_table(restaurant, restaurant.waiting_queue[0].size);
        if table > 0 {
            println!("Group {} id seat the {} table", restaurant.waiting_queue[0].id, table);
            restaurant.waiting_queue.remove(0);
        }
    }
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

fn create_tables() -> Vec<Table> {
// table capacity -> count of tables
    let tables_config = [
        (1, 1), // 4 tables for 1 person
        (2, 0), // 4 tables for 2 persons
        (3, 0), // 3 tables for 3 persons
        (4, 0), // 2 tables for 4 persons
        (5, 1), // 1 tables for 5 persons
    ];
    let mut tables: Vec<Table> = Vec::new();
    
    let mut ids = 1;

    for (capacity, count) in tables_config {
        for _ in 0..count {
            tables.push(Table::new(ids, capacity));
            ids += 1;
        }
    }

    tables
}
