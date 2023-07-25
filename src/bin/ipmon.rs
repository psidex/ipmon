use std::{error::Error, fs, net::Ipv4Addr, str::FromStr};

use log::info;

use ipmon::{client::apprise, client::config, platform};

const IP_CACHE_PATH: &str = "./ipmon.cache";

fn get_current_ipv4(server_url: &str) -> Result<Ipv4Addr, Box<dyn Error>> {
    let ip = match platform::is_debug() {
        true => Ipv4Addr::from_str("127.0.0.1")?,
        false => ureq::get(server_url)
            .call()?
            .into_string()?
            .parse::<Ipv4Addr>()?,
    };
    Ok(ip)
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    if platform::is_debug() {
        info!("Running in debug mode, won't use IP server or apprise")
    }

    if !apprise::exists() {
        panic!("cannot find apprise binary");
    }

    let mut config_path: String = "config.yaml".to_owned();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        config_path = args[1..].join(" ")
    }

    let cfg = config::load_config(&config_path);
    info!("Loaded config");

    let mut prev_ip = fs::read_to_string(IP_CACHE_PATH)
        .unwrap_or("127.0.0.1".to_string())
        .parse::<Ipv4Addr>()
        .unwrap_or(Ipv4Addr::new(127, 0, 0, 1));

    info!("Loaded current IP from cache: {}", prev_ip);

    info!("Starting loop");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(cfg.interval));

        let maybe_new_ip = match get_current_ipv4(&cfg.server) {
            Ok(ip) => ip,
            Err(error) => {
                info!("Error getting current IP: {}", error);
                continue;
            }
        };

        if maybe_new_ip != prev_ip {
            prev_ip = maybe_new_ip;
            let prev_ip_str = &prev_ip.to_string();
            info!("New IPv4: {}", prev_ip_str);
            fs::write(IP_CACHE_PATH, prev_ip_str)?;

            for notif_cfg in cfg.notifications.iter() {
                info!("Sending notification");

                if !platform::is_debug()
                    && !apprise::run_with(
                        &notif_cfg.title,
                        &notif_cfg.body.replace("{{ip}}", prev_ip_str),
                        &notif_cfg.url,
                    )
                {
                    // TODO: run_with should return what the err was
                    info!("Failed to run apprise");
                }
            }
        }
    }
}
