pub struct Bird {
    pub name: String,
    pub scientific_name: String,
    //TODO might be able to use a URL type here instead
    pub link: String,
    pub blurb: String,
}

pub struct BirdError {}

const BIRDME_ENDPOINT: &str = "";

pub fn fetch_birds() -> Result<Vec<Bird>, BirdError> {
    unimplemented!()
}
