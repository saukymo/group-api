use super::*;
use tide::{Request, Result, Response, Body};

use crate::models::proposals::{NewProposal, Proposal};

pub async fn get_proposals(request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();

    let proposals = Proposal::get_proposals(&db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "proposals": proposals
    }))?);
    Ok(res)
}

pub async fn create_proposal(mut request: Request<State>) -> Result {
    let proposal: NewProposal = request.body_json().await?;
    let db_pool = request.state().pool.clone();

    let created_proposal = Proposal::add_new_proposal(proposal, &db_pool).await?;

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&json!({
        "proposal": created_proposal
    }))?);
    Ok(res)
}

pub async fn get_proposal(request: Request<State>) -> Result {
    let proposal_id = request.param("proposal_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let proposal = Proposal::get_proposal_by_proposal_id(proposal_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "proposal": proposal
     }))?);
    Ok(res)
}

pub async fn update_proposal(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"proposal_id": request.param("proposal_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_proposal(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"proposal_id": request.param("proposal_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_appointments(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"proposal_id": request.param("proposal_id")?.parse::<i32>()? }))?);
    Ok(res)
}