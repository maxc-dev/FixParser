mod fix_file_reader;
mod fix_message_parser;
mod fix_message;
mod new_order;
mod order_cancel_request;
mod execution_report;
mod order_status_request;

use fix_file_reader::FixMessageFileReader;
use crate::execution_report::ExecutionReport;
use crate::fix_message::FixMessage;
use crate::new_order::NewOrder;
use crate::order_cancel_request::OrderCancelRequest;
use crate::order_status_request::OrderStatusRequest;

fn main() {
    let paths = vec![
        //"resources/new_order1.txt",
        "resources/execution_report1.txt",
        //"resources/order_cancel_request1.txt",
        "resources/order_status_request1.txt"
    ];

    for path in paths {
        handle_file(path);
    }
}

fn handle_file(path: &str) {
    match FixMessageFileReader::read_from_file(path) {
        Ok(messages) => {
            for fix_message in messages {
                match fix_message {
                    FixMessage::NewOrder(new_order) => handle_new_order(new_order),
                    FixMessage::ExecutionReport(execution_report) => handle_execution_report(execution_report),
                    FixMessage::OrderCancelRequest(order_cancel_request) => handle_order_cancel_request(order_cancel_request),
                    FixMessage::OrderStatusRequest(order_status_request) => handle_order_status_request(order_status_request),
                    FixMessage::Unknown => eprintln!("Unknown message type"),
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn handle_new_order(new_order: NewOrder) {
    println!("Handling NewOrder: {:?}", new_order.cl_ord_id);
    println!(" -> {:?}", new_order);
}

fn handle_execution_report(execution_report: ExecutionReport) {
    println!("Handling ExecutionReport: {:?}", execution_report.cl_ord_id);
    println!(" -> {:?}", execution_report);
}

fn handle_order_cancel_request(order_cancel_request: OrderCancelRequest) {
    println!("Handling OrderCancelRequest: {:?}", order_cancel_request.cl_ord_id);
    println!(" -> {:?}", order_cancel_request);
}

fn handle_order_status_request(order_status_request: OrderStatusRequest) {
    println!("Handling OrderStatusRequest: {:?}", order_status_request.cl_ord_id);
    println!(" -> {:?}", order_status_request);
}
