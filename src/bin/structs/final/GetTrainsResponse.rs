use crate::structs::r#final::TrainItem::TrainItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetTrainsResponse {
    pub status: String,
    pub data: Vec<TrainItem>,
}
