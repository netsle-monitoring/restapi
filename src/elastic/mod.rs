pub mod client;
pub mod packet_count;
pub mod ports_data;
pub mod ips_data;

pub struct ElasticClient(pub client::Client);
