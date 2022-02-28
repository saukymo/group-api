use sqlx::{query_as, PgPool};

use crate::models::models::{Game, Asset, Vendor, NewVendor};

impl Vendor {
    pub async fn add_new_vendor(new_vendor: NewVendor, pg_conn: &PgPool) -> tide::Result<Vendor> {
        let created_vendor = query_as!(Vendor, r#"INSERT INTO vendors (name, address, avatar) VALUES ($1, $2, $3) RETURNING vendor_id, name, address, avatar
    "#, new_vendor.name, new_vendor.address, new_vendor.avatar).fetch_one(pg_conn).await?;

        Ok(created_vendor)
    }

    pub async fn get_vendors(pg_conn: &PgPool) -> tide::Result<Vec<Vendor>> {
        let vendors = query_as!(Vendor, r#"SELECT vendor_id, name, address, avatar FROM vendors"#).fetch_all(pg_conn).await?;

        Ok(vendors)
    }

    pub async fn get_vendor_by_vendor_id(vendor_id: i32, pg_conn: &PgPool) -> tide::Result<Option<Vendor>> {
        let vendor = query_as!(Vendor, r#"SELECT vendor_id, name, address, avatar FROM vendors WHERE vendor_id=$1"#, vendor_id).fetch_optional(pg_conn).await?;

        Ok(vendor)
    }

    pub async fn get_games_by_vendor_id(vendor_id: i32, pg_conn: &PgPool) -> tide::Result<Vec<Game>> {
        
        let games_ids = Asset::get_game_id_by_vendor_id(vendor_id, pg_conn).await?;

        let games = query_as!(Game, r#"
            SELECT 
                game_id,
                name,
                author,
                publisher,
                description,
                quota,
                cover
            FROM games WHERE game_id = ANY($1)
            "#, &games_ids[..]).fetch_all(pg_conn).await?;

        Ok(games)
    }
}