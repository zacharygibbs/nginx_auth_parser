use std::str::FromStr;
use std::default::Default;
use serde::{Deserialize, Serialize};
use std::error::Error;
use regex::{Regex};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct LogEntryNginx {
    pub ip: String,
    pub user: String,
    pub timestamp: DateTime<Utc>,
    pub request: String,
    pub response: String,
    pub body_bytes_sent: String,
    pub http_referer: String,
    pub http_user_agent: String,
    pub country_name: String,
    pub continent_name: String
}

impl FromStr for LogEntryNginx {
    //type Err = String;
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //ip & user
        let tmp_vec = s.split(" ").collect::<Vec<&str>>();
        let ip = (&tmp_vec.first().unwrap()).to_string();
        let user = (&tmp_vec.get(2).unwrap()).to_string();
        let binding = tmp_vec.get(3..).unwrap().join(" ");
        let tmp_everything_else = binding.as_str().trim_start_matches("[");

        //time
        let mut splts = tmp_everything_else.split("]");
        let timestamp_str = splts.next().unwrap().to_string();
        let timestamp: DateTime<Utc> = DateTime::parse_from_str(
            &timestamp_str,
            "%d/%b/%Y:%H:%M:%S %z"
        ).unwrap_or_else(|err| {
            println!("Could not parse timestamp_str {}, {:?}, using default", timestamp_str, err);
            DateTime::default()
        }).into();
        let tmp_everything_else = tmp_everything_else.trim_start_matches(&timestamp_str)
            .trim_start_matches(r#"] ""#)
            .trim_start();

        // request
        let mut splts = tmp_everything_else.split(r#"""#);
        let request = splts.next().unwrap().to_string();
        let tmp_everything_else = tmp_everything_else
            .trim_start_matches(format!("{}{} ", &request, r#"""#).as_str());

        // response, body_bytes_sent
        let mut splts = tmp_everything_else.split(" ");
        let response = splts.next().unwrap().to_string();
        let body_bytes_sent = splts.next().unwrap().to_string();
        let tmp_everything_else = tmp_everything_else
            .trim_start_matches(format!("{} {} {}", response, body_bytes_sent, r#"""#).as_str());

        //http_referer
        let mut splts = tmp_everything_else.split(r#"""#);
        let http_referer = splts.next().unwrap().to_string();
        let tmp_everything_else = tmp_everything_else
            .trim_start_matches(format!("{}{}", http_referer, r#"" ""#).as_str());

        //http_user_agent
        let http_user_agent = tmp_everything_else.trim_end_matches(r#"""#).to_string();
        //println!("{ip}, {user}, {timestamp}, {request}, {response}, {body_bytes_sent}, {http_referer}, {http_user_agent}");
        Ok(LogEntryNginx {
            ip,
            user,
            timestamp,
            request,
            response,
            body_bytes_sent,
            http_referer,
            http_user_agent,
            country_name: String::from(""),
            continent_name: String::from(""),
        })
    }
}

impl LogEntryNginx {
    pub fn set_ip_locale(&mut self, country_name: String, continent_name: String){
        self.country_name = country_name;
        self.continent_name = continent_name;
    }

    pub fn remove_ip(&mut self){
        self.ip = String::from("");
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct LogEntryAuth {
    pub ip: String,
    pub user: String,
    pub timestamp: DateTime<Utc>,
    pub request_type: String,
    pub auth_type: String,
    pub success: bool,
    pub country_name: String,
    pub continent_name: String,
    pub port: String,
}

impl FromStr for LogEntryAuth {
    //type Err = String;
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let start = std::time::Instant::now();
        let splts = s.split_whitespace();
        let splts = splts.collect::<Vec<&str>>();
        let timestamp_str = splts.first().unwrap();
        let timestamp: DateTime<Utc> = DateTime::parse_from_str(
            timestamp_str,
            "%Y-%m-%dT%H:%M:%S%.f%z"
        ).unwrap_or_else(|err| {
            println!("Could not parse timestamp_str {}, {:?}, using default", timestamp_str, err);
            DateTime::default()
        }).into();
        let _host = splts.get(1).unwrap();
        let _proc = splts.get(2).unwrap();
        let mut everything_else = splts[3..].join(" ").to_string();


        // Do some string pre cleaning..
        //println!("{}", everything_else);
        if everything_else.starts_with("message repeated"){// message repeated \d+ times: [.. the message...]
            let tmp_everything_else = everything_else.split_whitespace();
            let tmp_everything_else = tmp_everything_else.collect::<Vec<&str>>();
            everything_else = tmp_everything_else
                .get(4..).unwrap()
                .join(" ")
                .trim_start_matches("[ ")
                .trim_end_matches("]")
                .parse() // convert to String..
                .unwrap();
        }


        // Now let's go and parse everything_else now conditionally...
        let mut success: bool = false;
        let mut auth_type: String = "".to_string();
        let mut user: String = "".to_string();
        let mut ip: String = "".to_string();
        let mut port: String = "".to_string();
        let mut request_type: String = "".to_string();
        let mut _proc: String = "".to_string();

        if everything_else.starts_with("Accepted ") {
            success = true;
            let splts = everything_else.split_whitespace();
            let splts = splts.collect::<Vec<&str>>();
            auth_type = splts.get(1).unwrap().to_string();
            user = splts.get(3).unwrap().to_string();
            ip = splts.get(5).unwrap().to_string();
            port = splts.get(7).unwrap().to_string();
            request_type = splts.get(8).unwrap().to_string();
            //let regex_str2 = Regex::new(r#"^Accepted (?<auth_type>publickey|password) for (?<user>[A-z0-9]+) from (?<ip>\d{1,3}\.\d{1,3}\.\d{1,3}.\d{1,3}) port (?<port>\d{1,5}) (?<request_type>[A-z0-9]+).*$"#).unwrap();
            //captures2 = regex_str2.captures(&everything_else);
        } else if everything_else.starts_with("Failed password for ") | everything_else[1..].starts_with("nvalid user") {//Regex::new(r#"^(?:Failed password for |[Ii]nvalid user).*"#).unwrap().is_match(&everything_else) {
            success = false;
            auth_type = "password".to_string();
            let tmp_everything_else = everything_else.trim_start_matches("Failed password for ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("Invalid user ").trim_start();
            let tmp_everything_else = tmp_everything_else.trim_start_matches("invalid user ").trim_start();
            let splts = tmp_everything_else.split_whitespace();
            let splts = splts.collect::<Vec<&str>>();
            let tmp_user = splts.first().unwrap();
            let add_index = match *tmp_user { // if user was " " then first word won't be user name..
                "from" => 1,
                _ => 0,
            };
            user = match *tmp_user {
                "from" => " ".to_string(),
                ess => ess.to_string()
            };
            ip = splts.get(2 - add_index).unwrap().to_string();
            port = splts.get(4 - add_index).unwrap().to_string();
            _proc = splts.get(5 - add_index).unwrap_or(&_proc.as_str()).to_string();// ok if this isn't present..
        } else if
            (everything_else.starts_with("pam_unix(sshd:auth): authentication failure"))
                | (
                    (everything_else.starts_with("PAM "))
                    & (!everything_else.contains("ignoring max retries;"))
                )
        {
            //2025-01-12T00:09:54.095845+00:00 vultr sshd[515000]: pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=0.0.0.0
            success = false;
            auth_type = "password".to_string();
            let tmp_everything_else = everything_else.trim_start_matches("pam_unix(sshd:auth): authentication failure; ");//logname= uid=");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("PAM ");
            let splts = tmp_everything_else.split(" ");
            let splts = splts.collect::<Vec<&str>>(); //to get rid of the number in "PAM 1 more"...
            let tmp_everything_else = splts.get(1..).unwrap().join(" ");
            //let tmp_everything_else = everything_else.trim_start_matches("pam_unix(sshd:auth): check pass; user unknown");//logname= uid=");
            let splts = tmp_everything_else.split("=");
            let splts = splts.collect::<Vec<&str>>();
            user = splts.get(1).unwrap()
                .split(" ").next().unwrap().to_string();
            ip = splts.get(5).unwrap()
                .split(" ").next().unwrap().to_string();
            if ip.len() == 0 {
                ip = splts.get(6).unwrap()
                    .split(" ").next().unwrap().to_string();
            }
        } else if everything_else.starts_with("Received disconnect") {//else if Regex::new(r#"^(?:Received disconnect|Disconnected)(?: from )(?:authenticating|[Ii]nvalid)?(?: user *)?.*"#).unwrap().is_match(&everything_else) {
            //r#"^(?:Received disconnect|Disconnected)(?: from )(?:authenticating|[Ii]nvalid)?(?: user *)? ?(?<user>[A-z0-9]+)? ?(?<ip>[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}).*$"#).unwrap();
            success = false;
            auth_type = "preauth".to_string();
            request_type = "disconnect".to_string();
            let tmp_everything_else = everything_else.trim_start_matches("Received disconnect from ");
            let len_before = tmp_everything_else.len();
            let tmp_everything_else = tmp_everything_else.trim_start_matches("authenticating user ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("Invalid user ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("invalid user ").trim_start();
            let len_after = tmp_everything_else.len();
            let splts = tmp_everything_else.split(" ");
            let splts = splts.collect::<Vec<&str>>();
            if len_after == len_before { // as in, if there's no user...
                ip = splts.first().unwrap().to_string();
            } else {
                user = splts.first().unwrap().to_string();
                ip = splts.get(1).unwrap().to_string();
            }
        } else if (everything_else.starts_with("Disconnected from"))
            | (everything_else.starts_with("Disconnecting ")) {
            //r#"^(?:Received disconnect|Disconnected)(?: from )(?:authenticating|[Ii]nvalid)?(?: user *)? ?(?<user>[A-z0-9]+)? ?(?<ip>[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}).*$"#).unwrap();
            success = false;
            auth_type = "preauth".to_string();
            request_type = "disconnect".to_string();
            let tmp_everything_else = everything_else.trim_start_matches("Disconnected from ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("Disconnecting ");
            let len_before = tmp_everything_else.len();
            let tmp_everything_else = tmp_everything_else.trim_start_matches("authenticating user ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("user ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("Invalid user ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("invalid user ").trim_start();
            let len_after = tmp_everything_else.len();
            let splts = tmp_everything_else.split(" ");
            let splts = splts.collect::<Vec<&str>>();
            if len_after == len_before { // as in, if there's no user...
                ip = splts.first().unwrap().to_string();
            } else if !Regex::new(r#"^(?:[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}).*"#)
                .unwrap().is_match(tmp_everything_else) {
                //users dont usually start with numbers..
                user = splts.first().unwrap().to_string();
                ip = splts.get(1).unwrap().to_string();
                port = splts.get(3).unwrap_or(&"").to_string();
            } else {
                // if it does start with number, assume its the ip
                ip = splts.first().unwrap().to_string();
                port = splts.get(2).unwrap_or(&"").to_string();
            }
        } else if (everything_else.starts_with("Connection ")) | (everything_else.starts_with("Connection reset")) {
            //r#"^Connection (?:closed|reset) ?by (?:authenticating|invalid)?(?: user )?(?<user>[A-z0-9]+)? ?(?<ip>[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}) port (?<port>\d{1,5}).*$"#
            success = false;
            auth_type = "preauth".to_string();
            request_type = "disconnect".to_string();
            let tmp_everything_else = everything_else.trim_start_matches("Connection closed by ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("Connection reset by ");
            let len_before = tmp_everything_else.len();
            let tmp_everything_else = tmp_everything_else.trim_start_matches("authenticating user ");
            let tmp_everything_else = tmp_everything_else.trim_start_matches("invalid user ");//.trim_start();
            let len_after = tmp_everything_else.len();
            let splts = tmp_everything_else.split(" ");
            let splts = splts.collect::<Vec<&str>>();
            if len_after == len_before { // as in, if there's no user...
                ip = splts.first().unwrap().to_string();
            } else if !Regex::new(r#"^(?:[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}).*"#)
                        .unwrap().is_match(tmp_everything_else) {
                //users dont usually start with numbers..
                user = splts.first().unwrap().to_string();
                ip = splts.get(1).unwrap().to_string();
                port = splts.get(3).unwrap_or(&"").to_string();
            } else {
                    // if it does start with number, assume its the ip
                ip = splts.first().unwrap().to_string();
                port = splts.get(2).unwrap_or(&"").to_string();
            }
        } else if everything_else.starts_with("Unable to negotiate with"){
            //r#"^Unable to negotiate with (?<ip>[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}) port (?<port>\d{1,5}): no matching key exchange method.*"#).unwrap();
            success = false;
            request_type = "key".to_string();

            let tmp_everything_else = everything_else.trim_start_matches("Unable to negotiate with ");
            let splts = tmp_everything_else.split(" ");
            let splts = splts.collect::<Vec<&str>>();

            ip = splts.first().unwrap().to_string();
            port = splts.get(2).unwrap().to_string();
        } else if everything_else.starts_with("banner exchange:") {
            //banner exchange: Connection from 0.0.0.0 port 65125: invalid format
            success = false;
            auth_type = "banner".to_string();
            let tmp_everything_else = everything_else.trim_start_matches("banner exchange: Connection from ");
            let splts = tmp_everything_else.split(" ");
            let splts = splts.collect::<Vec<&str>>();
            ip = splts.first().unwrap().to_string();
            port = splts.get(2).unwrap().to_string();
        } else if everything_else.starts_with("Failed none for invalid user ") {
            //Failed none for invalid user admin from 0.0.0.0 port 47718 ssh2
            let tmp_everything_else = everything_else.trim_start_matches("Failed none for invalid user ").trim_start();
            let tmp_everything_else = tmp_everything_else.trim_start_matches("from ");
            let splts = tmp_everything_else.split(" ");
            let splts = splts.collect::<Vec<&str>>();

            if !Regex::new(r#"^(?:[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}).*"#)
                .unwrap().is_match(tmp_everything_else) {
                user = splts.first().unwrap().to_string(); // if first isn't ip, its a user name
                ip = splts.get(2).unwrap().to_string();
                port = splts.get(4).unwrap().to_string();
            } else {
                ip = splts.first().unwrap().to_string();
                port = splts.get(2).unwrap().to_string();
            }

        } else {
            // skip the catch all for now to try to capture other info..
            drop(ip);//just to avoid unused assignment warning..
            return Err("Couldn't process line".into());
        }
        //println!("{}, {}, {}, {}, {}, {}, {}, {}, {}", timestamp, host, _proc, ip, user, request_type, port, success, auth_type);
        //let duration = start.elapsed();
        Ok(
            LogEntryAuth {
                ip,
                user,
                timestamp,
                request_type,
                auth_type,
                success,
                country_name: "".to_string(),
                continent_name: "".to_string(),
                port,
            }
        )
    }
}

impl LogEntryAuth {
    pub fn set_ip_locale(&mut self, country_name: String, continent_name: String){
        self.country_name = country_name;
        self.continent_name = continent_name;
    }

    pub fn remove_ip(&mut self){
        self.ip = String::from("");
    }
}
