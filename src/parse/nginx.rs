use std::{fs};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use serde_json;

use crate::config;
use crate::modules;

pub fn parser(log_directory: &Path, base_log_file: &str, out_json_path: &Path, clear_log: bool) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = PathBuf::from(&log_directory).join(base_log_file);
    let file = fs::File::open(&log_path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        //let _start = std::time::Instant::now();
        let line = line?;
        let mut entry: modules::log_entry::LogEntryNginx = line.parse().unwrap_or_else(|err| {
            println!("Problem parsing log line; skipping: {:?}", err);
            modules::log_entry::LogEntryNginx::default()
        });
        let res = modules::ip::IpinfoResult::new(entry.ip.clone())
            .unwrap_or_else(|err| {
                println!("Problem finding country.. {:?}", err);
                modules::ip::IpinfoResult::default()
        });
        entry.set_ip_locale(res.country_name, res.continent_name);
        if config::REMOVE_IPS {
            entry.remove_ip();
        }
        entries.push(entry);
    }

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&entries).expect("Failed to serialize to JSON");

    // Write JSON to a file
    let mut file_out = fs::File::create(out_json_path).expect("Failed to create file");
    file_out.write_all(json.as_bytes()).expect("Failed to write JSON to file");

    if clear_log {
        // Clear original file contents
        let mut _file = File::create(PathBuf::from(&log_path)).expect("Failed to clear file contents");
    }

    Ok(())

}
