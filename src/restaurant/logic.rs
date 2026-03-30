use crate::restaurant::{model::{Restaurant, RestaurantStatus, Table}};

pub fn create_rustaurant() -> Restaurant {
    // @todo think about `impl` with struct
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
