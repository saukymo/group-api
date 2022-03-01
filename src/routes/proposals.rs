use super::*;
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
    let proposal_id = request.param("proposal_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let result = Proposal::get_proposal_by_proposal_id(proposal_id, &db_pool).await?;

    let res = match result {
        Some(proposal) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(Body::from_json(&json!({
                "proposal": proposal
            }))?);
            response
        },
        None => {
            let mut response = Response::new(StatusCode::NotFound);
            response.set_body(Body::from_json(&json!({
                "status": "error",
                "message": "Proposal ID not found."
            }))?);
            response
        }
    };

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
    let proposal_id = request.param("proposal_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let appointments = Appointment::get_appointments_by_proposal_id(proposal_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "appointments": appointments
    }))?);
    Ok(res)
}