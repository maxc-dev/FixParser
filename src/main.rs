mod ws_client;
mod fix;
mod oms;

use oms::order_management_system::OrderManagementSystem;
use std::fmt::Debug;
use crate::fix::fix_message::FixMessage;
use crate::fix::fix_message_parser::FixMessageParser;

use axum::{routing::{get, post}, Router, extract::ws::{WebSocket, WebSocketUpgrade}, response::IntoResponse};
use crossbeam::channel::{unbounded, Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use axum::extract::ws::Message;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const FIX_TAG: &str = "fix";

#[derive(OpenApi)]
#[openapi(
    tags(
            (name = FIX_TAG, description = "Parses FIX protocol messages.")
    ),
    paths(post_fix_msg),
)]
struct ApiDoc;

#[utoipa::path(
    post,
    path = "/api/v1/fix",
    tag = FIX_TAG,
    request_body(content = String, description = "Raw FIX message"),
    responses(
        (status = 200, description = "FIX message received")
    )
)]
async fn post_fix_msg(body: String, sender: Sender<FixMessage>) -> String {
    let parsed_message = FixMessageParser::parse_message(&body);

    if sender.send(parsed_message).is_err() {
        return "Failed to send message to channel".to_string();
    }
    "FIX message received and sent to channel".to_string()
}

async fn ws_handler(ws: WebSocketUpgrade, sender: Sender<FixMessage>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_fix_msg(socket, sender))
}

async fn ws_fix_msg(mut socket: WebSocket, sender: Sender<FixMessage>) {
    while let Some(Ok(message)) = socket.next().await {
        if let Message::Text(text) = message {
            let parsed_message = FixMessageParser::parse_message(&text);

            if sender.send(parsed_message).is_err() {
                eprintln!("Failed to send message to channel");
            }

            if socket.send(Message::Text("FIX message received and sent to channel".to_string())).await.is_err() {
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (sender, receiver): (Sender<FixMessage>, Receiver<FixMessage>) = unbounded();

    let oms = OrderManagementSystem::new(receiver);
    tokio::spawn(async move {
        oms.listen_for_orders();
    });

    let post_sender = sender.clone();
    let ws_sender = sender.clone();
    let app = Router::new()
        .route("/api/v1/fix", post(move |body| post_fix_msg(body, post_sender)))
        .route("/ws", get(move |ws| ws_handler(ws, ws_sender)))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("listening on {}", addr);

    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}