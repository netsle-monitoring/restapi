
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
    pub ports: Vec<PortData>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PortData {
    pub port: i32,
    pub count: i32
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