use fnord_ui::components::View;
use human_bytes::human_bytes;
use icondata as i;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_icons::Icon;
use use_websocket::core::ConnectionReadyState;

use crate::signals::syncstate::ServerState;

#[component]
pub fn StatusBar(
    server_state: ServerState,
    ready_state: Signal<ConnectionReadyState>,
) -> impl IntoView {
    let dl_speed = move || human_bytes(server_state.dl_info_speed.get());
    let up_speed = move || human_bytes(server_state.up_info_speed.get());

    let status = move || match ready_state.get() {
        ConnectionReadyState::Open => "Open",
        ConnectionReadyState::Closed => "Closed",
        ConnectionReadyState::Closing => "Closing",
        ConnectionReadyState::Connecting => "Opening",
    };

    let status_icon = move || match ready_state.get() {
        ConnectionReadyState::Open => {
            view! { <Icon icon=i::TbNetwork class=TextProp::from("w-4 w-4 text-lime-600") /> }
        }
        ConnectionReadyState::Connecting => {
            view! { <Icon icon=i::TbNetwork class=TextProp::from("w-4 w-4 text-grey-300") /> }
        }
        ConnectionReadyState::Closed => {
            view! { <Icon icon=i::TbNetworkOff class=TextProp::from("w-4 w-4 text-red-600") /> }
        }
        ConnectionReadyState::Closing => {
            view! { <Icon icon=i::TbNetworkOff class=TextProp::from("w-4 w-4 text-grey-300") /> }
        }
    };

    view! {
        <View class="flex-row bg-background-highlight justify-between items-stretch fixed bottom-0 left-0 right-0 h-8 text-sm border-t border-t-gray-300 dark:border-t-gray-700 gap-0">
            <View class="flex-row gap-1 p-1 px-2 border-r border-t-gray-300 dark:border-r-gray-700 grow w-full items-center">
                <Icon icon=i::FaDownloadSolid class=TextProp::from("w-4 h-4") />
                <span>{move || dl_speed()} "/s"</span>
            </View>
            <View class="flex-row gap-1  p-1 px-2 border-r border-t-gray-300 dark:border-r-gray-700 grow w-full items-center">
                <Icon icon=i::FaUploadSolid class=TextProp::from("w-4 h-4") />
                <span>{move || up_speed()} "/s"</span>
            </View>
            <View class="flex-row gap-1 items-center p-1 px-2 border-r border-t-gray-300 dark:border-r-gray-700 justify-start shrink-0">
                <Icon icon=i::BiNetworkChartRegular class=TextProp::from("w-4 w-4") />
                <span>{move || server_state.dht_nodes.get()}</span>
            </View>
            <View class="flex-row gap-1 items-center p-1 px-2 justify-start">
                {move || status_icon()} {move || status()}
            </View>
        </View>
    }
}
