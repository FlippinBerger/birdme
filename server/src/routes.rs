use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Bird {
    pub name: String,
    pub scientific_name: String,
}

#[get("/birds")]
pub fn get_birds() -> Json<Vec<Bird>> {
    Json(vec![Bird {
        name: String::from("American Robin"),
        scientific_name: String::from("blah"),
    }])
}
