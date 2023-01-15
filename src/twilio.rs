use std::error::Error;

pub struct Client {
    sid: String,
    token: String,
    from: String,
    url: String,
}

pub fn build_client(sid: String, token: String, from: String) -> Client {
    let url = format!(
        "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
        sid
    );
    Client {
        sid,
        token,
        from,
        url,
    }
}

impl Client {
    pub fn send_text(&self, to: &str, message: &str) -> Result<(), Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let form = [("To", to), ("From", &self.from), ("Body", message)];
        client
            .post(&self.url)
            .basic_auth(&self.sid, Some(&self.token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .form(&form)
            .send()?;
        Ok(())
    }
}
