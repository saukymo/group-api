
use tide::prelude::*;
use tide::{Request, Result, Response, Body};

pub async fn get_games(_request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn create_game(_request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn get_game(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn update_game(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_game(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_proposals(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_vendors(request: Request<()>) -> Result {
    let mut res = Response::new(200);
    println!("{:?}", request.param("game_id")?);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}