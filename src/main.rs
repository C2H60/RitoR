extern crate riot_api;
use riot_api::riotapi::connection::{APIConnection, Region};
fn main() {
    let conn = APIConnection::new(Region::OCE, "34ioj-rgreg-345tg".to_string());
    println!("{:?}", conn.get_region());
}
