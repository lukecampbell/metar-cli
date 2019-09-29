extern crate reqwest;
extern crate csv;

use reqwest::header::CONTENT_TYPE;

const URL: &str = "https://www.aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=csv&stationString=KDEN&hoursBeforeNow=2";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metar_data = get_metar_data()?;
    let mut rdr = csv::Reader::from_reader(metar_data.as_bytes());
    let headers = rdr.headers()?.clone();
    let raw_text_idx: usize = headers.iter().position(|r| r == "raw_text").unwrap();
    for result in rdr.records() {
        let record = result?;
        println!("{}", &record[raw_text_idx]);
    }
    Ok(())
}

fn get_metar_data() -> Result<String, Box<dyn std::error::Error>> {
    let mut response: reqwest::Response = reqwest::get(URL)?;
    if !response.status().is_success() {
        return Err(From::from(response_message(&response.status())));
    }
    let content_type = response.headers().get(CONTENT_TYPE).unwrap();
    assert!(content_type == "application/x-csv", "Content-type is not application/x-csv");
    let body = response.text()?;
    let lines: Vec<&str> = body.split('\n').collect();
    assert!(lines.len() > 5, "Invalid response");
    Ok(lines[5..].join("\n"))
}

fn response_message(status: &reqwest::StatusCode) -> String {
    match status.canonical_reason() {
        Some(reason) => format!("Bad HTTP Status Code - HTTP {}: {}", status.as_str(), reason),
        None => format!("Bad HTTP Status Code - HTTP {}: {}", status.as_str(), "Unknown")
    }
}
