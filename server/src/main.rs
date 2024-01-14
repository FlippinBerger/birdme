use dotenv::dotenv;

mod routes;

#[macro_use]
extern crate rocket;

#[get("/hello")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    println!("Running birdme server...");
    dotenv().ok();

    // let ebird_api_key = std::env::var("EBIRD_API_KEY").expect("EBIRD_API_KEY must be set");

    rocket::build()
        .mount("/", routes![world])
        .mount("/", routes![routes::get_birds])
}
