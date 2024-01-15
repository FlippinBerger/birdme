use rand::Rng;
use serde::Deserialize;
use std::collections::HashSet;

const BASE_URL: &str = "https://api.ebird.org/v2/";
const KEY_HEADER: &str = "x-ebirdapitoken";

pub struct EbirdService {
    pub token: String,
    client: reqwest::Client,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TaxonomyResponse {
    sci_name: String,
    com_name: String,
    species_code: String,
    category: String,
    taxon_order: f32,
    banding_codes: Vec<String>,
    com_name_codes: Vec<String>,
    sci_name_codes: Vec<String>,
    order: String,
    family_code: String,
    family_com_name: String,
    family_sci_name: String,
}

pub struct Bird {
    pub name: String,
    pub family_name: String,
    pub scientific_name: String,
}

impl EbirdService {
    pub fn new(token: String) -> Self {
        let client = reqwest::Client::new();

        Self { token, client }
    }

    pub async fn get_birds(&self) -> Vec<Bird> {
        let species_codes = self.get_species_codes_for_region("US-CO").await;

        // choose a few random species to return to the user
        let codes = choose_random_codes(&species_codes, 5);

        self.get_taxonomy_for_codes(&codes).await
    }

    // for some reason, the species codes are returned as plain text from the API
    // so this method does some string stuff to clean up the result we get back
    async fn get_species_codes_for_region(&self, region: &str) -> Vec<String> {
        let v = match self
            .client
            .get(format!("{}product/spplist/{}", BASE_URL, region))
            .header(KEY_HEADER, &self.token)
            .send()
            .await
        {
            Ok(res) => {
                // parse as text and clean up the response
                if let Ok(r) = res.text().await {
                    let v = r[1..r.len() - 2]
                        .replace("\"", "")
                        .split(",")
                        .map(|code| String::from(code))
                        .collect();
                    v
                } else {
                    println!("Unable to parse the species codes as text");
                    vec![]
                }
            }
            Err(err) => {
                println!(
                    "Oops we failed fetching the species codes for region {}: {}",
                    region, err
                );
                vec![]
            }
        };

        v
    }

    async fn get_taxonomy_for_codes(&self, species_codes: &Vec<String>) -> Vec<Bird> {
        let codes = species_codes.join(",");

        let birds = match self
            .client
            .get(format!("{}ref/taxonomy/ebird", BASE_URL))
            .header(KEY_HEADER, &self.token)
            .query(&[("species", codes)])
            .query(&[("fmt", "json")])
            .send()
            .await
            .expect("didn't error grabbing the taxonomy")
            .json::<Vec<TaxonomyResponse>>()
            .await
        {
            Ok(taxes) => taxes
                .iter()
                .map(|tax| Bird {
                    name: tax.com_name.clone(),
                    family_name: tax.family_com_name.clone(),
                    scientific_name: tax.sci_name.clone(),
                })
                .collect(),
            Err(err) => {
                println!("Oops we failed fetching the taxonomies: {}", err);
                vec![]
            }
        };

        birds
    }
}

// choose_random_codes utilizes a random number generator to snag some random
// species codes to show the user
fn choose_random_codes(species_codes: &Vec<String>, number_to_choose: u8) -> Vec<String> {
    let mut codes: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut set = HashSet::new();

    let max_counter = 100;
    let mut kill_it = false;

    for _ in 0..number_to_choose {
        let mut i = 0;
        loop {
            let random_number = rng.gen_range(0..species_codes.len());
            if !set.contains(&random_number) {
                set.insert(random_number);
                codes.push(String::from(&species_codes[random_number]));
                break;
            }
            i += 1;
            if i >= max_counter {
                kill_it = true;
                break;
            }
        }
        if kill_it {
            break;
        }
    }

    codes
}
