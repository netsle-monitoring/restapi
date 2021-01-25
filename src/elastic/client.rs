extern crate base64;
use crate::elastic::packet_count;
use crate::elastic::ips_data;
use crate::elastic::ports_data;
use reqwest::blocking;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

// Thanks to static lifetime elision, you *usually* won't need to explicitly use 'static
const API_URI: &str = "http://localhost:9200";

pub struct Client {
    reqwest_client: blocking::Client,
    pub token: String,
}

impl Client {
    pub fn new(username: &'static str, pw: &'static str) -> Self {
        Self {
            reqwest_client: blocking::Client::new(),
            token: base64::encode(format!("{}:{}", username, pw)),
        }
    }

    // Return the whole request for now (act as a proxy)
    pub fn match_all_for_index(&self, index: &'static str) -> Value {
        self.get_request(
            index,
            json!({
                "query": {
                    "match_all": {}
                }
            })
            .to_string(),
        )
    }

    pub fn get_packet_count_since(&self, index: &'static str, minutes: i32) -> packet_count::FinalData {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            - ((minutes * 60000) as u128);

        let result = self.get_request(
            index,
            json!({
                "query": {
                  "range": {
                    "@timestamp": {
                      "gt": since_the_epoch.to_string(),
                      "format": "epoch_millis"
                    }
                  }
                },
                "size": 10000,
                "fields": ["@timestamp", "packet_count"]
              })
            .to_string(),
        );

        let main_hits: packet_count::MainData = serde_json::from_value(result).unwrap();
        packet_count::FinalData::from(main_hits)
        // println!("{:?}", hits);
        // serde_json::to_string(&hits).unwrap()
    }

    pub fn get_ports_since(&self, index: &'static str, minutes: i32) -> ports_data::FinalData {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            - ((minutes * 60000) as u128);

        let result = self.get_request(
            index,
            json!({
                "query": {
                  "range": {
                    "@timestamp": {
                      "gt": since_the_epoch.to_string(),
                      "format": "epoch_millis"
                    }
                  }
                },
                "size": 10000,
                "fields": ["@timestamp", "ports"]
              })
            .to_string(),
        );

        let main_hits: ports_data::MainData = serde_json::from_value(result).unwrap();
        ports_data::FinalData::from(main_hits)
        // println!("{:?}", hits);
        // serde_json::to_string(&hits).unwrap()
    }

    pub fn get_usage_since(&self, index: &'static str, minutes: i32) -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            - ((minutes * 60000) as u128);

        let result = self.get_request(
            index,
            json!({
                "query": {
                  "range": {
                    "@timestamp": {
                      "gt": since_the_epoch.to_string(),
                      "format": "epoch_millis"
                    }
                  }
                },
                "size": 10000,
                "fields": ["@timestamp", "ips"]
              })
            .to_string(),
        );

        let main_hits: ips_data::MainData = serde_json::from_value(result).unwrap();
        let all_data = ips_data::FinalData::from(main_hits);

        let mut result = 0u128;
        
        for data in all_data.data {
            result += data.usage / 1024 / 1024 / 2;
            // println!("{}-", data.usage);
        } 

        result.to_string()
    }

    fn get_request(&self, index: &'static str, body: String) -> Value {
        let full_url = format!("{}/{}/_search?filter_path=hits", API_URI, index);
        let result = self
            .reqwest_client
            .get(&full_url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &format!("Basic {}", &self.token))
            .body(body)
            .send()
            .unwrap()
            .text()
            .unwrap();

        serde_json::from_str(&result).unwrap()
    }
}
