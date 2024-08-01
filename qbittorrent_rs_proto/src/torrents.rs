use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TorrentStatus {
    #[serde(rename = "error")]
    /// Some error occurred, applies to paused torrents
    Error,

    #[serde(rename = "missingFiles")]
    /// Torrent data files is missing
    MissingFiles,

    #[serde(rename = "uploading")]
    /// Torrent is being seeded and data is being transferred
    Uploading,

    #[serde(rename = "pausedUP")]
    /// Torrent is paused and has finished downloading
    PausedUP,

    #[serde(rename = "queuedUP")]
    /// Queuing is enabled and torrent is queued for upload
    QueuedUP,

    #[serde(rename = "stalledUP")]
    /// Torrent is being seeded, but no connection were made
    StalledUP,

    #[serde(rename = "checkingUP")]
    /// Torrent has finished downloading and is being checked
    CheckingUP,

    #[serde(rename = "forcedUP")]
    /// Torrent is forced to uploading and ignore queue limit
    ForcedUP,

    #[serde(rename = "allocating")]
    /// Torrent is allocating disk space for download
    Allocating,

    #[serde(rename = "downloading")]
    /// Torrent is being downloaded and data is being transferred
    Downloading,

    #[serde(rename = "metaDL")]
    /// Torrent has just started downloading and is fetching metadata
    MetaDL,

    #[serde(rename = "pausedDL")]
    /// Torrent is paused and has NOT finished downloading
    PausedDL,

    #[serde(rename = "queuedDL")]
    /// Queuing is enabled and torrent is queued for download
    QueuedDL,

    #[serde(rename = "stalledDL")]
    /// Torrent is being downloaded, but no connection were made
    StalledDL,

    #[serde(rename = "checkingDL")]
    /// Same as checkingUP, but torrent has NOT finished downloading
    CheckingDL,

    #[serde(rename = "forcedDL")]
    /// Torrent is forced to downloading to ignore queue limit
    ForcedDL,

    #[serde(rename = "checkingResumeData")]
    /// Checking resume data on qBt startup
    CheckingResumeData,

    #[serde(rename = "moving")]
    /// Torrent is moving to another location
    Moving,

    #[serde(rename = "unknown")]
    /// Unknown status
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TorrentSummary {
    pub name: String,
    pub infohash_v1: String,
    pub progress: f64,
    pub state: TorrentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TorrentSummaryPartial {
    pub name: Option<String>,
    pub infohash_v1: Option<String>,
    pub progress: Option<f64>,
    pub state: Option<TorrentStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TorrentInfo {
    pub added_on: f64,     // Time (Unix Epoch) when the torrent was added to the client
    pub amount_left: f64,  // Amount of data left to download (bytes)
    pub auto_tmm: bool,    // Whether this torrent is managed by Automatic Torrent Management
    pub availability: f64, // Percentage of file pieces currently available
    pub category: String,  // Category of the torrent
    pub completed: f64,    // Amount of transfer data completed (bytes)
    pub completion_on: f64, // Time (Unix Epoch) when the torrent completed
    pub content_path: String, // Absolute path of torrent content (root path for multifile torrents, absolute file path for singlefile torrents)
    pub dl_limit: f64,        // Torrent download speed limit (bytes/s). -1 if ulimited.
    pub dlspeed: f64,         // Torrent download speed (bytes/s)
    pub downloaded: f64,      // Amount of data downloaded
    pub downloaded_session: f64, // Amount of data downloaded this session
    pub eta: f64,             // Torrent ETA (seconds)
    pub infohash_v1: String,  // Torrent hash
    pub f_l_piece_prio: bool, // True if first last piece are prioritized
    pub force_start: bool,    // True if force start is enabled for this torrent
    pub last_activity: f64,   // Last time (Unix Epoch) when a chunk was downloaded/uploaded
    pub magnet_uri: String,   // Magnet URI corresponding to this torrent
    pub max_ratio: f64,       // Maximum share ratio until torrent is stopped from seeding/uploading
    pub max_seeding_time: f64, // Maximum seeding time (seconds) until torrent is stopped from seeding
    pub name: String,          // Torrent name
    pub num_complete: f64,     // Number of seeds in the swarm
    pub num_incomplete: f64,   // Number of leechers in the swarm
    pub num_leechs: f64,       // Number of leechers connected to
    pub num_seeds: f64,        // Number of seeds connected to
    pub priority: f64, // Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
    pub progress: f64, // Torrent progress (percentage/100)
    pub ratio: f64,    // Torrent share ratio. Max ratio value: 9999.
    pub ratio_limit: f64, // TODO (what is different from max_ratio?)
    pub save_path: String, // Path where this torrent's data is stored
    pub seeding_time: f64, // Torrent elapsed time while complete (seconds)
    pub seeding_time_limit: f64, // TODO (what is different from max_seeding_time?) seeding_time_limit is a per torrent setting, when Automatic Torrent Management is disabled, furthermore then max_seeding_time is set to seeding_time_limit for this torrent. If Automatic Torrent Management is enabled, the value is -2. And if max_seeding_time is unset it have a default value -1.
    pub seen_complete: f64,      // Time (Unix Epoch) when this torrent was last seen complete
    pub seq_dl: bool,            // True if sequential download is enabled
    pub size: f64,               // Total size (bytes) of files selected for download
    pub state: TorrentStatus,    // Torrent state. See table here below for the possible values
    pub super_seeding: bool,     // True if super seeding is enabled
    pub tags: String,            // Comma-concatenated tag list of the torrent
    pub time_active: f64,        // Total active time (seconds)
    pub total_size: f64, // Total size (bytes) of all file in this torrent (including unselected ones)
    pub tracker: String, // The first tracker with working status. Returns empty string if no tracker is working.
    pub up_limit: f64,   // Torrent upload speed limit (bytes/s). -1 if ulimited.
    pub uploaded: f64,   // Amount of data uploaded
    pub uploaded_session: f64, // Amount of data uploaded this session
    pub upspeed: f64,    // Torrent upload speed (bytes/s)
}
