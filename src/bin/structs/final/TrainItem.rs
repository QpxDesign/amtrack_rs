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
    pub velocity: f64,
    pub destCode: String,
    pub eventCode: String,
    pub heading: String,
    pub origCode: String,
    pub origSchDep: String,
    pub originTZ: String,
    pub trainState: String,
    pub updatedAt: String,
}
