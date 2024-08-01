use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Connected,
    Firewalled,
    Disconnected,
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(v) => write!(f, "{}", v),
            Err(_) => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerStateFull {
    pub dl_info_speed: f64,                  // Global download rate (bytes/s)
    pub dl_info_data: f64,                   // Data downloaded this session (bytes)
    pub up_info_speed: f64,                  // Global upload rate (bytes/s)
    pub up_info_data: f64,                   // Data uploaded this session (bytes)
    pub dl_rate_limit: f64,                  // Download rate limit (bytes/s)
    pub up_rate_limit: f64,                  // Upload rate limit (bytes/s)
    pub dht_nodes: f64,                      // DHT nodes connected to
    pub connection_status: ConnectionStatus, // Connection status
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerStatePartial {
    pub dl_info_speed: Option<f64>, // Global download rate (bytes/s)
    pub dl_info_data: Option<f64>,  // Data downloaded this session (bytes)
    pub up_info_speed: Option<f64>, // Global upload rate (bytes/s)
    pub up_info_data: Option<f64>,  // Data uploaded this session (bytes)
    pub dl_rate_limit: Option<f64>, // Download rate limit (bytes/s)
    pub up_rate_limit: Option<f64>, // Upload rate limit (bytes/s)
    pub dht_nodes: Option<f64>,     // DHT nodes connected to
    pub connection_status: Option<ConnectionStatus>, // Connection status
}
