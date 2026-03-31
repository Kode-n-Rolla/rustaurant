use crate::restaurant::{model::{Restaurant, RestaurantStatus, Table}};

const TABLE_OCCUPANCY_TICKS: u32 = 4;

pub fn create_restaurant() -> Restaurant {
    let tables = create_tables();
    let rustaurant = Restaurant {
        tick: 0,
        status: RestaurantStatus::Open,
        tables,
        waiting_queue: vec![],
        next_guest_group_id: 1,
    };

    rustaurant
}

pub fn find_table(restaurant: &mut Restaurant, count_of_guests: u8) -> Option<u32> { 
    for table in restaurant.tables.iter_mut() {
        if table.capacity >= count_of_guests && table.remaining_ticks == 0 {
            table.remaining_ticks = TABLE_OCCUPANCY_TICKS;
            return Some(table.id);
        }
    }
    
    None
}

// Advance occupied tables by one simulation tick
pub fn tick(restaurant: &mut Restaurant) {
    for table in restaurant.tables.iter_mut() {
            if table.remaining_ticks > 0 {
                table.remaining_ticks -= 1;
                if table.remaining_ticks == 0 && restaurant.waiting_queue.is_empty() {
                    println!("{} is free.", table.id);
                }
            }
        }

    // MVP queue policy: seat only the first waiting group.
    while !restaurant.waiting_queue.is_empty() {
        let first_group_id = restaurant.waiting_queue[0].id;
        let first_group_size = restaurant.waiting_queue[0].size;

        if let Some(table_id) = find_table(restaurant, first_group_size) {
            println!("Group {} was seated at table {}", first_group_id, table_id);
            restaurant.waiting_queue.remove(0);
        } else {
            break;
        }
    }
}

fn create_tables() -> Vec<Table> {
    // capacity -> number of tables
    let tables_config = [
        (1, 4), // 4 tables for 1 person
        (2, 4), // 4 tables for 2 persons
        (3, 3), // 3 tables for 3 persons
        (4, 2), // 2 tables for 4 persons
        (5, 1), // 1 tables for 5 persons
    ];
    let mut tables: Vec<Table> = Vec::new();
    
    let mut id = 1;

    for (capacity, count) in tables_config {
        for _ in 0..count {
            tables.push(Table::new(id, capacity));
            id += 1;
        }
    }

    tables
}
