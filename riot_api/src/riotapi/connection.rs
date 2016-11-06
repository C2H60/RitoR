use std;
use std::io::Read;
use hyper::Client;
use riotapi::apisettings::APISettings;
#[derive(Serialize, Deserialize, Debug)]
pub enum Region {
    BR,
    EUNE,
    EUW,
    JP,
    KR,
    LAN,
    LAS,
    NA,
    OCE,
    RU,
    TR,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct APIConnection {
    pub region: Region,
    pub api_key: String,
}

pub struct APIConnectionManager {
    settings: APISettings,
}

impl APIConnectionManager {
    /// Constructs a new APIConnection
    ///
    /// #Examples
    ///
    /// ```
    /// use riot_api::riotapi::connection::{APIConnectionManager, Region};
    ///
    /// let conn = APIConnectionManager::new(Region::OCE,"YOUR_API_KEY".to_string());
    /// ```
    pub fn new(api_settings: APISettings) -> APIConnectionManager {
        APIConnectionManager { settings: api_settings }
    }

    pub fn get(url: &str) -> ::hyper::Result<String> {
        let client = Client::new();
        let mut response = try!(client.get(url).send());
        let mut buffer = String::new();
        try!(response.read_to_string(&mut buffer));
        Ok(buffer)
    }
}

/// Implements fmt::Display for Region
///
///  #Examples
///  ```
///
///     println!("{:?}",Region::NA.to_string());
///
/// ```

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}