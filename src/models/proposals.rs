use tide::prelude::*;
use sqlx::{query_as, PgPool};
use chrono::{DateTime, Utc};
use serde_with::{serde_as, TimestampSeconds};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub proposal_id: i32,
    pub vendor_id: i32,
    pub asset_id: i32,
    pub price: i32,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub date_time: DateTime<Utc>,
    pub quota: i32
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProposal {
    pub vendor_id: i32,
    pub asset_id: i32,
    pub price: i32,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub date_time: DateTime<Utc>,
    pub quota: i32
}

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
}