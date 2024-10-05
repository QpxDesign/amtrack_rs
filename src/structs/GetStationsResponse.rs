use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetStationsResponse {
    pub status: String,
    pub data: Vec<Stations>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stations {
    pub name: Option<String>,
    pub code: Option<String>,
}
