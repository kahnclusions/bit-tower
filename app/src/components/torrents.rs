use std::time::Duration;

use human_bytes::human_bytes;
use humantime::format_duration;
use rust_decimal::prelude::*;
use tailwind_fuse::tw_merge;

use crate::signals::syncstate::Torrent;
use fnord_ui::components::{Text, View};
use leptos::prelude::*;

static CELL_CLASS: &'static str = "shadow-border p-2 whitespace-nowrap text-left font-normal";

#[component]
pub fn TorrentList(torrents: Signal<Vec<Torrent>>) -> impl IntoView {
    view! {
        <div class="h-full w-full overflow-auto overscroll-none">
            <table class=" border-grey border-spacing-[2px] border-collapse border-px w-full">
                <thead class="">
                    <tr>
                        <th class=tw_merge!(
                            "sticky left-0 bg-gray-50 dark:bg-gray-950 z-10 overflow-hidden text-ellipsis whitespace-nowrap max-w-[40vw] shadow-border p-1 text-left font-normal w-[40vw]",
                            CELL_CLASS
                        )>"Name"</th>
                        <th class=CELL_CLASS>"Progress"</th>
                        <th class=tw_merge!(CELL_CLASS, "w-[90px]")>"DL/s"</th>
                        <th class=tw_merge!(CELL_CLASS, "w-[90px]")>"UP/s"</th>
                        <th class=CELL_CLASS>"Seeds"</th>
                        <th class=CELL_CLASS>"Leechs"</th>
                        <th class=CELL_CLASS>"Eta"</th>
                        <th class=CELL_CLASS>"Avail."</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=torrents
                        key=|torrent| torrent.name.clone()
                        children=move |torrent| {
                            view! { <TorrentSummary torrent=torrent/> }
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

    // let downloaded = move || human_bytes(torrent.downloaded.get());
    // let uploaded = move || human_bytes(torrent.downloaded.get());
    let dlspeed = move || human_bytes(torrent.dlspeed.get());
    let upspeed = move || human_bytes(torrent.upspeed.get());

    let availability = move || torrent.availability.get().min(1.0);
    let eta = move || format_duration(Duration::from_secs_f64(torrent.eta.get())).to_string();

    view! {
        <tr class="gap-0">
            <th class=tw_merge!(
                "sticky left-0 bg-gray-50 dark:bg-gray-950 z-10 overflow-hidden text-ellipsis whitespace-nowrap max-w-[40vw] shadow-border p-1 text-left font-normal",
                CELL_CLASS
            )>{move || name()}</th>
            <td class=CELL_CLASS>
                <Progress
                    progress=torrent.progress
                    downloaded=torrent.downloaded
                    size=torrent.size
                    total_size=torrent.total_size
                />
            </td>
            <td class=tw_merge!(CELL_CLASS, "w-[90px]")>{move || dlspeed()}</td>
            <td class=tw_merge!(CELL_CLASS, "w-[90px]")>{move || upspeed()}</td>
            <td class=CELL_CLASS>{move || torrent.num_seeds.get()}</td>
            <td class=CELL_CLASS>{move || torrent.num_leechs.get()}</td>
            <td class=CELL_CLASS>{move || eta()}</td>
            <td class=CELL_CLASS>{move || availability()}</td>
        </tr>
    }
}

#[component]
fn Progress(
    downloaded: ArcRwSignal<f64>,
    progress: ArcRwSignal<f64>,
    size: ArcRwSignal<f64>,
    total_size: ArcRwSignal<f64>,
) -> impl IntoView {
    let total = total_size.clone();
    let percent_selected = move || size.get() / total.get();
    let inner_bar_w = move || (percent_selected().min(1.0) * 110.0).ceil();
    let inner_bar_w2 = inner_bar_w.clone();
    let percent_complete = move || (progress.get().min(1.0) * inner_bar_w()) - 8.0;

    view! {
        <div class="flex flex-col w-[110px] gap-[2px]">
            <div class="rounded bg-gray-100 dark:bg-gray-900 h-2">
                <div
                    class="rounded bg-gray-200 dark:bg-gray-800 h-2"
                    style:width=move || { format!("{}px", inner_bar_w2()) }
                >
                    <div
                        class="border-t-[2px] border-t-cyan-600 relative top-[3px] left-[4px]"
                        style:width=move || {
                            format!("{}px", (percent_complete().ceil().max(0.0)))
                        }
                    >

                        ""
                    </div>
                </div>
            </div>
            <div class="flex flex-row justify-between text-2xs">
                <div class="text-[11px]">{move || human_bytes(downloaded.get())}</div>
                <div class="text-[11px]">{move || human_bytes(total_size.get())}</div>
            </div>
        </div>
    }
}
