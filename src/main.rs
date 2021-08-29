use std::io::Read;
use serde::{Deserialize};
use curl::easy::Easy;

#[derive(Debug, Deserialize)]
struct Slack {
    sites: Vec<String>,
    hook: String,
}

#[derive(Debug, Deserialize)]
struct Actions {
    slack: Slack,
}

#[derive(Debug, Deserialize)]
struct Config {
    actions: Actions,
}

fn curl(url:&str) -> u32 {
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.perform().unwrap();

    return easy.response_code().unwrap()
}

fn slack_post(message:&str, url:&str) {
    let mut data = message.as_bytes();
    let mut easy = Easy::new();

    easy.url(url).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("config.yml")?;
    let deserialized_config: Config = serde_yaml::from_reader(f)?;

    for url in deserialized_config.actions.slack.sites.iter() {
        let response_code = curl(&url);
        
        if response_code != 200 && response_code != 301 {
            let s = format!("{{\"text\":\"{} was {}\"}}", url, response_code);
            println!("response code for {} was {}", url, response_code);
            slack_post(&s, &deserialized_config.actions.slack.hook)
        }
    }

    Ok(())
}
