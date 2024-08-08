use leptos::prelude::*;
use qbittorrent_rs_proto::sync::MainData;
use use_websocket::{core::ConnectionReadyState, use_websocket, UseWebSocketReturn};

use codee::binary::MsgpackSerdeCodec;

use crate::{
    components::{status_bar::StatusBar, torrents::TorrentList},
    signals::{
        syncstate::SyncState,
        use_sync_maindata::{use_sync_maindata, UseSyncMaindataReturn},
    },
};

#[component]
pub fn Example() -> impl IntoView {
    let UseSyncMaindataReturn {
        connected,
        data,
        open,
        close,
    } = use_sync_maindata();
    let status = move || {
        if connected.get() {
            "Connected"
        } else {
            "Disconnected"
        }
    };
    let server_data = data.clone();
    let torrents = Signal::derive(move || {
        let v: Vec<_> = data().torrents.into_iter().map(|(_h, v)| v).collect();
        v
    });
    let server_state = Signal::derive(move || server_data().server_state);

    // let open_connection = move |_| {
    //     open();
    // };
    //
    // let close_connection = move |_| {
    //     close();
    // };

    view! {
        {move || view! {<TorrentList torrents=torrents />}}
        {move || view! {<StatusBar server_state=server_state() />}}
    }
}
