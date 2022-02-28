use super::*;
use tide::{Request, Result, Response, Body, StatusCode};

use crate::models::assets::{NewAsset, Asset};

pub async fn get_assets(request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();
    
    let assets = Asset::get_assets(&db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "assets": assets
    }))?);
    Ok(res)
}

pub async fn create_asset(mut request: Request<State>) -> Result {
    let asset: NewAsset = request.body_json().await?;
    let db_pool = request.state().pool.clone();

    let created_asset = Asset::add_new_asset(asset, &db_pool).await?;

    println!("{:?}", created_asset);

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&json!({
        "asset": created_asset
    }))?);
    Ok(res)
}

pub async fn get_asset(request: Request<State>) -> Result {
    let asset_id = request.param("asset_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let asset = Asset::get_asset_by_asset_id(asset_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "asset": asset
     }))?);
    Ok(res)
}

pub async fn update_asset(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"asset_id": request.param("asset_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_asset(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"asset_id": request.param("asset_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_proposals(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"asset_id": request.param("asset_id")?.parse::<i32>()? }))?);
    Ok(res)
}