use crate::error;
use regex::Regex;

const URL: &str = "http://worldclockapi.com/api/json/utc/now";

pub fn current() -> String {
    let res = reqwest::blocking::get(URL).unwrap_or_else(|error| {
        panic!(
            "Error: Could not get current time from '{}', error: {}!",
            URL, error
        )
    });

    if res.status() != 200 {
        error::handle(&format!("'{}' response status is not equal 200!", URL));
    }
    let body = res.json::<serde_json::Value>().unwrap_or_else(|error| {
        panic!(
            "Error: Could not parse json from '{}', error: {}!",
            URL, error
        )
    });

    let srd_date_with_time_in_z_timezone = body["currentDateTime"].to_string();
    validate_date(&srd_date_with_time_in_z_timezone);

    srd_date_with_time_in_z_timezone
}

fn validate_date(string_with_date: &str) {
    Regex::new(r"(\d{4})-(\d{2})-(\d{2})") // e.x. 2022-03-20
        .expect("Error: Parsing date regexp is broken!")
        .captures(string_with_date)
        .unwrap_or_else(|| {
            panic!(
                "Error: '{}'could not capture date parts parsed date is not valid!",
                URL
            )
        })
        .get(0)
        .unwrap_or_else(|| panic!("Error: Error: Could not get valid date from '{}'!", URL));
}
