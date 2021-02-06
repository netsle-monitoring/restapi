
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
    pub ips: Vec<HostData>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct HostData {
    pub count: i32,
    pub ip: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalData {
    pub data: Vec<FieldsData>
}

impl From<MainData> for FinalData {
    fn from(hits: MainData) -> FinalData {
        let mut data = Vec::<FieldsData>::new();
        
        for hit in hits.hits.hits {
            data.push(
                hit.source
            )
        }

        FinalData {
            data
        }
    }
}