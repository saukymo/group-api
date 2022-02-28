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

    pub async fn get_assets(pg_conn: &PgPool) -> tide::Result<Vec<Asset>> {
        let assets = query_as!(Asset, r#"
        SELECT 
            asset_id,
            vendor_id,
            game_id,
            price
        FROM assets
        "#)
        .fetch_all(pg_conn).await?;

        Ok(assets)
    }

    pub async fn get_asset_by_asset_id(asset_id: i32, pg_conn: &PgPool) -> tide::Result<Option<Asset>> {
        let asset = query_as!(Asset, r#"
        SELECT 
            asset_id,
            vendor_id,
            game_id,
            price
        FROM assets
        WHERE
            asset_id=$1
        "#,
        asset_id)
        .fetch_optional(pg_conn).await?;

        Ok(asset)
    }
}