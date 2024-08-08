use core::panic;
use std::net::SocketAddr;
use std::ops::ControlFlow;

use app::*;
use auth::ssr::Session;
use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use leptos::prelude::*;
use qbittorrent_rs_proto::sync::MainData;

use crate::AppState;

/// Actual websocket statemachine (one will be spawned per connection)
#[tracing::instrument(skip(socket))]
pub async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    app_state: AppState,
    session: Session,
) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        tracing::info!("Pinged {who}...");
    } else {
        tracing::info!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let mut rid = 0_u64;
        let sid = session.sid.as_str();
        loop {
            // handle_ws(app_state.qbt.clone(), session.sid).await
            let res = app_state.qbt.sync_maindata(sid, rid).await;

            let Ok(maindata) = res else {
                tracing::error!("Oops, the thing went boom.");
                break;
                // return Err(axum::BoxError::from(anyhow!("Oops")));
            };
            rid = match &maindata {
                MainData::Full(md) => md.rid,
                MainData::Partial(md) => md.rid,
            };
            let res = sender
                .send(Message::Binary(rmp_serde::to_vec(&maindata).unwrap()))
                .await;
            if let Err(err) = res {
                tracing::error!(error = %err);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
        rid
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => tracing::info!("{a} messages sent to {who}"),
                Err(a) => tracing::info!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => tracing::info!("Received {b} messages"),
                Err(b) => tracing::info!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    tracing::info!("Websocket context {who} destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
#[tracing::instrument]
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            tracing::info!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            tracing::info!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            tracing::info!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            tracing::info!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
