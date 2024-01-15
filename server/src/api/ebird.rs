const BASE_URL: &str = "https://api.ebird.org/v2/";
const KEY_HEADER: &str = "x-ebirdapitoken";

pub struct EbirdService {
    pub token: String,
    client: reqwest::Client,
}

pub struct Bird {
    name: String,
    family_name: String,
    scientific_name: String,
}

impl EbirdService {
    pub fn new(token: String) -> Self {
        let client = reqwest::Client::new();

        Self { token, client }
    }

    pub async fn get_birds(&self) -> Bird {
        let species_codes = self.get_species_codes_for_region("US-CO").await;
        println!("species codes are: {:?}", species_codes);

        Bird {
            name: String::from("sparrow"),
            family_name: String::from("sparrow"),
            scientific_name: String::from("sparrow"),
        }
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
}
