use std::error::Error;

pub struct Client {
    client: reqwest::blocking::Client,
    sid: String,
    token: String,
    from: String,
    url: String,
}

impl Client {
    pub fn new(sid: String, token: String, from: String) -> Client {
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            sid
        );
        Client {
            client,
            sid,
            token,
            from,
            url,
        }
    }

    pub fn send_text(&self, to: &str, message: &str) -> Result<(), Box<dyn Error>> {
        let form = [("To", to), ("From", &self.from), ("Body", message)];
        self.client
            .post(&self.url)
            .basic_auth(&self.sid, Some(&self.token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .form(&form)
            .send()?;
        Ok(())
    }
}
