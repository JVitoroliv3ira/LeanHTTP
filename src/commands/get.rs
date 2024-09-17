use clap::ArgMatches;
use reqwest::blocking::Client;

use crate::http_response::HttpResponse;
use std::error::Error;
use std::time::Instant;

pub fn handle_get(matches: &ArgMatches) {
    if let Some(url) = matches.get_one::<String>("url") {
        if let Err(e) = url::Url::parse(url) {
            eprintln!("Invalid URL: {}", e);
            return;
        }

        match perform_get_request(url.as_str()) {
            Ok(res) => {
                println!("{:?} {} (Time: {} ms)", res.version, res.status, res.duration);

                for (key, value) in res.headers.iter() {
                    println!("{}: {}", key, value.to_str().unwrap_or("<invalid header>"));
                }

                println!("{}", res.body);
            }
            Err(e) => eprintln!("Error occurred: {}", e),
        }
    }
}

fn perform_get_request(url: &str) -> Result<HttpResponse, Box<dyn Error>> {
    let client = Client::new();

    let start_time = Instant::now();

    let response = client.get(url).send()?;

    let duration = start_time.elapsed().as_millis();

    let http_response = HttpResponse {
        status: response.status(),
        version: response.version(),
        headers: response.headers().clone(),
        body: response.text()?,
        duration,
    };

    Ok(http_response)
}