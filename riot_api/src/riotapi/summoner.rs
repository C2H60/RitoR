
use riotapi::connection::APIRequest;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SummonerMetaInfo {
    id: usize,
    name: String,
    profile_icon_id: i32,
    revision_date: usize,
    summoner_level: i32,
}

impl APIRequest for SummonerMetaInfo {
    fn make_request<SummonerMetaInfo, F>(&self, callback: F) -> Result
        where F: Fn(SummonerMetaInfo) -> Result<SummonerMetaInfo, &'static str>
    {
        callback();
    }
}