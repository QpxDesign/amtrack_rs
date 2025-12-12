use amtk::decrypt;
use redis::Commands;
use redis::RedisError;
#[path = "./structs/mod.rs"]
mod structs;
#[path = "./utils/mod.rs"]
mod utils;
use crate::structs::raw::RawResponse::RawResponse;
use std::str;
use utils::ConvertRawTrains::ConvertRawTrains;
#[macro_use]
extern crate rocket;
use std::fs;
extern crate redis;
use chrono::Utc;
use lazy_static::lazy_static;

lazy_static! {
    static ref REDIS_CLIENT: redis::Client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
}

#[get("/")]
async fn index() -> rocket::response::content::RawHtml<&'static str> {
    rocket::response::content::RawHtml(
        r#"
        <title>AmTrack API</title>
        <p style='font-family:monospace; font-size:1.5em'>
        The API that powers AmTrack: Track Amtrak Trains. Download today on the Apple App Store and Google Play Store. See <a href='https://github.com/qpxdesign/amtrack_rs'>https://github.com/qpxdesign/amtrack_rs</a> for usage terms. </p>
        <p style='font-family:monospace; font-size:1.5em; font-weight:800'> A <a href='https://quinnpatwardhan.com'>https://quinnpatwardhan.com</a> + <a href='https://locomotive.systems'>https://locomotive.systems</a> product.</p>
        </p>
    "#,
    )
}
#[get("/get-trains")]
async fn getTrains() -> String {
    let CACHE_EXPIRY_SECONDS = 30;
    let is_redis_working = REDIS_CLIENT.get_connection().is_ok();
    if is_redis_working {
        let mut con = REDIS_CLIENT.get_connection().unwrap();
        let cache_time: i64 = con.get("amtrak_api_lastupdated").unwrap_or(0);
        if Utc::now().timestamp() - cache_time < CACHE_EXPIRY_SECONDS {
            let cached_res: String = con.get("amtrak_api_gettrains").unwrap_or("".to_string());
            if cached_res.len() > 5 {
                println!("{}", "Returing Cached");
                return cached_res;
            }
        }
    }

    let body = reqwest::get("https://maps.amtrak.com/services/MapDataService/trains/getTrainsData")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let a = decrypt(str::from_utf8(&body).unwrap());
    let data: RawResponse = serde_json::from_str(a.unwrap().as_str()).unwrap();
    let mut c = ConvertRawTrains(data).unwrap();
    let x = c
        .data
        .iter()
        .filter(|x| x.stations.len() > 0)
        .collect::<Vec<_>>();
    c.data = x.iter().map(|&item| item.clone()).collect();

    let b = serde_json::to_string(&c).unwrap();
    if is_redis_working {
        let mut con = REDIS_CLIENT.get_connection().unwrap();
        let _: () = con
            .set("amtrak_api_lastupdated", Utc::now().timestamp())
            .expect("Failed to set");
        let _: () = con
            .set("amtrak_api_gettrains", b.clone())
            .expect("failed to set");
    }
    return b.to_owned();
}

#[get("/get-trains-raw")]
async fn getTrainsRaw() -> String {
    let body = reqwest::get("https://maps.amtrak.com/services/MapDataService/trains/getTrainsData")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let a = decrypt(str::from_utf8(&body).unwrap());
    return a.unwrap();
    //let data: RawResponse = serde_json::from_str(a.unwrap().as_str()).unwrap();
    //return serde_json::to_string(&data).unwrap();
}

#[get("/get-stations")]
async fn getStations() -> String {
    let file_contents = fs::read_to_string("static/get-stations.json");
    if file_contents.is_ok() {
        return file_contents.unwrap();
    } else {
        println!("{}", file_contents.err().unwrap());
        return "".to_string();
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 1971)))
        .mount("/", routes![index, getTrains, getStations, getTrainsRaw])
}
