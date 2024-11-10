use crate::fix::fix_message::FixMessage;
use std::collections::HashMap;
use crate::fix::messages::execution_report::ExecutionReport;
use crate::fix::messages::new_order::NewOrder;
use crate::fix::messages::order_cancel_request::OrderCancelRequest;
use crate::fix::messages::order_status_request::OrderStatusRequest;

pub const DELIMITER: char = '|';
pub const ASSIGNMENT: char = '=';

pub struct FixMessageParser;

impl FixMessageParser {
    pub fn parse_message(message: &str) -> FixMessage {
        println!("Parsing message: {}", message);
        let mut fields = HashMap::new();
        let mut msg_type = "";

        for pair in message.split(DELIMITER) {
            if let Some((key, value)) = pair.split_once(ASSIGNMENT) {
                fields.insert(key.to_string(), value.to_string());
                if key == "35" {
                    msg_type = value;
                }
            }
        }

        match msg_type {
            "D" => NewOrder::new(fields).map_or(FixMessage::Unknown, FixMessage::NewOrder),
            "8" => ExecutionReport::new(fields).map_or(FixMessage::Unknown, FixMessage::ExecutionReport),
            "F" => OrderCancelRequest::new(fields).map_or(FixMessage::Unknown, FixMessage::OrderCancelRequest),
            "H" => OrderStatusRequest::new(fields).map_or(FixMessage::Unknown, FixMessage::OrderStatusRequest),
            _ => {
                eprintln!("Unknown message type: {}", msg_type);
                FixMessage::Unknown
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_new_order() {
        let message = "35=D|11=12345|54=1|60=20230101-12:00:00|38=100|40=2|44=50.5|55=XYZ|59=0|1=ACC123|15=USD|18=G|";
        let parsed_message = FixMessageParser::parse_message(message);

        match parsed_message {
            FixMessage::NewOrder(new_order) => {
                assert_eq!(new_order.cl_ord_id, "12345");
                assert_eq!(new_order.side, '1');
                assert_eq!(new_order.transact_time, "20230101-12:00:00");
                assert_eq!(new_order.order_qty, 100.0);
                assert_eq!(new_order.ord_type, '2');
                assert_eq!(new_order.price, Some(50.5));
                assert_eq!(new_order.symbol, "XYZ");
                assert_eq!(new_order.time_in_force, Some('0'));
                assert_eq!(new_order.account, Some("ACC123".to_string()));
                assert_eq!(new_order.currency, Some("USD".to_string()));
                assert_eq!(new_order.exec_inst, Some("G".to_string()));
            },
            _ => panic!("Expected NewOrder message"),
        }
    }

    #[test]
    fn test_parse_execution_report() {
        let message = "35=8|11=12345|17=1|150=0|39=2|55=XYZ|54=1|38=100|44=50.5|37=54321|151=100|14=0|60=20231027-15:48:00.123|";
        let parsed_message = FixMessageParser::parse_message(message);

        match parsed_message {
            FixMessage::ExecutionReport(execution_report) => {
                assert_eq!(execution_report.cl_ord_id, "12345");
                assert_eq!(execution_report.order_id, "54321");
                assert_eq!(execution_report.exec_type, '0');
                assert_eq!(execution_report.ord_status, '2');
                assert_eq!(execution_report.symbol, "XYZ");
                assert_eq!(execution_report.side, '1');
                assert_eq!(execution_report.order_qty, 100.0);
                assert_eq!(execution_report.price, 50.5);
            },
            _ => panic!("Expected ExecutionReport message"),
        }
    }

    #[test]
    fn test_parse_order_cancel_request() {
        let message = "35=F|11=12345|41=54321|54=1|55=XYZ|60=20231027-15:48:00.123|";
        let parsed_message = FixMessageParser::parse_message(message);

        match parsed_message {
            FixMessage::OrderCancelRequest(order_cancel_request) => {
                assert_eq!(order_cancel_request.cl_ord_id, "12345");
                assert_eq!(order_cancel_request.orig_cl_ord_id, "54321");
                assert_eq!(order_cancel_request.side, '1');
                assert_eq!(order_cancel_request.symbol, "XYZ");
            },
            _ => panic!("Expected OrderCancelRequest message"),
        }
    }

    #[test]
    fn test_parse_order_status_request() {
        let message = "35=H|11=12345|54=1|55=XYZ|60=20231027-15:48:00.123|";
        let parsed_message = FixMessageParser::parse_message(message);

        match parsed_message {
            FixMessage::OrderStatusRequest(order_status_request) => {
                assert_eq!(order_status_request.cl_ord_id, "12345");
                assert_eq!(order_status_request.side, '1');
                assert_eq!(order_status_request.symbol, "XYZ");
            },
            _ => panic!("Expected OrderStatusRequest message"),
        }
    }

    #[test]
    fn test_parse_unknown_message() {
        let message = "35=Z|11=12345|";
        let parsed_message = FixMessageParser::parse_message(message);

        match parsed_message {
            FixMessage::Unknown => (),
            _ => panic!("Expected Unknown message"),
        }
    }
}