use super::*;
use tide::{Request, Result, Response, Body, StatusCode};

use crate::models::vendors::{NewVendor, Vendor};

pub async fn get_vendors(request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();

    let vendors = Vendor::get_vendors(&db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "vendors": vendors
    }))?);
    Ok(res)
}

pub async fn create_vendor(mut request: Request<State>) -> Result {
    let vendor: NewVendor = request.body_json().await?;

    let db_pool = request.state().pool.clone();

    let created_vendor = Vendor::add_new_vendor(
        vendor, &db_pool
    ).await?;

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&json!({
        "vendor": created_vendor
    }))?);
    Ok(res)
}

pub async fn get_vendor(request: Request<State>) -> Result {
    let vendor_id = request.param("vendor_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let vendor = Vendor::get_vendor_by_vendor_id(vendor_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "vendor": vendor
    }))?);
    Ok(res)
}

pub async fn update_vendor(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"vendor_id": request.param("vendor_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_vendor(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"vendor_id": request.param("vendor_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_proposals(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"vendor_id": request.param("vendor_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_assets(request: Request<State>) -> Result {
    let vendor_id = request.param("vendor_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let games = Vendor::get_games_by_vendor_id(vendor_id, &db_pool).await?;

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({
        "games": games
    }))?);
    Ok(res)
}