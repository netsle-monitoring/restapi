use crate::elastic;
use crate::guards::ApiKey;
use rocket::response::content;
use rocket::State;
// use serde_json::Value;
use elastic::ports_data::PortData;
use std::collections::HashMap;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, netsle!"
}

#[get("/dashboard/packet_count_graph")]
pub fn dashboard_packet_count_graph(
    _access: ApiKey,
    elastic: State<elastic::ElasticClient>,
) -> content::Json<String> {
    let response = elastic.0.get_packet_count_since("netsle", 15);
    let response = serde_json::to_string(&response)
        .unwrap_or_else(|_| panic!("Error parsing packet count payload"));
    content::Json(response)
}

#[get("/dashboard/total_packets")]
pub fn dashboard_total_packets(
    _access: ApiKey,
    elastic: State<elastic::ElasticClient>,
) -> content::Json<String> {
    let response = elastic.0.get_packet_count_since("netsle", 10080);
    let mut count = 0;

    for i in response.data {
        count += i.count
    }

    content::Json(json!({
        "count": count
    }).to_string())
}

#[get("/dashboard/ports_data")]
pub fn dashboard_ports_data(
    _access: ApiKey,
    elastic: State<elastic::ElasticClient>,
) -> content::Json<String> {
    let response = elastic.0.get_ports_since("netsle", 10080);
    let mut ports = HashMap::<u16, u32>::new();

    for hit in response.data {
        for port in hit.ports {
            *ports.entry(port.port as u16).or_insert(0) += port.count as u32;
        }
    }
    content::Json(json!({
        // "ports": count
    }).to_string())
}
