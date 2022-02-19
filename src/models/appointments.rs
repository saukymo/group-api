use tide::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    pub appointment_id: i32,
    pub proposal_id: i32,
    pub user_id: i32
}