# nginx_auth_parser

Nginx access.log and sshd auth.log parser written in Rust. Does a few key things
- Combines auth.log (and auth.log.1, auth.log.2.gz, ...) into a single file (i.e. auth.log)
- Leverages country geo ip info, i.e. [https://ipinfo.io/](https://ipinfo.io/) using mmdb in order to assign countries to list of IPs (optionally removing IPs) - Bring your own "countries.mmdb" file..
- Parses auth.log (sshd) and/or access.log (nginx) into a json file, which can be more easily analyzed

# Requirements
- Rust compiler ([Install Rust](https://www.rust-lang.org/tools/install))
- countries.mmdb, which you can download from [https://ipinfo.io/](https://ipinfo.io/) for free if you sign up for a developer account; (see update_db.sh). Store in the ./data/country.mmdb folder
- access.log files stored in ./data/nginx, or auth.log file(s) stored in ./data/auth

# Usage
- compile the source using `cargo build --release`
- (for nginx) run the code using `./target/release/nginx_auth_parser nginx`
- (for auth) run the code, optionally, first by using `./target/release/nginx_auth_parser auth-combine` (optional if you want to combine auth.log.x.gz files into auth.log)
- (for auth) run the code using `./target/release/nginx_auth_parser auth`

These will create file(s) ./data/auth/auth_log_entries.json and/or ./data/nginx/nginx_log_entries.json
which can be further analyzed in other software (e.g. python) to estimate ssh attempts, track down failed attempts, figure out how often folks from each country are visiting your site, etc.

