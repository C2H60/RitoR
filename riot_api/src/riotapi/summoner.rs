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
    fn make_request() -> APIRequestResult<SummonerMetaInfo> {
        let x = SummonerMetaInfo {
            id: 1,
            name: "SumCoolAid".to_string(),
            profile_icon_id: 1,
            revision_date: 1,
            summoner_level: 30,
        };
        if true {
            Ok(x)
        } else {
            Err(())
        }
    }
}