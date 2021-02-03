
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
    pub packet_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalData {
    pub data: Vec<PacketData>
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
                    count: hit.source.packet_count / 2, // The BPF program records both counts of source and dest.
                    timestamp: String::from(&hit.source.timestamp)
                }
            )
        }

        FinalData {
            data
        }
        // ()
    }
}