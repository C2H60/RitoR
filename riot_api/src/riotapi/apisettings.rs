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
    pub fn load_from_file(file: &str) -> APISettings {
        let f_path = Path::new(file);
        let mut file = match File::open(&f_path) {
            Err(why) => panic!("Failed to open settings.json file"),
            Ok(file) => file,
        };
        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Err(why) => panic!("Cant read file"),
            Ok(_) => println!("Settings Loaded OK"),
        };
        let settings: APISettings = ::serde_json::from_str(buffer.as_ref()).unwrap();
        settings
    }


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

    pub fn find_by_key<T>(vector: Vec<T>, key: &str) -> usize {
        vector.iter().position(|&elem| elem == key).unwrap();
    }
}