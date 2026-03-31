use crate::restaurant::{model::{Restaurant, RestaurantStatus, Table}};

const OCCUPAY_TABLE_FOR: u32 = 4;

pub fn create_rustaurant() -> Restaurant {
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
            table.remaining_ticks = OCCUPAY_TABLE_FOR;
            return Some(table.id);
        }
    }
    
    None
}

pub fn tick(restaurant: &mut Restaurant) {
    for table in restaurant.tables.iter_mut() {
            if table.remaining_ticks > 0 {
                table.remaining_ticks -= 1;
                if table.remaining_ticks == 0 && restaurant.waiting_queue.is_empty() {
                    println!("{} is free.", table.id);
                }
            }
        }

    if !restaurant.waiting_queue.is_empty() {
        if let Some(table_id) = find_table(restaurant, restaurant.waiting_queue[0].size) {
                println!("Group {} was seated at table {}", restaurant.waiting_queue[0].id, table_id);
                restaurant.waiting_queue.remove(0);            
        }
    }
}

fn create_tables() -> Vec<Table> {
    // table capacity -> count of tables
    // @todo return default values after testing
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
