use dotenv::dotenv;
use tide::prelude::*;
use sqlx::Pool;
use sqlx::PgPool;
use tide::{Middleware, Next, Request, Result, Body, StatusCode};
use serde::{Deserialize, Serialize};

mod routes;
mod models;

#[derive(Debug, Serialize, Deserialize)]
struct GroupResponse { 
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<models::users::User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<models::users::User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game: Option<models::games::Game>,
    #[serde(skip_serializing_if = "Option::is_none")]
    games: Option<Vec<models::games::Game>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proposal: Option<models::proposals::Proposal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proposals: Option<Vec<models::proposals::Proposal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset: Option<models::assets::Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Vec<models::assets::Asset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<models::vendors::Vendor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendors: Option<Vec<models::vendors::Vendor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    appointment: Option<models::appointments::Appointment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    appointments: Option<Vec<models::appointments::Appointment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>
}



#[derive(Clone, Debug)]
pub struct State {
    pool: PgPool
}


struct ReturnPathMiddleWare;


#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ReturnPathMiddleWare {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        

        let method = req.method().to_string();
        let path = req.url().path().to_string();
        let mut res = next.run(req).await;

        println!("{:?}", res);

        match res.take_error() {
            None => {
                let body = res.take_body();

                let mut response: GroupResponse = body.into_json().await?;
                response.path = Some(format!("{method} {path}"));

                res.set_body(json!(&response));
            },
            Some(error) => {

                println!("Error: {:?}", error);

                let response = Body::from_json(
                    &json!({
                        "status": "error",
                        "message": error.to_string(),
                        "path": format!("{method} {path}")
                    })
                )?;

                res.set_body(response);
                res.set_status(StatusCode::BadRequest);
            }
        }
        
        Ok(res)
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    tide::log::start();
    

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = Pool::connect(&database_url).await?;
    let state = State {
        pool,
    };

    let mut app = tide::with_state(state);
    app.with(ReturnPathMiddleWare);

    app

    // user can make an appointment.
    .at("/users")
        .get(|req| async move { routes::users::get_users(req).await })
        .post(|req| async move { routes::users::create_user(req).await })
        .at("/:user_id")
            .get(|req| async move { routes::users::get_user(req).await })
            .patch(|req| async move { routes::users::update_user(req).await})
            .delete(|req| async move { routes::users::delete_user(req).await})
            .at("/appointments")
                .get(|req| async move { routes::users::get_appointments(req).await });

    app.at("/games")
        .get(|req| async move { routes::games::get_games(req).await })
        .post(|req| async move { routes::games::create_game(req).await })
        .at("/:game_id")
            .get(|req| async move { routes::games::get_game(req).await })
            .patch(|req| async move { routes::games::update_game(req).await})
            .delete(|req| async move { routes::games::delete_game(req).await})
            .at("/proposals")
                .get(|req| async move { routes::games::get_proposals(req).await });

    app.at("/games")
        .at("/:game_id")
            .at("/vendors")
                .get(|req| async move { routes::games::get_vendors(req).await });

    // // appointment is a relation between user and proposal
    app.at("/appointments")
        .get(|req| async move { routes::appointments::get_appointments(req).await })
        .post(|req| async move { routes::appointments::create_appointment(req).await })
        .at("/:appointment_id")
            .get(|req| async move { routes::appointments::get_appointment(req).await })
            .patch(|req| async move { routes::appointments::update_appointment(req).await})
            // Only user, owner or admins can delete appointment.
            .delete(|req| async move { routes::appointments::delete_appointment(req).await});

    // // proposal is belong to a asset so that is also belong to a vendor
    app.at("/proposals")
        .get(|req| async move { routes::proposals::get_proposals(req).await })
        .post(|req| async move { routes::proposals::create_proposal(req).await })
        .at("/:proposal_id")
            .get(|req| async move { routes::proposals::get_proposal(req).await })
            .patch(|req| async move { routes::proposals::update_proposal(req).await})
            // Only owner or admins can delete proposal.
            .delete(|req| async move { routes::proposals::delete_proposal(req).await})
            .at("/appointments")
                .get(|req| async move { routes::proposals::get_appointments(req).await });

    // // asset is belong to a vendor
    app.at("/assets")
        .get(|req| async move { routes::assets::get_assets(req).await })
        .post(|req| async move { routes::assets::create_asset(req).await })
        .at("/:asset_id")
            .get(|req| async move { routes::assets::get_asset(req).await })
            .patch(|req| async move { routes::assets::update_asset(req).await})
            .delete(|req| async move { routes::assets::delete_asset(req).await})
            .at("/proposals")
                .get(|req| async move { routes::assets::get_proposals(req).await });

    // // vendor has many assets which has many proposals
    app.at("/vendors")
        .get(|req| async move { routes::vendors::get_vendors(req).await })
        .post(|req| async move { routes::vendors::create_vendor(req).await })
        .at("/:vendor_id")
            .get(|req| async move { routes::vendors::get_vendor(req).await })
            .patch(|req| async move { routes::vendors::update_vendor(req).await})
            .delete(|req| async move { routes::vendors::delete_vendor(req).await})
            .at("/assets")
                .get(|req| async move { routes::vendors::get_assets(req).await });
    app.at("/vendors")
        .at("/:vendor_id")
            .at("/proposals")
                .get(|req| async move { routes::vendors::get_proposals(req).await });

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}