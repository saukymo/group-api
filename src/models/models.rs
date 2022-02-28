use tide::prelude::*;
use chrono::{DateTime, Utc};
use serde_with::{serde_as, TimestampSeconds};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub wx_open_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub wx_open_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    pub appointment_id: i32,
    pub proposal_id: i32,
    pub user_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAppointment {
    pub proposal_id: i32,
    pub user_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub asset_id: i32,
    pub vendor_id: i32,
    pub game_id: i32,
    pub price: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAsset {
    pub vendor_id: i32,
    pub game_id: i32,
    pub price: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub game_id: i32,
    pub name: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub description: String,
    pub quota: i32,
    pub cover: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGame {
    pub name: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub description: String,
    pub quota: i32,
    pub cover: Option<String>
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub proposal_id: i32,
    pub vendor_id: i32,
    pub asset_id: i32,
    pub price: i32,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub date_time: DateTime<Utc>,
    pub quota: i32
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProposal {
    pub vendor_id: i32,
    pub asset_id: i32,
    pub price: i32,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub date_time: DateTime<Utc>,
    pub quota: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vendor {
    pub vendor_id: i32,
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewVendor {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>
}