use std::{env};
use std::path::Path;

pub mod parse;
pub mod modules;
pub mod config;
use crate::modules::file::combine_files;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example: Accessing data
    let args: Vec<String> = env::args().collect();

    // The first argument is typically the program name itself
    if args.len() > 1 {
        println!("Arguments passed: {:?}", &args[1..]);
        let config_json = config::get_config_json(config::CONFIG_PATH)?;

        // You can process each argument as needed
        for arg in args.iter().skip(1) {
            println!("Argument: {}", arg);
            if (arg == "nginx") | (arg == "n") {
                let logs_dir_nginx = Path::new(config_json["logs_dir_nginx"].as_str().unwrap());
                let out_json_path = Path::new(config_json["out_json_nginx"].as_str().unwrap());
                parse::nginx::parser(
                    logs_dir_nginx,
                    "access.log",
                    out_json_path,
                    false
                ).expect("Failed to parse nginx file");
            } else if (arg == "auth") | (arg == "a") {
                let config_auth_path = Path::new(config_json["logs_dir_auth"].as_str().unwrap());
                let out_json_path = Path::new(config_json["out_json_auth"].as_str().unwrap());
                parse::auth::parser(
                    config_auth_path,
                    "auth.log",
                    out_json_path,
                    false,
                ).expect("Failed to parse auth file");
            } else if arg == "auth-combine" {
                let config_json = config::get_config_json(config::CONFIG_PATH)?;
                let config_auth_path = Path::new(config_json["logs_dir_auth"].as_str().unwrap());
                combine_files(config_auth_path, "auth.log").expect("Combine files did not work....")
            }
        }
    } else {
        println!("No arguments supplied; use `nginx_parser nginx` or `nginx_parser auth-combine` or `nginx_parser auth`");
    }
    Ok(())
}
