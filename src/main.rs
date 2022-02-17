use tide::prelude::*;
use tide::{Middleware, Next, Request, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GroupResponse { 
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proposal_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    appointment_id: Option<i32>
}

mod routes;

struct ReturnPathMiddleWare;


#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ReturnPathMiddleWare {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        

        let method = req.method().to_string();
        let path = req.url().path().to_string();
        let mut res = next.run(req).await;
        let body = res.take_body();

        let mut response: GroupResponse = body.into_json().await?;
        response.path = Some(format!("{method} {path}"));

        res.set_body(json!(&response));
        Ok(res)
    }
}


#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
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
            .delete(|req| async move { routes::appointments::delete_appointment(req).await});

    // // proposal is belong to a asset so that is also belong to a vendor
    app.at("/proposals")
        .get(|req| async move { routes::proposals::get_proposals(req).await })
        .post(|req| async move { routes::proposals::create_proposal(req).await })
        .at("/:proposal_id")
            .get(|req| async move { routes::proposals::get_proposal(req).await })
            .patch(|req| async move { routes::proposals::update_proposal(req).await})
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