extern crate base64;
use reqwest::blocking;
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
use serde_json::{json};

// Thanks to static lifetime elision, you *usually* won't need to explicitly use 'static
const API_URI: &str = "http://localhost:9200";

pub struct Client {
    reqwest_client: blocking::Client,
    pub token : String
}

impl Client {
    pub fn new(username : &'static str, pw : &'static str) -> Self {
        Self { 
            reqwest_client: blocking::Client::new(),
            token: base64::encode(format!("{}:{}", username, pw)),
        }
    }

    // Return the whole request for now (act as a proxy)
    pub fn match_all_for_index(&self, index : &'static str) -> String {
        let full_url = format!("{}/{}/_search", index, API_URI);
        let result = self.reqwest_client
            .get(&full_url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &format!("Basic {}", &self.token))
            .body(json!({
                "query": {
                    "match_all": {}
                }
            }).to_string())
            .send()
            .unwrap()
            .text()
            .unwrap();

        result
    }
}