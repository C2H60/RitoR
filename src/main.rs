extern crate riot_api;
use riot_api::riotapi::connection::{APIConnection, Region};
use riot_api::riotapi::apisettings::APISettings;
fn main() {
    println!("{:?}",
             APIConnection::get("https://oce.api.pvp.net/api/lol/oce/v1.\
                                 4/summoner/by-name/SumCoolAid?api_key=16e553c8-70cf-44cf-95b4-81c6c2269bee"));

    let settings = APISettings::load_from_file("settings.json");
    // println!("{:?}", settings);

}
