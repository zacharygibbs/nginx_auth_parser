use std::default::Default;
use std::error::Error;
use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use maxminddb::{Reader};//, MaxMindDBError};

use std::sync::LazyLock;
use crate::config;

static COUNTRY_READER: LazyLock<Reader<Vec<u8>>> = LazyLock::new(|| {
    // Allows this to be read once, and re-used; not re-loaded every time resulting in a huge speed up
    let mmdb_path = config::get_config_json(config::CONFIG_PATH).unwrap();
    let mmdb_path2 = mmdb_path["country_mmdb_file"].as_str().unwrap();
    Reader::open_readfile(&mmdb_path2).unwrap_or_else(|_| panic!("Could not find country.mmdb file at path: {}", mmdb_path2))
});

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct IpinfoCountry<'a> {
    pub country: Option<&'a str>,
    pub country_name: Option<&'a str>,
    pub continent: Option<&'a str>,
    pub continent_name: Option<&'a str>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct IpinfoResult {
    pub country: String,
    pub country_name: String,
    pub continent: String,
    pub continent_name: String,
    pub ip: String,
}

impl IpinfoResult {
    pub fn new(ip: String) -> Result<Self, Box<dyn Error>> {
        let ip_address: IpAddr = ip.parse()?;
        let _ = &*COUNTRY_READER;
        let r: IpinfoCountry= COUNTRY_READER.lookup(ip_address)?;
        Ok(
            IpinfoResult{
                country: r.country.unwrap().to_string(),
                country_name: r.country_name.unwrap().to_string(),
                continent: r.continent.unwrap().to_string(),
                continent_name: r.continent_name.unwrap().to_string(),
                ip,
            }
        )

    }
}
