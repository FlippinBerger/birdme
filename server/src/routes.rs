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
    let _ = config.ebird.get_birds().await;
    Json(vec![Bird {
        name: String::from("American Robin"),
        scientific_name: String::from("blah"),
    }])
}
