use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    torrents::{TorrentSummary, TorrentSummaryPartial},
    transfer::{ServerStateFull, ServerStatePartial},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncMainDataFull {
    pub full_update: bool,
    pub rid: u64,
    pub torrents: HashMap<String, TorrentSummary>,
    pub server_state: ServerStateFull,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncMainDataPartial {
    pub rid: u64,
    pub torrents: Option<HashMap<String, TorrentSummaryPartial>>,
    pub server_state: Option<ServerStatePartial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MainData {
    Full(SyncMainDataFull),
    Partial(SyncMainDataPartial),
}
