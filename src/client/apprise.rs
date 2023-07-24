use std::process::Command;

use which::which;

pub fn exists() -> bool {
    which("apprise").is_ok()
}

pub fn run_with(title: &str, body: &str, url: &str) -> bool {
    match Command::new("apprise")
        .args(["-t", title, "-b", body, url])
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
