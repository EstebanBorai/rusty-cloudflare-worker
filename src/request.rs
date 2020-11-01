use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
  method: String,
  headers: HashMap<String, String>,
  url: String,
  body: Option<Box<[u8]>>,
}
