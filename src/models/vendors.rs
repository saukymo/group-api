use tide::prelude::*;
use sqlx::{query_as, PgPool};

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

impl Vendor {
    pub async fn add_new_vendor(new_vendor: NewVendor, pg_conn: &PgPool) -> tide::Result<Vendor> {
        let create_vendor = query_as!(Vendor, r#"INSERT INTO vendors (name, address, avatar) VALUES ($1, $2, $3) RETURNING vendor_id, name, address, avatar
    "#, new_vendor.name, new_vendor.address, new_vendor.avatar).fetch_one(pg_conn).await?;

        Ok(create_vendor)
    }

    pub async fn get_vendors(pg_conn: &PgPool) -> tide::Result<Vec<Vendor>> {
        let vendors = query_as!(Vendor, r#"Select vendor_id, name, address, avatar from vendors"#).fetch_all(pg_conn).await?;

        Ok(vendors)
    }

    pub async fn get_vendor_by_vendor_id(vendor_id: i32, pg_conn: &PgPool) -> tide::Result<Option<Vendor>> {
        let vendor = query_as!(Vendor, r#"Select vendor_id, name, address, avatar from vendors where vendor_id=$1"#, vendor_id).fetch_optional(pg_conn).await?;

        Ok(vendor)
    }
}