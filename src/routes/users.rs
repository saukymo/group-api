
use super::*;
use tide::prelude::*;
use sqlx::prelude::*;
use sqlx::postgres::Postgres;
use tide::{Request, Result, Response, Body, StatusCode};

use crate::models::users::{NewUser, User};

pub async fn get_users(request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();

    let users = User::get_users(&db_pool).await?;
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "users": users
    }))?);
    Ok(res)
}

pub async fn create_user(mut request: Request<State>) -> Result {
    let user: NewUser = request.body_json().await?;
    println!("User name: {}", user.name);
    println!("User wx_open_id: {:?}", user.wx_open_id);

    // let mut pg_conn = request.sqlx_conn::<Postgres>().await;
    let db_pool = request.state().pool.clone();

    // let created_user = sqlx::query!(r#"INSERT INTO users (name) VALUES ($1) returning user_id, name
    // "#, user.name).fetch_one(pg_conn.acquire().await?).await?;
    let created_user = User::add_new_user(
        user, &db_pool
    ).await?;

    println!("{:?}", created_user);

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&json!({
        "user": Some(created_user)
    }))?);
    Ok(res)
}

pub async fn get_user(request: Request<State>) -> Result {
    let user_id = request.param("user_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();
    
    let user = User::get_user_by_user_id(user_id, &db_pool).await?;

    println!("{:?}", user);

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"user": user}))?);
    Ok(res)
}

pub async fn update_user(request: Request<State>) -> Result {
    let user_id = request.param("user_id")?.parse::<i32>()?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"user_id": user_id }))?);
    Ok(res)
}

pub async fn delete_user(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"user_id": request.param("user_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_appointments(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"user_id": request.param("user_id")?.parse::<i32>()? }))?);
    Ok(res)
}