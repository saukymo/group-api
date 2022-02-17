
use tide::prelude::*;
use tide::{Request, Result, Response, Body};

pub async fn get_users(_request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn create_user(_request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn get_user(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"user_id": request.param("user_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn update_user(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"user_id": request.param("user_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_user(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"user_id": request.param("user_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_appointments(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"user_id": request.param("user_id")?.parse::<i32>()? }))?);
    Ok(res)
}