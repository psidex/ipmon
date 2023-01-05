use std::error::Error;
use std::net::Ipv4Addr;

mod ip;
mod stringcache;

const IP_CACHE_PATH: &str = "./ipmon.cache";
const WAIT_TIME_SECONDS: u64 = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let prev_ip_str = stringcache::get_str(IP_CACHE_PATH).unwrap_or("0.0.0.0".to_string());
    let mut prev_ip = prev_ip_str.parse::<Ipv4Addr>()?;
    println!("Loaded {} from cache", prev_ip);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(WAIT_TIME_SECONDS));

        let addr = match ip::get_ip() {
            Ok(a) => a,
            Err(error) => {
                println!("Error getting IPv4: {}", error);
                continue;
            }
        };

        if addr != prev_ip {
            println!("Got new IPv4 {:?}", addr);
            prev_ip = addr;
            stringcache::set_str(IP_CACHE_PATH, &addr.to_string())?;
        } else {
            println!("Got old IPv4 {:?}", addr);
        }
    }
}
