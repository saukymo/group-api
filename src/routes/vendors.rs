
use tide::prelude::*;
use tide::{Request, Result, Response, Body};

pub async fn get_vendors(_request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn create_vendor(_request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn get_vendor(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"vendor_id": request.param("vendor_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn update_vendor(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"vendor_id": request.param("vendor_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_vendor(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"vendor_id": request.param("vendor_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_proposals(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"vendor_id": request.param("vendor_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_assets(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"vendor_id": request.param("vendor_id")?.parse::<i32>()? }))?);
    Ok(res)
}