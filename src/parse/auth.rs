use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use regex::{Regex};
use serde_json;

use crate::config;
use crate::modules;

pub fn parser(log_directory: &Path, base_log_file: &str, out_json_path: &Path, clear_log: bool) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = PathBuf::from(&log_directory).join(base_log_file);
    let file = File::open(&log_path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    // create regex prior to loop so it doesn't need to be done each time..
    let regex_has_ip = Regex::new(r#".*(?:[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}).*"#).unwrap();

    for line in reader.lines() {
        //let start = std::time::Instant::now();
        let line = line?;
        if line.contains("sshd"){//captures.is_match(&line) {
            match modules::log_entry::LogEntryAuth::from_str(&line) {
                Ok(mut entry1) => {
                    let res = modules::ip::IpinfoResult::new(
                        entry1.ip.clone()
                    ).unwrap_or_else(|error| {
                            println!("Problem finding country.. {:?}, ip:{}, line:{}", error, entry1.ip, &line);
                            modules::ip::IpinfoResult::default()
                    });
                    entry1.set_ip_locale(res.country_name, res.continent_name);
                    if config::REMOVE_IPS {
                        entry1.remove_ip();
                    }
                    entries.push(entry1)

                },
                Err(error) => {
                    if regex_has_ip.is_match(&line) {
                        println!("Problem parsing (seemingly valid) log line; skipping: {:?}, {}", error, &line);
                    }
                }
            };

        }
    }

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&entries).expect("Failed to serialize to JSON");

    // Write JSON to a file
    let mut file_out = File::create(out_json_path).expect("Failed to create file");
    file_out.write_all(json.as_bytes()).expect("Failed to write JSON to file");

    if clear_log {
        // Clear original file contents
        let mut _file = File::create(PathBuf::from(&log_path)).expect("Failed to clear file contents");
    }

    Ok(())

}
