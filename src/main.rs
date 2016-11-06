extern crate riot_api;
use riot_api::riotapi::connection::{APIConnectionManager, Region};
use riot_api::riotapi::apisettings::APISettings;
fn main() {
    let connection = APIConnectionManager::new(APISettings::load_from_file("settings.json"));

}
