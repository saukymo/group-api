use super::*;
use tide::Response;
use surf;

use crate::models::models::{NewUser, User, Appointment, WxLoginRequest, WxLoginResponse};

pub async fn login(request: Request<State>) -> Result {
    let app_id = std::env::var("APP_ID")?;
    let app_secret = std::env::var("APP_SECRET")?;

    let wx_req: WxLoginRequest = request.query()?;
    let code = wx_req.code;

    println!("{:?}", app_id);
    println!("{:?}", app_secret);
    println!("{:?}", code);

    let wx_res: WxLoginResponse = surf::get(format!("https://api.weixin.qq.com/sns/jscode2session?appid={app_id}&secret={app_secret}&js_code={code}&grant_type=authorization_code")).recv_json().await?;

    println!("{:?}", wx_res);
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "session": wx_res
    }))?);
    Ok(res)

}

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
    let db_pool = request.state().pool.clone();

    let created_user = User::add_new_user(
        user, &db_pool
    ).await?;

    println!("{:?}", created_user);

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&json!({
        "user": created_user
    }))?);
    Ok(res)
}

pub async fn get_user(request: Request<State>) -> Result {
    let user_id = request.param("user_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();
    
    let result = User::get_user_by_user_id(user_id, &db_pool).await?;

    let res = match result {
        Some(user) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(Body::from_json(&json!({"user": user}))?);
            response
        },
        None => {
            let mut response = Response::new(StatusCode::NotFound);
            response.set_body(Body::from_json(&json!({
                "status": "error",
                "message": "User ID not found."
            }))?);
            response
        }
    };

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
    let user_id = request.param("user_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let appointments = Appointment::get_appointments_by_user_id(user_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "appointments": appointments
    }))?);
    Ok(res)
}

pub async fn get_proposals(request: Request<State>) -> Result {
    let user_id = request.param("user_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let proposals = Proposal::get_proposals_by_user_id(user_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "proposals": proposals
    }))?);
    Ok(res)
}