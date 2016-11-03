extern crate riot_api;
use riot_api::riotapi::connection::{APIConnection, Region, APIRequest};
use riot_api::riotapi::summoner::SummonerMetaInfo;
fn main() {
    let conn = APIConnection::new(Region::OCE, "34ioj-rgreg-345tg".to_string());
    let summoner_meta = SummonerMetaInfo::make_request();

    println!("{:?}", summoner_meta.unwrap());
}
