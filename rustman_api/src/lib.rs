use reqwest::Error;
use serde_json::Value;

pub fn fetch_data(url: &str) -> Result<String, Error> {
    let body = reqwest::blocking::get(url)?.text()?;

    return Ok(body);
}

pub fn parse_word_from_json(json_str: &str) -> Option<String> {
    let json_value: Result<Value, _> = serde_json::from_str(json_str);

    match json_value {
        Ok(value) => {
            if let Some(word) = value.as_array()?.get(0)?.as_str() {
                Some(word.to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
