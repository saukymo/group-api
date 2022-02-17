
use tide::prelude::*;
use tide::{Request, Result, Response, Body};

pub async fn get_assets(_request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn create_asset(_request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn get_asset(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"asset_id": request.param("asset_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn update_asset(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"asset_id": request.param("asset_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_asset(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"asset_id": request.param("asset_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_proposals(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"asset_id": request.param("asset_id")?.parse::<i32>()? }))?);
    Ok(res)
}