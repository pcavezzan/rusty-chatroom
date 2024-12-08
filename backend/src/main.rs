use std::collections::HashMap;
use std::os::macos::raw::stat;
use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::async_stream::stream;
use rocket::futures::{SinkExt, StreamExt};
use rocket::futures::stream::SplitSink;
use rocket::{routes, State};
use rocket::tokio::sync::Mutex;
use rocket_ws::{Channel, Message, WebSocket};
use rocket_ws::stream::DuplexStream;

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
struct ChatRoom {
    connections: Mutex<HashMap<usize, SplitSink<DuplexStream, Message>>>,
}

#[rocket::get("/")]
fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();
            {
                let mut cons = state.connections.lock().await;
                cons.insert(user_id, ws_sink);
            }
            while let Some(message) = ws_stream.next().await {
                {
                    let msg = message?;
                    let mut cons = state.connections.lock().await;
                    for (_, sink) in cons.iter_mut() {
                        let _ = sink.send(msg.clone()).await;
                    }
                }
            }

            {
                let mut cons = state.connections.lock().await;
                cons.remove(&user_id);
            }
            Ok(())
        })
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![chat])
        .manage(ChatRoom::default())
        .launch().await;
}
