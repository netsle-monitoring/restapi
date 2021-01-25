
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MainData {
    pub hits: Hits,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits {
    pub hits: Vec<NestedHits>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NestedHits {
    #[serde(rename(deserialize = "_source"))]
    pub source: FieldsData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldsData {
    #[serde(rename(deserialize = "@timestamp"))]
    pub timestamp: String,
    pub ips: Vec<IpData>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpData {
    pub count: i32,
    pub usage: u128,
    pub ip: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalData {
    pub data: Vec<IpData>
}

impl From<MainData> for FinalData {
    fn from(hits: MainData) -> FinalData {
        let mut data = Vec::<IpData>::new();
        
        for hits in hits.hits.hits {
            for ip_data in hits.source.ips {
                data.push(ip_data);
            }
        }

        FinalData {
            data
        }
        // ()
    }
}