extern crate riot_api;
use riot_api::riotapi::connection::APIConnectionManager;
use riot_api::riotapi::summoner::SummonerManager;
use riot_api::riotapi::apisettings::APISettings;
fn main() {
    let connection = APIConnectionManager::new(APISettings::load_from_file("settings.json"));
    let x = SummonerManager::new(connection);
    x.get_summoner_meta_by_id("".to_string());
}
