mod fix_file_reader;
mod fix_message_parser;
mod fix_message;
mod new_order;
mod order_cancel_request;
mod execution_report;
mod order_status_request;

use crate::execution_report::ExecutionReport;
use crate::fix_message::FixMessage;
use crate::new_order::NewOrder;
use crate::order_cancel_request::OrderCancelRequest;
use crate::order_status_request::OrderStatusRequest;
use crate::fix_message_parser::FixMessageParser;
use crate::FixMessageParsed::{Success, UnknownMessageType};

use axum::{routing::post, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;

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
    let result = match handle_fix_str(body.as_str()) {
        Success => "Message parsed successfully.".to_string(),
        UnknownMessageType => "Unknown message type received.".to_string()
    };
    format!("Parse response: {:?}", result)

}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1/fix", post(post_fix_msg))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("listening on {}", addr);

    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

enum FixMessageParsed {
    Success,
    UnknownMessageType,
}

fn handle_fix_str(msg: &str) -> FixMessageParsed {
    match FixMessageParser::parse_message(msg) {
        FixMessage::NewOrder(new_order) => handle_new_order(new_order),
        FixMessage::ExecutionReport(execution_report) => handle_execution_report(execution_report),
        FixMessage::OrderCancelRequest(order_cancel_request) => handle_order_cancel_request(order_cancel_request),
        FixMessage::OrderStatusRequest(order_status_request) => handle_order_status_request(order_status_request),
        FixMessage::Unknown => FixMessageParsed::UnknownMessageType
    }
}

fn handle_new_order(new_order: NewOrder) -> FixMessageParsed {
    println!("Handling NewOrder: {:?}", new_order.cl_ord_id);
    println!(" -> {:?}", new_order);
    Success
}

fn handle_execution_report(execution_report: ExecutionReport) -> FixMessageParsed {
    println!("Handling ExecutionReport: {:?}", execution_report.cl_ord_id);
    println!(" -> {:?}", execution_report);
    Success
}

fn handle_order_cancel_request(order_cancel_request: OrderCancelRequest) -> FixMessageParsed {
    println!("Handling OrderCancelRequest: {:?}", order_cancel_request.cl_ord_id);
    println!(" -> {:?}", order_cancel_request);
    Success
}

fn handle_order_status_request(order_status_request: OrderStatusRequest) -> FixMessageParsed {
    println!("Handling OrderStatusRequest: {:?}", order_status_request.cl_ord_id);
    println!(" -> {:?}", order_status_request);
    Success
}
