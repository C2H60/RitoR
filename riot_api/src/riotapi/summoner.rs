use riotapi::connection::{APIRequest, APIRequestResult};
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SummonerMetaInfo {
    id: usize,
    name: String,
    profile_icon_id: i32,
    revision_date: usize,
    summoner_level: i32,
}

impl APIRequest<SummonerMetaInfo> for SummonerMetaInfo {
    fn make_request(&self) -> APIRequestResult<SummonerMetaInfo> {
        if true {
            Ok(SummonerMetaInfo {
                id: 1,
                name: "SumCoolAid".to_string(),
                profile_icon_id: 1,
                revision_date: 1,
                summoner_level: 30,
            })
        } else {
            Err(())
        }
    }
}

impl SummonerMetaInfo {
    pub fn new() -> SummonerMetaInfo {}
}