use super::*;
use tide::{Request, Result, Response, Body};

use crate::models::appointments::{NewAppointment, Appointment};

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

    let appointment = Appointment::get_appointment_by_appointment_id(appointment_id, &db_pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "appointment": appointment
    }))?);
    Ok(res)
}

pub async fn update_appointment(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"appointment_id": request.param("appointment_id")?.parse::<i32>()? }))?);
    Ok(res)
}

pub async fn delete_appointment(request: Request<State>) -> Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({"appointment_id": request.param("appointment_id")?.parse::<i32>()? }))?);
    Ok(res)
}