use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Response {
  status: u16,
  headers: HashMap<String, String>,
  body: Option<String>
}

impl Response {
  pub fn new(status: u16, headers: HashMap<String, String>, body: String) -> Self {
    let mut headers: HashMap<String, String> = HashMap::new();

    headers.insert(String::from("Content-Type"), String::from("text/plain"));

    Self {
      status,
      headers,
      body: Some(String::default()),
    }
  }

  pub fn from_str(body: &str) -> Self {
    let mut headers: HashMap<String, String> = HashMap::new();

    headers.insert(String::from("Content-Type"), String::from("text/plain"));

    Self {
      status: 200 as u16,
      headers,
      body: Some(body.to_string()),
    }
  }
}
