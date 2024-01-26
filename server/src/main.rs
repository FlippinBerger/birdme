use dotenv::dotenv;
use server::config;

mod routes;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    println!("Running birdme server...");
    // dotenv().ok();
    let _ = match dotenv() {
        Ok(_) => (),
        Err(e) => println!("dotenv failed {:?}", e),
    };

    let config = config::ServiceConfig::new().await.unwrap();

    let limiter = server::rate_limiter::RateLimiter::new(5);

    rocket::build()
        .manage(config)
        .manage(limiter)
        .mount("/", routes![routes::get_birds])
}
