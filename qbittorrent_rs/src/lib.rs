use http::header::COOKIE;
use serde_json::Value;

use qbittorrent_rs_proto::sync::{MainData, SyncMainDataFull, SyncMainDataPartial};
use qbittorrent_rs_proto::torrents::TorrentSummary;

pub static BASE_QBT_URL: &str = "http://localhost:9090/api/v2";
pub static TORRENTS_API: &str = "/torrents";
pub static INFO_API: &str = "/info";
pub static SYNC_API: &str = "/sync";
pub static MAINDATA_API: &str = "/maindata";

#[derive(Clone, Debug)]
pub struct QbtClient {
    base_url: String,
}

#[derive(thiserror::Error, Debug)]
pub enum QbtError {
    #[error("Unauthenticated")]
    Unauthenticated,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl QbtClient {
    #[tracing::instrument]
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_owned(),
        }
    }

    #[tracing::instrument]
    pub async fn auth_login(&self, username: String, password: String) -> Result<String, QbtError> {
        let url = format!("{}{}{}", self.base_url, TORRENTS_API, INFO_API);
        let client = reqwest::Client::builder().build()?;

        let params = [("username", username), ("password", password)];
        let response = client.post(url).form(&params).send().await?;

        let Some(sid) = response.cookies().find(|c| c.name() == "SID") else {
            return Err(QbtError::Unauthenticated);
        };

        Ok(sid.value().to_owned())
    }

    #[tracing::instrument]
    pub async fn torrents_info(&self, sid: String) -> Result<Vec<TorrentSummary>, reqwest::Error> {
        let url = format!("{}{}{}", self.base_url, TORRENTS_API, INFO_API);
        let client = reqwest::Client::builder().build()?;

        // let cookie: Cookie = Cookie::build(("SID", sid)).build();

        // Make an initial request to set some cookies
        let response = client
            .get(url)
            .header(COOKIE, format!("SID={}", sid).to_string())
            .send()
            .await?;

        response.json().await
    }

    #[tracing::instrument]
    pub async fn sync_maindata(&self, sid: String, rid: u64) -> Result<MainData, reqwest::Error> {
        let url = format!("{}{}{}?rid={}", self.base_url, SYNC_API, MAINDATA_API, rid);
        let client = reqwest::Client::builder().build()?;

        let response = client
            .get(url)
            .header(COOKIE, format!("SID={}", sid).to_string())
            .send()
            .await?;

        let data = response.json::<Value>().await?;
        let is_full_update = match data.get("full_update") {
            Some(full_update) => serde_json::from_value(full_update.to_owned()).unwrap(),
            None => false,
        };

        if is_full_update == true {
            let data: SyncMainDataFull = serde_json::from_value(data).unwrap();
            Ok(MainData::Full(data))
        } else {
            let data: SyncMainDataPartial = serde_json::from_value(data).unwrap();
            Ok(MainData::Partial(data))
        }
    }
}
