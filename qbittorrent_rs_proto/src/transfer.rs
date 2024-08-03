use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Connected,
    Firewalled,
    #[default]
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

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl ServerStateFull {
    pub fn apply_partial(&mut self, partial: ServerStatePartial) {
        if let Some(cs) = partial.connection_status {
            self.connection_status = cs;
        }
        if let Some(v) = partial.dl_info_speed {
            self.dl_info_speed = v;
        }
        if let Some(v) = partial.up_info_speed {
            self.up_info_speed = v;
        }
        if let Some(v) = partial.dl_info_data {
            self.dl_info_data = v;
        }
        if let Some(v) = partial.up_info_data {
            self.up_info_data = v;
        }
        if let Some(v) = partial.dl_rate_limit {
            self.dl_rate_limit = v;
        }
        if let Some(v) = partial.up_rate_limit {
            self.up_rate_limit = v;
        }
        if let Some(v) = partial.dht_nodes {
            self.dht_nodes = v;
        }
    }
}
