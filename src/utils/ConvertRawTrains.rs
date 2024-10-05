#[path = "../structs/mod.rs"]
mod structs;
extern crate chrono;
extern crate chrono_tz;
use crate::structs::r#final::TrainStation::TrainStation;
use crate::structs::raw::RawResponse::RawResponse;
use crate::structs::raw::RawStation::RawStation;
use crate::structs::GetStationsResponse::GetStationsResponse;
use chrono::offset::TimeZone;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;
use structs::r#final::GetTrainsResponse::GetTrainsResponse;
use uuid::Uuid;

lazy_static! {
    static ref STATIONS_FILE: GetStationsResponse = readStations();
}

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
            trainID: "".to_string(),
            lat: 0.0,
            lon: 0.0,
            trainTimely: "".to_string(),
            stations: Vec::new(),
            velocity: 0.0,
            destCode: "".to_string(),
            eventCode: "".to_string(),
            heading: "".to_string(),
            origCode: "".to_string(),
            origSchDep: "".to_string(),
            trainState: "".to_string(),
            originTZ: "".to_string(),
            updatedAt: "".to_string(),
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
        if properties.Velocity.is_some() {
            item.velocity = properties.Velocity.unwrap().parse().expect("Woops");
        }
        if properties.DestCode.is_some() {
            item.destCode = properties.DestCode.unwrap();
        }
        if properties.EventCode.is_some() {
            item.eventCode = properties.EventCode.unwrap();
        }
        if properties.Heading.is_some() {
            item.heading = properties.Heading.unwrap();
        }
        if properties.OrigCode.is_some() {
            item.origCode = properties.OrigCode.unwrap();
        }
        if properties.OrigSchDep.is_some() {
            item.origSchDep = properties.OrigSchDep.unwrap();
        }
        if properties.OriginTZ.is_some() {
            item.originTZ = properties.OriginTZ.unwrap();
        }
        if properties.TrainState.is_some() {
            item.trainState = properties.TrainState.unwrap();
        }

        if properties.updated_at.is_some() {
            item.updatedAt = convertTime(
                properties.updated_at.unwrap(),
                "America/New_York".to_string(),
            );
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
        if stations[0].is_none() {
            println!("{}", "FUCK");
        }
        stations.clone().into_iter().for_each(|s| {
            if s.is_none() {
                return;
            }
            let mut item_station = TrainStation {
                name: "".to_string(), //TODO
                code: "".to_string(),
                tz: "America/New_York".to_string(),
                bus: false,
                schArr: "".to_string(),
                schDep: "".to_string(),
                arr: "".to_string(),
                dep: "".to_string(),
                arrCmnt: "".to_string(),
                depCmnt: "".to_string(),
                status: "".to_string(), // TODO
                platform: "".to_string(),
            };
            let rS: RawStation = serde_json::from_str(s.unwrap().as_str()).unwrap();
            if rS.code.clone().is_some() {
                item_station.code = rS.code.clone().unwrap();
                item_station.name = getNameFromStationCode(rS.code.unwrap());
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
            if rS.estarr.clone().is_some() && rS.estarr.clone().unwrap() != "" {
                item_station.arr =
                    convertTime(rS.estarr.unwrap().to_string(), item_station.tz.clone());
            } else if rS.postarr.clone().is_some() && rS.postarr.clone().unwrap() != "" {
                item_station.arr =
                    convertTime(rS.postarr.unwrap().to_string(), item_station.tz.clone());
            }
            if rS.estdep.clone().is_some() && rS.estdep.clone().unwrap() != "" {
                item_station.dep =
                    convertTime(rS.estdep.unwrap().to_string(), item_station.tz.clone());
            } else if rS.postdep.clone().is_some() && rS.postdep.clone().unwrap() != "" {
                item_station.dep =
                    convertTime(rS.postdep.unwrap().to_string(), item_station.tz.clone());
            }
            if rS.estarrcmnt.clone().is_some() {
                item_station.arrCmnt = rS.estarrcmnt.unwrap().to_string();
            }
            if rS.estdepcmnt.clone().is_some() {
                item_station.depCmnt = rS.estdepcmnt.unwrap().to_string();
            }
            if rS.schdep.is_some() {
                item_station.schDep =
                    convertTime(rS.schdep.unwrap().to_string(), item_station.tz.clone());
            }
            if stringToUnix(item_station.dep.clone()) < Utc::now().timestamp() {
                item_station.status = "Departed".to_string();
            } else {
                item_station.status = "Enroute".to_string();
            }
            item.stations.push(item_station);
        });
        if item.stations.len() != 0 {
            if item.stations[0].arrCmnt == "" && item.stations.len() > 1 {
                item.trainTimely = item.stations[1].arrCmnt.clone();
            } else {
                item.trainTimely = item.stations[0].arrCmnt.clone();
            }
        }
        //tripTimley
        let tt: Vec<TrainStation> = item
            .stations
            .clone()
            .into_iter()
            .filter(|x| x.status == "Enroute")
            .collect();
        if tt.len() > 0 {
            item.trainTimely = tt[0].arrCmnt.clone();
        }
        out.data.push(item);
    });
    return Some(out);
}

fn convertTime(time: String, tz: String) -> String {
    let parse_from_str = NaiveDateTime::parse_from_str;
    let dt;
    if time.contains("PM") || time.contains("AM") {
        dt = parse_from_str(&time, "%m/%d/%Y %I:%M:%S %p");
    } else {
        dt = parse_from_str(&time, "%m/%d/%Y %H:%M:%S");
    }
    let timezone: chrono_tz::Tz = tz.parse().unwrap();
    let mut unix = timezone.from_local_datetime(&dt.unwrap()).single();
    if unix.is_none() {
        return "".to_string();
    }
    let unix = unix.unwrap().timestamp();
    let u = Utc.timestamp(unix, 0).with_timezone(&timezone);
    return u.to_rfc3339();
}

fn getNameFromStationCode(stationCode: String) -> String {
    let fr: Vec<crate::structs::GetStationsResponse::Stations> = STATIONS_FILE
        .clone()
        .data
        .into_iter()
        .filter(|x| x.code.is_some() && x.name.is_some() && x.code.clone().unwrap() == stationCode)
        .collect();
    if fr.len() != 0 {
        if fr[0].name.is_some() {
            return fr[0].name.clone().expect("woops");
        } else {
            return stationCode;
        }
    } else {
        return stationCode;
    }
}

fn stringToUnix(string_time: String) -> i64 {
    if DateTime::parse_from_rfc3339(&string_time).is_ok() {
        return DateTime::parse_from_rfc3339(&string_time)
            .unwrap()
            .timestamp();
    } else {
        return Utc::now().timestamp();
    }
}

fn readStations() -> GetStationsResponse {
    let mut station_data = String::new();
    File::open("./static/get-stations.json")
        .unwrap()
        .read_to_string(&mut station_data)
        .unwrap();
    let stations: GetStationsResponse = serde_json::from_str(&station_data).unwrap();
    return stations;
}
