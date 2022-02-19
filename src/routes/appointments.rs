use super::*;
use tide::prelude::*;
use tide::{Request, Result, Response, Body};

pub async fn get_appointments(_request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn create_appointment(_request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn get_appointment(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"appointment_id": request.param("appointment_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn update_appointment(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"appointment_id": request.param("appointment_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_appointment(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"appointment_id": request.param("appointment_id")?.parse::<i32>()? }))?);
    Ok(res)
}