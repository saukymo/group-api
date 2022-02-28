use super::*;
use tide::{Request, Result, Response, Body, StatusCode};

use crate::models::games::{NewGame, Game};

pub async fn get_games(request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();

    let games = Game::get_games(&db_pool).await?;
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "games": games
    }))?);
    Ok(res)
}

pub async fn create_game(mut request: Request<State>) -> Result {
    let game: NewGame = request.body_json().await?;
    let db_pool = request.state().pool.clone();

    let created_game = Game::add_new_game(game, &db_pool).await?;

    println!("{:?}", created_game);

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&json!({
        "game": created_game
    }))?);
    Ok(res)
}

pub async fn get_game(request: Request<State>) -> Result {
    let game_id = request.param("game_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let result = Game::get_game_by_game_id(game_id, &db_pool).await?;

    println!("{:?}", result);
    let res = match result {
        Some(game) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(Body::from_json(&json!({
                "game": game 
            }))?);

            response
        },
        None => {
            let mut response = Response::new(StatusCode::NotFound);
            response.set_body(Body::from_json(&json!({
                "status": "error",
                "message": "Game ID not found."
            }))?);
            response
        }
    };
    
    Ok(res)
}

pub async fn update_game(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_game(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_proposals(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_vendors(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    println!("{:?}", request.param("game_id")?);
    res.set_body(Body::from_json(&json!({"game_id": request.param("game_id")?.parse::<i32>()? }))?);
    Ok(res)
}