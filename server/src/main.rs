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
    // dotenv().ok();
    let _ = match dotenv() {
        Ok(_) => (),
        Err(e) => println!("dotenv failed {:?}", e),
    };

    let config = config::ServiceConfig::new().unwrap();

    rocket::build()
        .manage(config)
        .mount("/", routes![world])
        .mount("/", routes![routes::get_birds])
}
