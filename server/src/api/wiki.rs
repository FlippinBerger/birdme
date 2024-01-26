// use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use std::fmt;

pub struct WikiService {
    pub client_id: String,
    pub client_secret: String,

    client: reqwest::Client,
    // refresh_token: String,
}

pub struct WikiInfo {
    pub title: String,
    pub snippet: String,
}

// used for both access tokens and refresh tokens
const TOKEN_ENDPOINT: &str = "https://meta.wikimedia.org/w/rest.php/oauth2/access_token";

#[derive(Deserialize)]
struct WikiAuthResponse {
    access_token: String,
}

struct Auth {
    tokens: WikiAuthResponse,
}

#[derive(Debug, Clone)]
pub struct WikiError {
    pub message: String,
}

impl WikiError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for WikiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl WikiService {
    pub async fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, name: &str) -> Result<WikiInfo, WikiError> {
        let auth = Self::auth(&self.client_id, &self.client_secret)
            .await
            .unwrap();

        let url = "https://api.wikimedia.org/core/v1/wikipedia/en/search/page"; //q=earth&limit=10
        let res = self
            .client
            .get(url)
            .header(
                "Authorization",
                "Bearer ".to_owned() + &auth.tokens.access_token,
            )
            .query(&[("q", name)])
            .query(&[("limit", "5")])
            .send()
            .await
            .expect("error searching wiki")
            .json::<PagesResult>()
            .await;

        match res {
            Ok(r) => {
                // let url = format_html_link(&r.pages[0].title);
                // let res = self
                //     .client
                //     .get(url)
                //     .header(
                //         "Authorization",
                //         "Bearer ".to_owned() + &auth.tokens.access_token,
                //     )
                //     // .query(&[("q", name)])
                //     // .query(&[("limit", "5")])
                //     .send()
                //     .await
                //     .expect("error searching wiki")
                //     .text()
                //     .await;

                // if let Ok(s) = res {
                //     let sanitized = sanitize_snippet(&s);
                //     fs::write("/tmp/bird.html", s).expect("unable to write to file");
                //     // let sanitized = sanitize_snippet(&s);
                //     fs::write("/tmp/sanitized_bird.html", sanitized)
                //         .expect("Unable to write sanitzied file");
                // }

                Ok(WikiInfo {
                    title: r.pages[0].title.clone(),
                    snippet: sanitize_snippet(&r.pages[0].excerpt.clone()),
                })
            }
            Err(e) => {
                println!("oops we fucked up somewhere searching: {:?}", e);
                Err(WikiError::new(format!("{:?}", e)))
            }
        }
    }

    async fn auth(client_id: &str, client_secret: &str) -> Result<Auth, WikiError> {
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ];

        let client = reqwest::Client::new();

        let res = client
            .post(TOKEN_ENDPOINT)
            .form(&params)
            .send()
            .await
            .expect("Unable to fetch wiki token");

        let new_res = res.json::<WikiAuthResponse>().await;

        match new_res {
            Ok(r) => Ok(Auth { tokens: r }),
            Err(e) => {
                println!("Couldn't get the wiki auth tokens: {}", e);
                Err(WikiError::new(format!("{:?}", e)))
            }
        }
    }
}

// fn format_html_link(bird_name: &str) -> String {
//     let mut name_portion = String::from(bird_name);
//     name_portion = name_portion.replace(" ", "_");

//     let base = "https://api.wikimedia.org/core/v1/wikipedia/en/page/";

//     format!("{}{}/html", base, name_portion)
// }

// sanitize_snippet takes the html tags out of the given snippet
fn sanitize_snippet(snippet: &str) -> String {
    let mut clean_snippet = String::new();

    let mut start = 0;
    let mut slice = snippet;

    while start < slice.len() {
        if let Some(open) = slice.find("<") {
            if open > start {
                clean_snippet.push_str(&slice[start..open]);
            }

            slice = &slice[open + 1..];

            if let Some(close) = slice.find(">") {
                start = 0;
                slice = &slice[close + 1..];
            } else {
                break;
            }
        } else {
            clean_snippet.push_str(&slice[start..]);
            break;
        }
    }

    clean_snippet.push_str("...");

    clean_snippet
}

#[derive(Deserialize)]
struct PagesResult {
    pages: Vec<SearchResult>,
}

#[derive(Deserialize)]
struct SearchResult {
    id: usize,
    key: String,
    title: String,
    excerpt: String,
    matched_title: Option<String>,
    description: Option<String>,
    thumbnail: Option<Thumbnail>,
    // "excerpt": "<span class=\"searchmatch\">Earth</span> is the third planet from the Sun and the only astronomical object known to harbor life. About 29% of <span class=\"searchmatch\">Earth</span>'s surface is land consisting of continents",
}

#[derive(Deserialize)]
struct Thumbnail {
    mimetype: String,
    size: Option<usize>,
    width: Option<usize>,
    height: Option<usize>,
    duration: Option<usize>,
    url: String,
}
