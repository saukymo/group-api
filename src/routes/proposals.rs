use super::*;
use std::future::Future;
use tide::Response;

use crate::models::models::{NewProposal, Proposal, Appointment};

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
    check_proposal_id_before(return_proposal, request).await
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
    check_proposal_id_before(get_appointments_by_proposal_id, request).await
}

async fn get_appointments_by_proposal_id(proposal: Proposal, request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();

    let appointments = Appointment::get_appointments_by_proposal_id(proposal.proposal_id, &db_pool).await?;

    let mut response = Response::new(StatusCode::Ok);
    response.set_body(Body::from_json(&json!({
        "appointments": appointments
    }))?);

    Ok(response)
}

async fn check_proposal_id_before<F, Fut>(f: F, request: Request<State>) -> Result 
    where 
        F: FnOnce(Proposal, Request<State>) -> Fut,
        Fut: Future<Output = Result>
{
    let proposal_id = request.param("proposal_id")?.parse::<i32>()?; 
    let db_pool = request.state().pool.clone();

    if let Some(proposal) = Proposal::get_proposal_by_proposal_id(proposal_id, &db_pool).await?{
        return f(proposal, request).await;
    } else {
        return proposal_not_found(proposal_id).await;
    }
}


async fn proposal_not_found(proposal_id: i32) -> Result {
    let mut response = Response::new(StatusCode::NotFound);
    response.set_body(Body::from_json(&json!({
        "status": "error",
        "message": format!("Proposal ID {proposal_id} not found.")
    }))?);
    Ok(response)
}

async fn return_proposal(proposal: Proposal, _request: Request<State>) -> Result {
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(Body::from_json(&json!({
        "proposal": proposal
    }))?);
    Ok(response)
}