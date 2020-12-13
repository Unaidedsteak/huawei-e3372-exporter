use anyhow::{anyhow, Result};
use log::debug;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::header::COOKIE;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Session {
    host: String,
    token: Option<String>,
    timeout_secs: u64,
}

impl Session {
    pub fn new(host: String, timeout_secs: u64) -> Session {
        Session {
            host,
            token: None,
            timeout_secs,
        }
    }

    pub async fn refresh_session_token(&mut self) -> Result<()> {
        debug!("refreshing huawei api session token");
        let url = format!("http://{}/api/webserver/SesTokInfo", self.host);
        let url_parsed = reqwest::Url::parse(&url)?;
        let client = reqwest::Client::new();

        let response = client
            .get(url_parsed)
            .timeout(Duration::from_secs(self.timeout_secs))
            .send()
            .await?
            .text()
            .await?;
        self.token = Some(get_value_from_tag(&response, "SesInfo").await?);
        debug!("session token refreshed, token: {:?}", self.token);
        Ok(())
    }
}

pub async fn generic_request(session: &Session, url: &str) -> Result<String> {
    debug!("querying huawei api path {}", url);
    let url = format!("http://{}/api{}", session.host, url);
    let url_parsed = reqwest::Url::parse(&url)?;

    match &session.token {
        Some(token) => {
            let client = reqwest::Client::new();
            let res = client
                .get(url_parsed)
                .header(COOKIE, token)
                .timeout(Duration::from_secs(session.timeout_secs))
                .send()
                .await?
                .text()
                .await?;
            debug!("generic request response: {:?}", res);
            Ok(res)
        }
        None => return Err(anyhow!("Missing session token")),
    }
}

pub async fn get_value_from_tag(xml: &str, tag: &str) -> Result<String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == tag.as_bytes() => {
                txt.push(
                    reader
                        .read_text(tag.as_bytes(), &mut Vec::new())
                        .expect("Cannot decode text value"),
                );
                return Ok(txt[0].clone());
            }
            Ok(Event::Eof) => return Err(anyhow!("expected tag {} not found", tag)), // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        buf.clear();
    }
}
