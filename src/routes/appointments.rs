use super::*;
use tide::Response;

use crate::models::models::{NewAppointment, Appointment, AppointmentQuery};

pub async fn get_appointments(request: Request<State>) -> Result {
    let db_pool = request.state().pool.clone();

    let appointments = Appointment::get_appointments(&db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "appointments": appointments
    }))?);
    Ok(res)
}

pub async fn create_appointment(mut request: Request<State>) -> Result {
    let appointment: NewAppointment = request.body_json().await?;
    let db_pool = request.state().pool.clone();

    let created_appointment = Appointment::add_new_appointment(appointment, &db_pool).await?;

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&json!({
        "appointment": created_appointment
    }))?);
    Ok(res)
}

pub async fn get_appointment(request: Request<State>) -> Result {
    let appointment_id = request.param("appointment_id")?.parse::<i32>()?;

    let db_pool = request.state().pool.clone();

    let result = Appointment::get_appointment_by_appointment_id(appointment_id, &db_pool).await?;

    let res = match result {
        Some(appointment) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(Body::from_json(&json!({
                "appointment": appointment
            }))?);
            response
        },
        None => {
            let mut response = Response::new(StatusCode::NotFound);
            response.set_body(Body::from_json(&json!({
                "status": "error",
                "message": "Appointment ID not found."
            }))?);
            response
        }
    };

    Ok(res)
}

pub async fn update_appointment(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"appointment_id": request.param("appointment_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_appointment_by_appointment_id(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"appointment_id": request.param("appointment_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_appointment_by_user_and_proposal_id(request: Request<State>) -> Result {
    let query: AppointmentQuery = request.query()?;
    let proposal_id = query.proposal_id.unwrap();
    let user_id = query.user_id.unwrap();

    let db_pool = request.state().pool.clone();

    let appointment = Appointment::delete_appointment_by_user_and_proposal_id(user_id, proposal_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "appointment": appointment 
    }))?);
    Ok(res)
}