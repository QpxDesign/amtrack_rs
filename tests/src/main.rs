use amtrack_rs::GetStations;
use amtrack_rs::GetTrains;

#[tokio::main]
async fn main() {
    let t = GetTrains();
    let s = GetStations().await.unwrap();
    println!("{}", s);
}
