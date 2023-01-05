use std::error::Error;
use std::net::Ipv4Addr;

pub fn get_ip() -> Result<Ipv4Addr, Box<dyn Error>> {
    let get = reqwest::blocking::get("https://api.ipify.org/")?;
    let text = get.text()?;
    Ok(text.parse::<Ipv4Addr>()?)
}
