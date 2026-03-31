#[derive(Debug)]
pub struct Restaurant {
    pub tick: u32,
    pub status: RestaurantStatus,
    pub tables: Vec<Table>,
    pub waiting_queue: Vec<GuestGroup>,
    pub next_guest_group_id: u32,
}

#[derive(Debug)]
pub struct Table {
    pub id: u32,
    pub capacity: u8,
    pub remaining_ticks: u32,
}

impl Table {
    pub fn new(id: u32, capacity: u8) -> Self {
        Self {
            id,
            capacity,
            remaining_ticks: 0,
        }
    }
}

#[derive(Debug)]
pub struct GuestGroup {
    pub id: u32,
    pub size: u8,
}

#[derive(Debug, PartialEq)]
pub enum RestaurantStatus {
    Open,
    Closed,
}
