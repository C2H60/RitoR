use std;
use std::io::Read;
use hyper::Client;

#[derive(Debug)]
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

pub struct APIConnection {
    region: Region,
    api_key: String,
}

impl APIConnection {
    /// Constructs a new APIConnection
    ///
    /// #Examples
    ///
    /// ```
    /// use riot_api::riotapi::connection::{APIConnection, Region};
    ///
    /// let conn = APIConnection::new(Region::OCE,"YOUR_API_KEY".to_string());
    /// ```
    pub fn new(con_region: Region, con_api_key: String) -> APIConnection {
        APIConnection {
            region: con_region,
            api_key: con_api_key,
        }
    }
    pub fn get(url: &str) -> ::hyper::Result<String> {
        let client = Client::new();
        let mut response = try!(client.get(url).send());
        let mut buffer = String::new();
        try!(response.read_to_string(&mut buffer));
        Ok(buffer)
    }

    /// Gets the currently selected region as a String
    ///
    /// #Examples
    ///
    /// ```
    /// let region: String = conn.get_region();
    /// ```
    pub fn get_region(&self) -> String {
        self.region.to_string()
    }

    /// Gets the currently selected region as a String
    ///
    /// #Examples
    ///
    /// ```
    /// let api_key: String = conn.get_api_key();
    /// ```
    pub fn get_api_key(&self) -> String {
        self.api_key.to_owned()
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