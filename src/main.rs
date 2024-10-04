use amtk::decrypt;
use chrono::{Local, Utc};
use rocket::tokio::time::{sleep, Duration};
use serde_json::{Result, Value};
#[path = "./structs/mod.rs"]
mod structs;
#[path = "./utils/mod.rs"]
mod utils;
use crate::structs::raw::RawResponse::RawResponse;
use std::str;
use utils::ConvertRawTrains::ConvertRawTrains;
#[macro_use]
extern crate rocket;

#[get("/get-trains")]
async fn index() -> String {
    let body = reqwest::get("https://maps.amtrak.com/services/MapDataService/trains/getTrainsData")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let a = decrypt(str::from_utf8(&body).unwrap());
    let data: RawResponse = serde_json::from_str(a.unwrap().as_str()).unwrap();
    let c = ConvertRawTrains(data);
    let b = serde_json::to_string(&c).unwrap();
    return b.to_owned();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
