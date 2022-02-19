use tide::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vendor {
    pub vendor_id: i32,
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>
}
