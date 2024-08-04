use std::collections::HashMap;

use leptos::prelude::*;
use qbittorrent_rs_proto::{
    torrents::{TorrentInfo, TorrentInfoPartial},
    transfer::{ConnectionStatus, ServerStateFull, ServerStatePartial},
};

pub struct SyncState {
    torrents: HashMap<String, Torrent>,
    server_state: ServerState,
}

pub struct Torrent {
    pub name: RwSignal<String>,
    pub progress: RwSignal<f64>,
}

impl From<TorrentInfo> for Torrent {
    fn from(value: TorrentInfo) -> Self {
        Torrent {
            name: RwSignal::new(value.name),
            progress: RwSignal::new(value.progress),
        }
    }
}
impl Torrent {
    pub fn apply_partial(&self, partial: TorrentInfoPartial) {
        if let Some(new_value) = partial.name {
            self.name.set(new_value);
        }
        if let Some(new_value) = partial.progress {
            self.progress.set(new_value);
        }
    }
}

pub struct ServerState {
    pub dl_info_speed: RwSignal<f64>, // Global download rate (bytes/s)
    pub dl_info_data: RwSignal<f64>,  // Data downloaded this session (bytes)
    pub up_info_speed: RwSignal<f64>, // Global upload rate (bytes/s)
    pub up_info_data: RwSignal<f64>,  // Data uploaded this session (bytes)
    pub dl_rate_limit: RwSignal<f64>, // Download rate limit (bytes/s)
    pub up_rate_limit: RwSignal<f64>, // Upload rate limit (bytes/s)
    pub dht_nodes: RwSignal<f64>,     // DHT nodes connected to
    pub connection_status: RwSignal<ConnectionStatus>, // Connection status
}

impl From<ServerStateFull> for ServerState {
    fn from(value: ServerStateFull) -> Self {
        ServerState {
            dl_info_speed: RwSignal::new(value.dl_info_speed),
            dl_info_data: RwSignal::new(value.dl_info_data),
            up_info_speed: RwSignal::new(value.up_info_speed),
            up_info_data: RwSignal::new(value.up_info_data),
            dl_rate_limit: RwSignal::new(value.dl_rate_limit),
            up_rate_limit: RwSignal::new(value.up_rate_limit),
            dht_nodes: RwSignal::new(value.dht_nodes),
            connection_status: RwSignal::new(value.connection_status),
        }
    }
}

impl ServerState {
    pub fn apply_partial(&self, partial: ServerStatePartial) {
        if let Some(cs) = partial.connection_status {
            self.connection_status.set(cs);
        }
        if let Some(v) = partial.dl_info_speed {
            self.dl_info_speed.set(v);
        }
        if let Some(v) = partial.up_info_speed {
            self.up_info_speed.set(v);
        }
        if let Some(v) = partial.dl_info_data {
            self.dl_info_data.set(v);
        }
        if let Some(v) = partial.up_info_data {
            self.up_info_data.set(v);
        }
        if let Some(v) = partial.dl_rate_limit {
            self.dl_rate_limit.set(v);
        }
        if let Some(v) = partial.up_rate_limit {
            self.up_rate_limit.set(v);
        }
        if let Some(v) = partial.dht_nodes {
            self.dht_nodes.set(v);
        }
    }
}
