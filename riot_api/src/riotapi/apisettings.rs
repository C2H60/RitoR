use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
#[derive(Serialize, Deserialize, Debug)]
struct Endpoint {
    lookup_name: String,
    url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct EndpointCategory {
    name: String,
    major_version: u32,
    minor_version: u32,
    endpoints: Vec<Endpoint>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Endpoints {
    has_endpoints: bool,
    available_categories: Vec<EndpointCategory>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct APISettings {
    realm_info: ::riotapi::connection::APIConnection,
    endpoints: Endpoints,
}

impl APISettings {
    // pub fn load_from_file(file: &str) -> APISettings {
    // let mut s_file = try!(File::open(file));
    // let mut buffer = String::new();
    // try!(s_file.read_to_string(&mut buffer));
    // let settings: APISettings = ::serde_json::from_str(buffer.as_ref());
    // APISettings {}
    // }


    pub fn create_settings_file(file: &str) {
        let settings = APISettings {
            realm_info: ::riotapi::connection::APIConnection {
                api_key: "".to_string(),
                region: ::riotapi::connection::Region::OCE,
            },
            endpoints: Endpoints {
                has_endpoints: true,
                available_categories: vec![EndpointCategory {
                                               name: "summoner".to_string(),
                                               major_version: 1,
                                               minor_version: 3,
                                               endpoints: vec![Endpoint {
                                                                   lookup_name: "sample_endpoint"
                                                                       .to_string(),
                                                                   url: "/blaa/foobar".to_string(),
                                                               }],
                                           }],
            },
        };

        let write_file = |f_name: &str, buffer: String| -> ::std::io::Result<()> {
            let mut f = try!(File::create(f_name));
            try!(f.write_all(buffer.as_bytes()));
            Ok(())
        };

        let data = ::serde_json::to_string_pretty(&settings).unwrap();
        write_file(file, data);
    }
}