use std::{env, fs};
use std::{error::Error, net::Ipv4Addr};

mod twilio;

const IP_CACHE_PATH: &str = "./ipmon.cache";
const SLEEP_TIME_SECONDS: u64 = 60;

pub fn get_current_ipv4() -> Result<Ipv4Addr, Box<dyn Error>> {
    let get = reqwest::blocking::get("https://api.ipify.org/")?;
    Ok(get.text()?.parse::<Ipv4Addr>()?)
}

fn process(
    client: &twilio::Client,
    to: &String,
    prev_ip: Ipv4Addr,
) -> Result<Ipv4Addr, Box<dyn Error>> {
    let addr = get_current_ipv4()?;
    if addr != prev_ip {
        println!("New IPv4 {}", addr);
        client.send_text(to, &format!("Can I have a new IP please, {}", addr))?;
        fs::write(IP_CACHE_PATH, &addr.to_string())?;
    }
    Ok(addr)
}

fn main() -> Result<(), Box<dyn Error>> {
    let prev_ip_str = fs::read_to_string(IP_CACHE_PATH).unwrap_or("127.0.0.1".to_string());
    let mut prev_ip = prev_ip_str
        .parse::<Ipv4Addr>()
        .unwrap_or(Ipv4Addr::new(127, 0, 0, 1));
    println!("Loaded {} from cache", prev_ip);

    let twilio_sid = env::var("TWILIO_SID")?;
    let twilio_token = env::var("TWILIO_TOKEN")?;
    let twilio_from = env::var("IPMON_TWILIO_FROM")?;
    let client = &twilio::build_client(twilio_sid, twilio_token, twilio_from);

    let to = &env::var("IPMON_TO")?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(SLEEP_TIME_SECONDS));
        prev_ip = match process(client, to, prev_ip) {
            Ok(maybe_new_ip) => maybe_new_ip,
            Err(error) => {
                println!("Error getting current ip: {}", error);
                continue;
            }
        };
    }
}
