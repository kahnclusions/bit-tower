use anyhow::anyhow;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream;
use futures::stream::Stream;
use futures::TryStreamExt;
use leptos::prelude::*;
use qbittorrent_rs::QbtClient;
use qbittorrent_rs_proto::sync::MainData;
use std::time::Duration;
use tokio_stream::StreamExt as _;

pub async fn handle_sse(
    qbt: QbtClient,
    sid: String,
) -> Sse<impl Stream<Item = Result<Event, axum::BoxError>>> {
    let stream = stream::try_unfold(0_u64, move |state| {
        let qbt = qbt.clone();
        let sid = sid.clone();
        async move {
            let res = qbt.sync_maindata(sid.as_str(), state).await;

            let Ok(maindata) = res else {
                tracing::error!("Oops, the thing went boom.");
                return Err(axum::BoxError::from(anyhow!("Oops")));
            };
            let rid = match &maindata {
                MainData::Full(md) => md.rid,
                MainData::Partial(md) => md.rid,
            };
            let json = serde_json::to_string(&maindata).unwrap();
            Ok(Some((Event::default().data(json), rid)))
        }
    })
    .throttle(Duration::from_secs_f32(1.0));
    Sse::new(stream.into_stream()).keep_alive(KeepAlive::default())
}
