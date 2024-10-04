use crate::structs::r#final::TrainStation::TrainStation;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct TrainItem {
    pub routeName: String,
    pub trainNum: String,
    pub trainID: String,
    pub lat: f64,
    pub lon: f64,
    pub trainTimely: String,
    pub stations: Vec<TrainStation>,
}
