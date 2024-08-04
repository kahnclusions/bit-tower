use human_bytes::human_bytes;
use rust_decimal::prelude::*;
use std::collections::HashMap;
use tailwind_fuse::tw_merge;

use fnord_ui::components::{Text, View};
use leptos::prelude::*;
use qbittorrent_rs_proto::torrents::TorrentInfo;
use qbittorrent_rs_sse::signals::Torrent;

static CELL_CLASS: &'static str = "shadow-border p-2 whitespace-nowrap text-left font-normal";

#[component]
pub fn TorrentList(torrents: Signal<Vec<Torrent>>) -> impl IntoView {
    view! {
        <div class="h-full w-full overflow-auto ">
            <table class=" border-grey border-spacing-[2px] border-collapse border-px">
                <thead class="">
                    <tr>
                        <th class=tw_merge!("sticky left-0 bg-background z-10 overflow-hidden text-ellipsis whitespace-nowrap max-w-[40vw] shadow-border p-1 text-left font-normal", CELL_CLASS)>"Name"</th>
                        <th class=CELL_CLASS>"%"</th>
                        <th class=CELL_CLASS>"DL/s"</th>
                        <th class=CELL_CLASS>"UP/s"</th>
                        <th class=CELL_CLASS>"DL"</th>
                        <th class=CELL_CLASS>"UP"</th>
                        <th class=CELL_CLASS>"SD"</th>
                        <th class=CELL_CLASS>"LE"</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=torrents
                        key=|torrent| torrent.name.clone()
                        children=move |torrent| {
                            view! { <TorrentSummary torrent=torrent /> }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn TorrentSummary(torrent: Torrent) -> impl IntoView {
    let name = move || torrent.name.get();
    let progress = move || {
        Decimal::from_str(format!("{:.2}", torrent.progress.get() * 100.0).as_str())
            .unwrap()
            .normalize()
            .to_string()
    };

    let downloaded = move || human_bytes(torrent.downloaded.get());
    let uploaded = move || human_bytes(torrent.downloaded.get());
    let dlspeed = move || human_bytes(torrent.dlspeed.get());
    let upspeed = move || human_bytes(torrent.upspeed.get());

    view! {
        <tr class="gap-0">
            <th class=tw_merge!("sticky left-0 bg-background z-10 overflow-hidden text-ellipsis whitespace-nowrap max-w-[40vw] shadow-border p-1 text-left font-normal", CELL_CLASS)>{move || name()}</th>
            <td class=CELL_CLASS>{move || progress()}</td>
            <td class=CELL_CLASS>{move || dlspeed()}</td>
            <td class=CELL_CLASS>{move || upspeed()}</td>
            <td class=CELL_CLASS>{move || downloaded()}</td>
            <td class=CELL_CLASS>{move || uploaded()}</td>
            <td class=CELL_CLASS>{move || torrent.num_seeds.get()}</td>
            <td class=CELL_CLASS>{move || torrent.num_leechs.get()}</td>
        </tr>
    }
}
