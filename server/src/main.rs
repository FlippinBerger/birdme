use dotenv::dotenv;

mod config;
mod routes;

#[macro_use]
extern crate rocket;

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    println!("Running birdme server...");
    dotenv().ok();

    rocket::build()
        .manage(config::ServiceConfig::new())
        .mount("/", routes![world])
        .mount("/", routes![routes::get_birds])
}
