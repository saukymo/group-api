use tide::prelude::*;
use sqlx::{query_as, PgPool};

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

impl Asset {
    pub async fn add_new_asset(new_asset: NewAsset, pg_conn: &PgPool) -> tide::Result<Asset> {
        let created_asset = query_as!(Asset, 
        r#"
            INSERT INTO assets (
                vendor_id,
                game_id,
                price
            ) VALUES ($1, $2, $3) RETURNING 
                asset_id,
                vendor_id,
                game_id,
                price
        "#, 
        new_asset.vendor_id, 
        new_asset.game_id,
        new_asset.price)
        .fetch_one(pg_conn).await?;

        Ok(created_asset)
    }
}