use super::*;
use tide::Response;

use crate::models::models::{NewAsset, Asset, Proposal, Game, Vendor};

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

    let result = Asset::get_asset_by_asset_id(asset_id, &db_pool).await?;

    let res = match result {
        Some(asset) => {

            let game = Game::get_game_by_game_id(asset.game_id, &db_pool).await?;
            let vendor = Vendor::get_vendor_by_vendor_id(asset.vendor_id, &db_pool).await?;

            let mut response = Response::new(StatusCode::Ok);
            response.set_body(Body::from_json(&json!({
                "asset": asset,
                "game": game,
                "vendor": vendor
            }))?);
            response
        },
        None => {
            let mut response = Response::new(StatusCode::NotFound);
            response.set_body(Body::from_json(&json!({
                "status": "error",
                "message": "Asset ID not found."
            }))?);
            response
        }
    };

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
    let asset_id = request.param("asset_id")?.parse::<i32>()?;
    
    let db_pool = request.state().pool.clone();

    let proposals = Proposal::get_proposals_by_asset_id(asset_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "proposals": proposals
    }))?);
    Ok(res)
}