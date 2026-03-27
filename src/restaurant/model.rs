#[derive(Debug)]
pub struct Restaurant {
    pub tick: u32,
    pub status: RestaurantStatus,
    pub tables: Vec<Table>,
    pub waiting_queue: Vec<GuestGroup>,
}

#[derive(Debug)]
pub struct Table {
    pub id: u32,
    pub capacity: u8,
    pub remaining_ticks: u32,
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
