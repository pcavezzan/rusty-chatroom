use rocket::async_stream::stream;
use rocket::futures::{SinkExt, StreamExt};
use rocket::routes;
use rocket_ws::{Channel, WebSocket};

#[rocket::get("/")]
fn chat(ws: WebSocket) -> Channel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let _ = stream.send(message?).await;
            }
            Ok(())
        })
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![chat]).launch().await;
}
