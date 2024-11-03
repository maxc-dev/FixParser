mod fix_file_reader;
mod fix_message_parser;
mod fix_message;
mod new_order;
mod order_cancel_request;
mod execution_report;
mod order_status_request;
mod client;

use crate::execution_report::ExecutionReport;
use crate::fix_message::FixMessage;
use crate::fix_message_parser::FixMessageParser;
use crate::new_order::NewOrder;
use crate::order_cancel_request::OrderCancelRequest;
use crate::order_status_request::OrderStatusRequest;
use crate::FixMessageParseResponse::{Success, UnknownMessageType};

use axum::{routing::{post, get}, Router, {response::IntoResponse}, {extract::ws::{WebSocket, WebSocketUpgrade}}};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use futures_util::{SinkExt, StreamExt};

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
async fn post_fix_msg(body: String) -> String {
    let result = handle_fix_str(body.as_str());
    format!("Parse response: {:?}", result)

}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_websocket)
}

async fn handle_websocket(mut socket: WebSocket) {
    while let Some(Ok(message)) = socket.next().await {
        if let axum::extract::ws::Message::Text(text) = message {
            let parsed_message = handle_fix_str(&text);
            socket.send(axum::extract::ws::Message::Text(format!("FIX Parser Response: {:?}", parsed_message))).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1/fix", post(post_fix_msg))
        .route("/ws", get(ws_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("listening on {}", addr);

    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug)]
enum FixMessageParseResponse {
    Success,
    UnknownMessageType,
}

fn handle_fix_str(msg: &str) -> FixMessageParseResponse {
    match FixMessageParser::parse_message(msg) {
        FixMessage::NewOrder(new_order) => handle_new_order(new_order),
        FixMessage::ExecutionReport(execution_report) => handle_execution_report(execution_report),
        FixMessage::OrderCancelRequest(order_cancel_request) => handle_order_cancel_request(order_cancel_request),
        FixMessage::OrderStatusRequest(order_status_request) => handle_order_status_request(order_status_request),
        FixMessage::Unknown => UnknownMessageType
    }
}

fn handle_new_order(new_order: NewOrder) -> FixMessageParseResponse {
    println!("Handling NewOrder: {:?}", new_order.cl_ord_id);
    println!(" -> {:?}", new_order);
    Success
}

fn handle_execution_report(execution_report: ExecutionReport) -> FixMessageParseResponse {
    println!("Handling ExecutionReport: {:?}", execution_report.cl_ord_id);
    println!(" -> {:?}", execution_report);
    Success
}

fn handle_order_cancel_request(order_cancel_request: OrderCancelRequest) -> FixMessageParseResponse {
    println!("Handling OrderCancelRequest: {:?}", order_cancel_request.cl_ord_id);
    println!(" -> {:?}", order_cancel_request);
    Success
}

fn handle_order_status_request(order_status_request: OrderStatusRequest) -> FixMessageParseResponse {
    println!("Handling OrderStatusRequest: {:?}", order_status_request.cl_ord_id);
    println!(" -> {:?}", order_status_request);
    Success
}
