use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Brewery {
    pub name: String,
    pub city: String,
    pub country: String,
    pub website_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MetaResponse {
    pub total: usize,
}
