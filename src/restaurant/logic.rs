use std::io;

use crate::restaurant::model::{Restaurant, RestaurantStatus, Table};

pub fn user_input_guest() -> u8 {

    println!("Input count of guest(s):");

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

pub fn find_table(restaurant: &mut Restaurant, count_of_guests: u8) -> u32{ 
    for table in restaurant.tables.iter_mut() {
        if table.capacity >= count_of_guests && table.remaining_ticks == 0 {
            table.remaining_ticks = 2;
            return table.id;
        }
    }
    
    0 // @todo think about return Option
}

pub fn tick(restaurant: &mut Restaurant) {
    for table in restaurant.tables.iter_mut() {
        if table.remaining_ticks != 0 {
            table.remaining_ticks -= 1;
            //@todo add message, if table is free now
        }
    }
}

fn create_tables() -> Vec<Table> {
    // 4 tables for 1 person
    // let tables_for_1 = (1..=4).map(|id| Table {
    //     id,
    //     capacity: 1,
    //     remaining_ticks: 0,
    // });
    // 4 tables for 2 person
    // 4 tables for 3 person
    // 3 tables for 4 person
    // 2 tables for 5 person
    // Total 17 tables
    let mut tables: Vec<Table> = Vec::new();

    // Add tabels for 1
    for id in 1..=17 {
        if id < 5 {
            let table = Table {
                id,
                capacity: 1,
                remaining_ticks: 0,
            };
            tables.push(table);
        } else if id < 9 {
            let table = Table {
                id,
                capacity: 2,
                remaining_ticks: 0,
            };
            tables.push(table);
        } else if id < 13 {
            let table = Table {
                id,
                capacity: 3,
                remaining_ticks: 0,
            };
            tables.push(table);
        } else if id < 16 {
            let table = Table {
                id,
                capacity: 4,
                remaining_ticks: 0,
            };
            tables.push(table);
        } else {
            let table = Table {
                id,
                capacity: 5,
                remaining_ticks: 0,
            };
            tables.push(table);
        }
    }

    tables
}
