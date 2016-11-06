use riotapi::connection::APIConnectionManager;
use riotapi::apisettings::APISettings;
use std::vec::*;
pub struct SummonerMetaInfo {

}

pub struct Summoner {
    summoner_meta: SummonerMetaInfo,
}

pub struct SummonerManager {
    connection: APIConnectionManager,
}

impl SummonerManager {
    pub fn new(connection: APIConnectionManager) -> SummonerManager {
        SummonerManager { connection: connection }
    }


    pub fn get_summoner_meta_by_id(&self, id: String) {

        APISettings::find_by_key(self.connection.settings.endpoints.available_categories,
                                 "summoner");
        // println!("{:?}",self.connection.get(self.connection.settings.endpoints.available_categories.iter().position).unwrap());

    }
}