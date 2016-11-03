use riotapi::connection::APIRequest;
pub struct SummonerMetaInfo {
    id: usize,
    name: String,
    profile_icon_id: i32,
    revision_date: usize,
    summoner_level: i32,
}

impl APIRequest for SummonerMetaInfo {}