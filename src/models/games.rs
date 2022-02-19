use tide::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub game_id: i32,
    pub name: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub avatar: Option<String>,
    pub price: i32,
    pub description: String,
    pub quota: i32
}

