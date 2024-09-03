use crate::app::hooks::use_websocket::{
    core::ConnectionReadyState, use_websocket, use_websocket_with_options, UseWebSocketError,
    UseWebSocketOptions, UseWebSocketReturn,
};
use crate::qbittorrent::proto::sync::MainData;
use leptos::prelude::*;

use codee::binary::MsgpackSerdeCodec;

use crate::app::{
    components::{status_bar::StatusBar, torrents::TorrentList},
    signals::syncstate::SyncState,
};

#[derive(Clone)]
pub struct UseSyncMaindataReturn<OpenFn, CloseFn>
where
    OpenFn: Fn() + Clone + 'static,
    CloseFn: Fn() + Clone + 'static,
{
    pub ready_state: Signal<ConnectionReadyState>,
    pub connected: Signal<bool>,
    pub data: ReadSignal<SyncState>,
    pub open: OpenFn,
    pub close: CloseFn,
}

pub fn use_sync_maindata(
    url: &str,
) -> UseSyncMaindataReturn<impl Fn() + Clone + 'static, impl Fn() + Clone + 'static> {
    let (data, set_data) = signal(SyncState::default());

    let opts = UseWebSocketOptions::default();

    let UseWebSocketReturn {
        ready_state,
        message,
        open,
        close,
        ..
    } = use_websocket_with_options::<MainData, MsgpackSerdeCodec>(url, opts);

    let connected = Signal::derive(move || ready_state.get() == ConnectionReadyState::Open);

    Effect::new(move |_| {
        message.with(|message| {
            if let Some(m) = message {
                match m {
                    MainData::Full(full_data) => set_data.set(SyncState::from(full_data)),
                    MainData::Partial(partial_data) => {
                        data.with(|data| {
                            let torrents = partial_data.clone().torrents;
                            if let Some(torrents) = torrents {
                                for (hash, partial) in torrents {
                                    data.torrents
                                        .get(&hash)
                                        .map(|torrent| torrent.apply_partial(partial));
                                }
                            }
                            if let Some(server_state) = partial_data.clone().server_state {
                                data.server_state.apply_partial(server_state);
                            }
                        });
                    }
                }
            }
        });
    });

    UseSyncMaindataReturn {
        ready_state,
        connected,
        data,
        open,
        close,
    }
}
