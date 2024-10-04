#[path = "../structs/mod.rs"]
mod structs;
extern crate chrono;
extern crate chrono_tz;
use crate::structs::r#final::TrainStation::TrainStation;
use crate::structs::raw::RawResponse::RawResponse;
use crate::structs::raw::RawStation::RawStation;
use chrono::offset::TimeZone;
use chrono::NaiveDateTime;
use chrono::Utc;
use structs::r#final::GetTrainsResponse::GetTrainsResponse;
use uuid::Uuid;

pub fn ConvertRawTrains(raw: RawResponse) -> Option<GetTrainsResponse> {
    if raw.features.is_none() {
        return None;
    }
    let mut trains = raw.features.unwrap();
    let mut out: GetTrainsResponse = GetTrainsResponse {
        status: "worked".to_string(),
        data: Vec::new(),
    };
    trains.into_iter().for_each(|t| {
        let mut item = crate::structs::r#final::TrainItem::TrainItem {
            routeName: "".to_string(),
            trainNum: "".to_string(),
            trainID: "".to_string(), //TODO
            lat: 0.0,
            lon: 0.0,
            trainTimely: "".to_string(),
            stations: Vec::new(),
        };
        if t.properties.is_none() {
            return;
        }
        let properties = t.properties.unwrap();
        if t.geometry.is_some() {
            if t.geometry.clone().unwrap().coordinates.is_some()
                && t.geometry.clone().unwrap().coordinates.unwrap().len() == 2
            {
                item.lat = t.geometry.clone().unwrap().coordinates.unwrap()[1];
                item.lon = t.geometry.unwrap().coordinates.unwrap()[0];
            }
        }
        if properties.RouteName.is_some() {
            item.routeName = properties.RouteName.unwrap();
        }
        if properties.TrainNum.is_some() {
            item.trainNum = properties.TrainNum.unwrap();
        }
        if properties.OBJECTID.is_some() {
            item.trainID = properties.OBJECTID.unwrap().to_string();
        } else {
            item.trainID = Uuid::new_v4().to_string();
        }
        let mut stations: Vec<Option<String>> = Vec::new();
        stations.push(properties.Station1);
        stations.push(properties.Station2);
        stations.push(properties.Station3);
        stations.push(properties.Station4);
        stations.push(properties.Station5);
        stations.push(properties.Station6);
        stations.push(properties.Station7);
        stations.push(properties.Station8);
        stations.push(properties.Station9);
        stations.push(properties.Station10);
        stations.push(properties.Station11);
        stations.push(properties.Station12);
        stations.push(properties.Station13);
        stations.push(properties.Station14);
        stations.push(properties.Station15);
        stations.push(properties.Station16);
        stations.push(properties.Station17);
        stations.push(properties.Station18);
        stations.push(properties.Station19);
        stations.push(properties.Station20);
        stations.push(properties.Station21);
        stations.push(properties.Station22);
        stations.push(properties.Station23);
        stations.push(properties.Station24);
        stations.push(properties.Station25);
        stations.push(properties.Station26);
        stations.push(properties.Station27);
        stations.push(properties.Station28);
        stations.push(properties.Station29);
        stations.push(properties.Station30);
        stations.push(properties.Station31);
        stations.push(properties.Station32);
        stations.push(properties.Station33);
        stations.push(properties.Station34);
        stations.push(properties.Station35);
        stations.push(properties.Station36);
        stations.push(properties.Station37);
        stations.push(properties.Station38);
        stations.push(properties.Station39);
        stations.push(properties.Station40);
        stations.push(properties.Station41);
        stations.push(properties.Station42);
        stations.push(properties.Station43);
        stations.push(properties.Station44);
        stations.push(properties.Station45);
        stations.push(properties.Station46);
        //TODO: add in id shit
        stations
            .clone()
            .into_iter()
            .filter(|s| s.is_some())
            .for_each(|s| {
                let mut item_station = TrainStation {
                    name: "".to_string(), //TODO
                    code: "".to_string(),
                    tz: "America/New_York".to_string(),
                    bus: false,
                    schArr: "".to_string(),
                    arr: "".to_string(),
                    dep: "".to_string(),
                    arrCmnt: "".to_string(),
                    depCmnt: "".to_string(),
                    status: "".to_string(),
                    platform: "".to_string(),
                };
                let rS: RawStation = serde_json::from_str(s.unwrap().as_str()).unwrap();
                if rS.code.clone().is_some() {
                    item_station.code = rS.code.unwrap();
                }
                if rS.tz.clone().is_some() {
                    if rS.tz.clone().unwrap() == "P" {
                        item_station.tz = "America/Los_Angeles".to_string();
                    }
                    if rS.tz.clone().unwrap() == "C" {
                        item_station.tz = "America/Chicago".to_string();
                    }
                    if rS.tz.clone().unwrap() == "E" {
                        item_station.tz = "America/New_York".to_string();
                    }
                    if rS.tz.clone().unwrap() == "M" {
                        item_station.tz = "America/Denver".to_string();
                    }
                }
                if rS.scharr.clone().is_some() {
                    item_station.schArr =
                        convertTime(rS.scharr.unwrap().to_string(), item_station.tz.clone());
                }
                if rS.estarr.clone().is_some() {
                    item_station.arr =
                        convertTime(rS.estarr.unwrap().to_string(), item_station.tz.clone());
                }
                if rS.estdep.clone().is_some() {
                    item_station.dep =
                        convertTime(rS.estdep.unwrap().to_string(), item_station.tz.clone());
                }
                if rS.estarrcmnt.clone().is_some() {
                    item_station.arrCmnt = rS.estarrcmnt.unwrap().to_string();
                }
                if rS.estdepcmnt.clone().is_some() {
                    item_station.depCmnt = rS.estdepcmnt.unwrap().to_string();
                }
                if item_station.dep != "" {
                    item.stations.push(item_station);
                }
            });
        if item.stations.len() != 0 {
            if item.stations[0].arrCmnt == "" && item.stations.len() > 1 {
                item.trainTimely = item.stations[1].arrCmnt.clone();
            } else {
                item.trainTimely = item.stations[0].arrCmnt.clone();
            }
        }
        out.data.push(item);
    });
    return Some(out);
}

fn convertTime(time: String, tz: String) -> String {
    let parse_from_str = NaiveDateTime::parse_from_str;
    let dt = parse_from_str(&time, "%m/%d/%Y %H:%M:%S");
    let timezone: chrono_tz::Tz = tz.parse().unwrap();
    let mut unix = timezone.from_local_datetime(&dt.unwrap()).single();
    if unix.is_none() {
        return "".to_string();
    }
    let unix = unix.unwrap().timestamp();
    let u = Utc.timestamp(unix, 0).with_timezone(&timezone);
    return u.to_rfc3339();
}
