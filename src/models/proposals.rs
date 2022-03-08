use sqlx::{query_as, query, PgPool};
use json_patch::{patch, Patch};
use serde_json;

use crate::models::models::{NewProposal, Proposal, Asset, Appointment};

impl Proposal {
    pub async fn add_new_proposal(new_proposal: NewProposal, pg_conn: &PgPool) -> tide::Result<Proposal> {
        let created_proposal = query_as!(Proposal, r#"
            INSERT INTO proposals 
            (vendor_id, asset_id, price, date_time, quota) 
            VALUES 
            ($1, $2, $3, $4, $5) 
            RETURNING 
            proposal_id, vendor_id, asset_id, price, date_time, quota
        "#, new_proposal.vendor_id, new_proposal.asset_id, new_proposal.price, new_proposal.date_time, new_proposal.quota)
        .fetch_one(pg_conn)
        .await?;

        Ok(created_proposal)
    }

    pub async fn update_proposals_by_proposal_id(mut proposal: Proposal, operations: Patch, pg_conn: &PgPool) -> tide::Result<Proposal> {
        let mut proposal_json = serde_json::to_value(proposal)?;
        patch(&mut proposal_json, &operations)?;

        proposal = serde_json::from_value(proposal_json)?;
        query!(
            r#"
                UPDATE proposals
                SET 
                    vendor_id=$2,
                    asset_id=$3,
                    price=$4,
                    date_time=$5,
                    quota=$6
                WHERE 
                    proposal_id=$1
            "#,
            proposal.proposal_id,
            proposal.vendor_id,
            proposal.asset_id,
            proposal.price,
            proposal.date_time,
            proposal.quota
        ).execute(pg_conn).await?;
        Ok(proposal)
    } 

    pub async fn get_proposals(pg_conn: &PgPool) -> tide::Result<Vec<Proposal>> {
        let proposals = query_as!(Proposal, r#"
            SELECT 
            proposal_id, vendor_id, asset_id, price, date_time, quota 
            FROM proposals
        "#).fetch_all(pg_conn).await?;

        Ok(proposals)
    }

    pub async fn get_proposal_by_proposal_id(proposal_id: i32, pg_conn: &PgPool) -> tide::Result<Option<Proposal>> {
        let proposal = query_as!(Proposal, r#"
            SELECT 
            proposal_id, vendor_id, asset_id, price, date_time, quota 
            FROM proposals 
            WHERE proposal_id=$1
        "#, proposal_id).fetch_optional(pg_conn).await?;

        Ok(proposal)
    }

    pub async fn get_proposals_by_vendor_id(vendor_id: i32, pg_conn: &PgPool) -> tide::Result<Vec<Proposal>> {
        let proposals = query_as!(Proposal, r#"
            SELECT 
            proposal_id, vendor_id, asset_id, price, date_time, quota 
            FROM proposals 
            WHERE 
            vendor_id=$1
        "#, vendor_id).fetch_all(pg_conn).await?;

        Ok(proposals)
    }

    pub async fn get_proposals_by_asset_id(asset_id: i32, pg_conn: &PgPool) -> tide::Result<Vec<Proposal>> {
        let proposals = query_as!(Proposal, r#"
            SELECT 
            proposal_id, vendor_id, asset_id, price, date_time, quota 
            FROM proposals 
            WHERE 
            asset_id=$1
        "#, asset_id).fetch_all(pg_conn).await?;

        Ok(proposals)
    }

    pub async fn get_proposals_by_game_id(game_id: i32, pg_conn: &PgPool) -> tide::Result<Vec<Proposal>> {
        let asset_ids = Asset::get_assets_by_game_id(game_id, pg_conn).await?
        .iter()
        .map(|x| x.asset_id)
        .collect::<Vec<_>>();

        let proposals = query_as!(Proposal, r#"
            SELECT 
            proposal_id, vendor_id, asset_id, price, date_time, quota 
            FROM proposals 
            WHERE 
            asset_id= ANY($1)
        "#, &asset_ids[..]).fetch_all(pg_conn).await?;

        Ok(proposals)
    }  
    
    pub async fn get_proposals_by_user_id(user_id: i32, pg_conn: &PgPool) -> tide::Result<Vec<Proposal>> {
        let proposal_ids = Appointment::get_appointments_by_user_id(user_id, pg_conn).await?
        .iter()
        .map(|x| x.proposal_id)
        .collect::<Vec<_>>();

        let proposals = query_as!(Proposal, r#"
            SELECT 
            proposal_id, vendor_id, asset_id, price, date_time, quota 
            FROM proposals 
            WHERE 
            proposal_id = ANY($1)
        "#, &proposal_ids[..]).fetch_all(pg_conn).await?;

        Ok(proposals)
    }  
}