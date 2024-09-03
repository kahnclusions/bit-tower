use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::torrents::{TorrentInfo, TorrentInfoPartial};
use super::transfer::{ServerStateFull, ServerStatePartial};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncMainDataFull {
    pub full_update: bool,
    pub rid: u64,
    pub torrents: HashMap<String, TorrentInfo>,
    pub server_state: ServerStateFull,
}

impl SyncMainDataFull {
    pub fn apply_partial(&mut self, partial: SyncMainDataPartial) {
        self.rid = partial.rid;
        if let Some(server_state) = partial.server_state {
            self.server_state.apply_partial(server_state);
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncMainDataPartial {
    pub rid: u64,
    pub torrents: Option<HashMap<String, TorrentInfoPartial>>,
    pub server_state: Option<ServerStatePartial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MainData {
    Full(SyncMainDataFull),
    Partial(SyncMainDataPartial),
}

impl MainData {
    pub fn rid(&self) -> u64 {
        match self {
            Self::Full(fd) => fd.rid,
            Self::Partial(pd) => pd.rid,
        }
    }
}

impl Default for MainData {
    fn default() -> Self {
        MainData::Partial(SyncMainDataPartial::default())
    }
}
