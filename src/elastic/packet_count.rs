
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
    pub fields: FieldsData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldsData {
    #[serde(rename(deserialize = "@timestamp"))]
    pub timestamp: Vec<String>,
    pub packet_count: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalData {
    data: Vec<PacketData>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacketData {
    pub timestamp: String,
    pub count: i32
}

impl From<MainData> for FinalData {
    fn from(hits: MainData) -> FinalData {
        let mut data = Vec::<PacketData>::new();
        
        for hit in hits.hits.hits {
            data.push(
                PacketData {
                    count: hit.fields.packet_count[0],
                    timestamp: String::from(&hit.fields.timestamp[0])
                }
            )
        }

        FinalData {
            data
        }
        // ()
    }
}