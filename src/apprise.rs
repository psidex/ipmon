use std::process::Command;

use which::which;

pub fn exists() -> bool {
    which("apprise").is_ok()
}

pub fn run_with(title: &str, body: &str, url: &str) -> Result<(), String> {
    match Command::new("apprise")
        .args(["-t", title, "-b", body, url])
        .output()
        .map_err(|e| e.to_string())?
        .status
        .success()
    {
        true => Ok(()),
        false => Err("apprise terminated with non-zero exit code".to_string()),
    }
}
