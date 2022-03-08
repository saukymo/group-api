use super::*;
use std::future::Future;
use tide::Response;

use crate::models::models::{NewProposal, Proposal, Appointment, Game, Vendor};

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
    check_proposal_id_before(get_proposal_by_proposal_id, request).await
}

pub async fn update_proposal(request: Request<State>) -> Result {
    check_proposal_id_before(update_proposals_by_proposal_id, request).await
}

pub async fn delete_proposal(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"proposal_id": request.param("proposal_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn get_appointments(request: Request<State>) -> Result {
    check_proposal_id_before(get_appointments_by_proposal_id, request).await
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
        return proposal_not_found_response(proposal_id).await;
    }
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

async fn get_proposal_by_proposal_id(proposal: Proposal, request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();

    let asset = Asset::get_asset_by_asset_id(proposal.asset_id, &db_pool).await?.unwrap();
    let game = Game::get_game_by_game_id(asset.game_id, &db_pool).await?;
    let vendor = Vendor::get_vendor_by_vendor_id(asset.vendor_id, &db_pool).await?;
    let appointments = Appointment::get_appointments_by_proposal_id(proposal.proposal_id, &db_pool).await?;
    let users = User::get_users_by_proposal_id(proposal.proposal_id, &db_pool).await?;

    let mut response = Response::new(StatusCode::Ok);
    response.set_body(Body::from_json(&json!({
        "proposal": proposal,
        "asset": asset,
        "game": game,
        "vendor": vendor,
        "appointments": appointments,
        "users": users
    }))?);
    Ok(response)

}

async fn update_proposals_by_proposal_id(mut proposal: Proposal, mut request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();
    let patch: Patch = request.body_json().await?;

    proposal = Proposal::update_proposals_by_proposal_id(proposal, patch, &db_pool).await?;

    proposal_to_response(proposal, request).await
}

async fn proposal_not_found_response(proposal_id: i32) -> Result {
    let mut response = Response::new(StatusCode::NotFound);
    response.set_body(Body::from_json(&json!({
        "status": "error",
        "message": format!("Proposal ID {proposal_id} not found.")
    }))?);
    Ok(response)
}

async fn proposal_to_response(proposal: Proposal, _request: Request<State>) -> Result {
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(Body::from_json(&json!({
        "proposal": proposal
    }))?);
    Ok(response)
}