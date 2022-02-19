use tide::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub asset_id: i32,
    pub vendor_id: i32,
    pub game_id: i32,
    pub price: i32
}
