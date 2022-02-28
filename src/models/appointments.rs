use tide::prelude::*;
use sqlx::{query_as, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    pub appointment_id: i32,
    pub proposal_id: i32,
    pub user_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAppointment {
    pub proposal_id: i32,
    pub user_id: i32
}

impl Appointment {
    pub async fn add_new_appointment(new_appointment: NewAppointment, pg_conn: &PgPool) -> tide::Result<Appointment> {
        let created_appointment = query_as!(Appointment, r#"
        INSERT INTO appointments
        (proposal_id, user_id)
        VALUES
        ($1, $2)
        RETURNING appointment_id, proposal_id, user_id
        "#, new_appointment.proposal_id, new_appointment.user_id)
        .fetch_one(pg_conn)
        .await?;

        Ok(created_appointment)
    }

    pub async fn get_appointments(pg_conn: &PgPool) -> tide::Result<Vec<Appointment>> {
        let appointments = query_as!(Appointment, r#"
        SELECT 
            appointment_id,
            proposal_id,
            user_id
        FROM appointments
        "#).fetch_all(pg_conn).await?;

        Ok(appointments)
    }

    pub async fn get_appointment_by_appointment_id(appointment_id: i32, pg_conn: &PgPool) -> tide::Result<Option<Appointment>> {
        let appointment = query_as!(Appointment, r#"
        SELECT 
            appointment_id,
            proposal_id,
            user_id
        FROM appointments
        WHERE appointment_id=$1
        "#, appointment_id)
        .fetch_optional(pg_conn)
        .await?;

        Ok(appointment)
    }

}
