use std::fs;
use std::io::BufReader;
use serde_json;
pub const REMOVE_IPS: bool = true;
pub const CONCAT_AUTH_LOGS: bool = true;
pub const CONFIG_PATH: &str = "./config.json";
pub fn get_config_json(pth: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let file = fs::File::open(pth)?;
    let reader = BufReader::new(file);
    let result: serde_json::Value = serde_json::from_reader(reader)?;
    Ok(result)
}