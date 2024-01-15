use rocket::serde::{json::Json, Serialize};
use rocket::State;

use crate::config::ServiceConfig;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Bird {
    pub name: String,
    pub scientific_name: String,
}

#[get("/birds")]
pub async fn get_birds(config: &State<ServiceConfig>) -> Json<Vec<Bird>> {
    let birds = config.ebird.get_birds().await;
    Json(
        birds
            .iter()
            .map(|bird| Bird {
                name: bird.name.clone(),
                scientific_name: bird.scientific_name.clone(),
            })
            .collect(),
    )
}
