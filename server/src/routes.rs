#[get("/birds")]
pub fn get_birds() -> &'static str {
    "Here are the birds"
}
