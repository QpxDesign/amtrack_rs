#[path = "./bin/structs/mod.rs"]
pub mod structs;
#[path = "./bin/utils/mod.rs"]
pub mod utils;
use crate::structs::r#final::GetTrainsResponse::GetTrainsResponse;
use crate::structs::raw::RawResponse::RawResponse;
use amtk::decrypt;
use std::fs;
use std::str;
use structs::GetStationsResponse::GetStationsResponse;
use utils::ConvertRawTrains::ConvertRawTrains;

pub async fn GetTrains() -> Option<GetTrainsResponse> {
    let body = reqwest::get("https://maps.amtrak.com/services/MapDataService/trains/getTrainsData")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let a = decrypt(str::from_utf8(&body).unwrap());
    let data: RawResponse = serde_json::from_str(a.unwrap().as_str()).unwrap();
    return ConvertRawTrains(data);
}

pub fn GetStations() -> Option<String> {
    let file_contents = fs::read_to_string("static/get-stations.json");
    if file_contents.is_ok() {
        return Some(file_contents.unwrap());
    } else {
        println!("{}", file_contents.err().unwrap());
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stations() {
        println!("{}", GetStations().unwrap())
    }

    #[tokio::test]
    async fn test_get_trains() {
        println!("{}", GetTrains().await.unwrap().data[0].lat)
    }
}
