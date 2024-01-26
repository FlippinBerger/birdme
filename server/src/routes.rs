use core::net::IpAddr;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use server::rate_limiter::RateLimiter;

use crate::config::ServiceConfig;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Bird {
    pub name: String,
    pub scientific_name: String,
    pub link: String,
    pub blurb: String,
}

#[get("/birds/<region>")]
pub async fn get_birds(
    config: &State<ServiceConfig>,
    limiter: &State<RateLimiter>,
    ip: IpAddr,
    region: &str,
) -> Option<Json<Vec<Bird>>> {
    let ip = ip.to_string();
    if !limiter.can_request(ip) {
        // TODO can change this into an error type for better UX
        return None;
    }

    let birds = config.ebird.get_birds(region).await;

    let mut r_birds: Vec<Bird> = vec![];
    for bird in birds {
        let link = format_link(&bird.name);

        let wiki_info = config.wiki.get(&bird.name).await;
        match wiki_info {
            Ok(info) => {
                let b = Bird {
                    name: bird.name.clone(),
                    scientific_name: bird.scientific_name.clone(),
                    link: link.clone(),
                    blurb: info.snippet,
                };

                r_birds.push(b);
            }
            Err(e) => {
                println!(
                    "Unable to get the wiki info for {}: {}",
                    bird.name, e.message
                );
            }
        }
    }

    Some(Json(r_birds))
}

// format_link generates a wiki link by taking the common name of the
// bird and replacing spaces with underscores
fn format_link(bird_name: &str) -> String {
    let base = "https://en.wikipedia.org/wiki/";
    let mut name_portion = String::from(bird_name);
    name_portion = name_portion.replace(" ", "_");

    format!("{}{}", base, name_portion)
}
