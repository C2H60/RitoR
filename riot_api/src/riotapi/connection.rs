use std::fmt::{Display, Result, Formatter};
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


trait APIRequest<'T> {
    fn make_request<F>(&self, callback: F) -> Result where F: Fn() -> bool;
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}