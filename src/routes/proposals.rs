use super::*;
use tide::prelude::*;
use tide::{Request, Result, Response, Body};

pub async fn get_proposals(_request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn create_proposal(_request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({}))?);
    Ok(res)
}

pub async fn get_proposal(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"proposal_id": request.param("proposal_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn update_proposal(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"proposal_id": request.param("proposal_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_proposal(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"proposal_id": request.param("proposal_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_appointments(request: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"proposal_id": request.param("proposal_id")?.parse::<i32>()? }))?);
    Ok(res)
}