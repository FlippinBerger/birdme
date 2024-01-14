use server::api::{ebird::EbirdService, wiki::WikiService};

pub struct ServiceConfig {
    wiki: WikiService,
    ebird: EbirdService,
}

// read environment variables to stick into the different services here to be
// passed to Rocket as State
impl ServiceConfig {
    pub fn new() -> Result<Self, std::env::VarError> {
        let wiki_client_id = std::env::var("WIKI_CLIENT_ID")?;
        let wiki_client_secret = std::env::var("WIKI_CLIENT_SECRET")?;

        let ebird_api_key = std::env::var("EBIRD_API_KEY")?;

        Ok(Self {
            wiki: WikiService {
                client_id: wiki_client_id,
                client_secret: wiki_client_secret,
            },
            ebird: EbirdService {
                token: ebird_api_key,
            },
        })
    }
}
