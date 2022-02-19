use tide::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub proposal_id: i32,
    pub vendor_id: i32,
    pub asset_id: i32,
    pub price: i32,
    pub date_time: NaiveDateTime,
    pub quota: i32
}
