use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TrainStation {
    pub name: String,
    pub code: String,
    pub tz: String,
    pub bus: bool,
    pub schArr: String,
    pub arr: String,
    pub dep: String,
    pub arrCmnt: String,
    pub depCmnt: String,
    pub status: String,
    pub platform: String,
}
