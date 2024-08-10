use std::collections::HashMap;

use leptos::prelude::*;
use qbittorrent_rs_proto::{
    sync::SyncMainDataFull,
    torrents::{TorrentInfo, TorrentInfoPartial},
    transfer::{ConnectionStatus, ServerStateFull, ServerStatePartial},
};

#[derive(Debug, Clone, Default)]
pub struct SyncState {
    pub torrents: HashMap<String, Torrent>,
    pub server_state: ServerState,
}
impl From<SyncMainDataFull> for SyncState {
    fn from(value: SyncMainDataFull) -> Self {
        Self {
            torrents: value
                .torrents
                .into_iter()
                .map(|(hash, torrent)| (hash, Torrent::from(torrent)))
                .collect(),
            server_state: ServerState::from(value.server_state),
        }
    }
}
impl From<&SyncMainDataFull> for SyncState {
    fn from(value: &SyncMainDataFull) -> Self {
        Self {
            torrents: value
                .torrents
                .clone()
                .into_iter()
                .map(|(hash, torrent)| (hash, Torrent::from(torrent)))
                .collect(),
            server_state: ServerState::from(value.server_state.clone()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Torrent {
    pub infohash_v1: String,
    pub name: ArcRwSignal<String>,
    pub progress: ArcRwSignal<f64>,
    pub downloaded: ArcRwSignal<f64>,
    pub uploaded: ArcRwSignal<f64>,
    pub dlspeed: ArcRwSignal<f64>,
    pub upspeed: ArcRwSignal<f64>,
    pub num_seeds: ArcRwSignal<f64>,
    pub num_leechs: ArcRwSignal<f64>,
    pub size: ArcRwSignal<f64>,
    pub total_size: ArcRwSignal<f64>,
    pub availability: ArcRwSignal<f64>,
    pub eta: ArcRwSignal<f64>,
}

impl From<TorrentInfo> for Torrent {
    fn from(value: TorrentInfo) -> Self {
        Torrent {
            infohash_v1: value.infohash_v1,
            name: ArcRwSignal::new(value.name),
            progress: ArcRwSignal::new(value.progress),
            downloaded: ArcRwSignal::new(value.downloaded),
            uploaded: ArcRwSignal::new(value.uploaded),
            dlspeed: ArcRwSignal::new(value.dlspeed),
            upspeed: ArcRwSignal::new(value.upspeed),
            num_seeds: ArcRwSignal::new(value.num_seeds),
            num_leechs: ArcRwSignal::new(value.num_leechs),
            size: ArcRwSignal::new(value.size),
            total_size: ArcRwSignal::new(value.total_size),
            availability: ArcRwSignal::new(value.availability),
            eta: ArcRwSignal::new(value.eta),
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
        if let Some(new_value) = partial.downloaded {
            self.downloaded.set(new_value);
        }
        if let Some(new_value) = partial.uploaded {
            self.uploaded.set(new_value);
        }
        if let Some(new_value) = partial.dlspeed {
            self.dlspeed.set(new_value);
        }
        if let Some(new_value) = partial.upspeed {
            self.upspeed.set(new_value);
        }
        if let Some(new_value) = partial.num_seeds {
            self.num_seeds.set(new_value);
        }
        if let Some(new_value) = partial.num_leechs {
            self.num_leechs.set(new_value);
        }
        if let Some(new_value) = partial.size {
            self.size.set(new_value);
        }
        if let Some(new_value) = partial.total_size {
            self.total_size.set(new_value);
        }
        if let Some(new_value) = partial.availability {
            self.availability.set(new_value);
        }
        if let Some(new_value) = partial.eta {
            self.eta.set(new_value);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServerState {
    pub dl_info_speed: ArcRwSignal<f64>, // Global download rate (bytes/s)
    pub dl_info_data: ArcRwSignal<f64>,  // Data downloaded this session (bytes)
    pub up_info_speed: ArcRwSignal<f64>, // Global upload rate (bytes/s)
    pub up_info_data: ArcRwSignal<f64>,  // Data uploaded this session (bytes)
    pub dl_rate_limit: ArcRwSignal<f64>, // Download rate limit (bytes/s)
    pub up_rate_limit: ArcRwSignal<f64>, // Upload rate limit (bytes/s)
    pub dht_nodes: ArcRwSignal<f64>,     // DHT nodes connected to
    pub connection_status: ArcRwSignal<ConnectionStatus>, // Connection status
}

impl From<ServerStateFull> for ServerState {
    fn from(value: ServerStateFull) -> Self {
        ServerState {
            dl_info_speed: ArcRwSignal::new(value.dl_info_speed),
            dl_info_data: ArcRwSignal::new(value.dl_info_data),
            up_info_speed: ArcRwSignal::new(value.up_info_speed),
            up_info_data: ArcRwSignal::new(value.up_info_data),
            dl_rate_limit: ArcRwSignal::new(value.dl_rate_limit),
            up_rate_limit: ArcRwSignal::new(value.up_rate_limit),
            dht_nodes: ArcRwSignal::new(value.dht_nodes),
            connection_status: ArcRwSignal::new(value.connection_status),
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
