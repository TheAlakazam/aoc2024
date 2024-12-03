use reqwest::blocking::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct AocClient {
    client: Client,
    base_url: String,
    session_token: String,
}

impl AocClient {
    pub fn new(base_url: &str, session_token: &str) -> Self {
        AocClient {
            client: Client::new(),
            base_url: base_url.to_string(),
            session_token: session_token.to_string().trim().to_string(),
        }
    }

    pub fn download_input(
        &self,
        year: u32,
        day: u32,
        output_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/{}/day/{}/input", self.base_url, year, day);
        let response = self
            .client
            .get(&url)
            .header("Cookie", format!("session={}", self.session_token))
            .send()?
            .text()?;

        let mut file = File::create(output_path)?;
        file.write_all(response.as_bytes())?;
        Ok(())
    }

    pub fn upload_solution(
        &self,
        year: u32,
        day: u32,
        level: u32,
        answer: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/{}/day/{}/answer", self.base_url, year, day);
        let response = self
            .client
            .post(&url)
            .header("Cookie", format!("session={}", self.session_token))
            .form(&[("level", level.to_string()), ("answer", answer.to_string())])
            .send()?
            .text()?;

        Ok(response)
    }
}
