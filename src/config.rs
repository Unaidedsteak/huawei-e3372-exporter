use crate::metric::Metric;
use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub settings: Settings,
    pub metrics: Vec<Metric>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub listen_address: Ipv4Addr,
    pub listen_port: u16,
    pub dongle_ip: Ipv4Addr,
    pub collection_timeout_seconds: u64,
}
