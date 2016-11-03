extern crate riot_api;
use riot_api::riotapi::connection::{APIConnection, Region, APIRequest};
use riot_api::riotapi::summoner::SummonerMetaInfo;
fn main() {
    let conn = APIConnection::new(Region::OCE,"7fd26ccd-8fa7-44f8-90eb-992704a96da2".to_string());
    let result conn.generic_api_request(SummonerMetaInfo::new());
}
