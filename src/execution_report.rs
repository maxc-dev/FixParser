use crate::fix_message::{parse_field, parse_field_optional};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ExecutionReport {
    pub cl_ord_id: String,        // Tag 11: Client's order ID
    pub order_id: String,         // Tag 37: Unique identifier of the order assigned by the broker
    pub exec_id: String,          // Tag 17: Execution ID
    pub exec_type: char,          // Tag 150: Type of execution (e.g., New, Fill)
    pub ord_status: char,         // Tag 39: Order status (e.g., '0' for New, '2' for Filled)
    pub side: char,               // Tag 54: Side of the order
    pub leaves_qty: f64,          // Tag 151: Remaining quantity
    pub cum_qty: f64,             // Tag 14: Cumulative quantity filled
    pub avg_px: Option<f64>,      // Tag 6: Average fill price
    pub symbol: String,           // Tag 55: Ticker symbol
    pub transact_time: String,    // Tag 60: Execution transaction time
    pub order_qty: f64,           // Tag 38: Quantity of the order
    pub price: f64,               // Tag 44: Price of the order
}

impl ExecutionReport {
    pub fn new(fix_msg: HashMap<String, String>) -> Result<Self, String> {
        if fix_msg.get("35") != Some(&"8".to_string()) {
            return Err("Invalid message type".to_string());
        }

        Ok(ExecutionReport {
            cl_ord_id: parse_field(&fix_msg, "11")?,
            order_id: parse_field(&fix_msg, "37")?,
            exec_id: parse_field(&fix_msg, "17")?,
            exec_type: parse_field::<char>(&fix_msg, "150")?,
            ord_status: parse_field::<char>(&fix_msg, "39")?,
            side: parse_field::<char>(&fix_msg, "54")?,
            leaves_qty: parse_field::<f64>(&fix_msg, "151")?,
            cum_qty: parse_field::<f64>(&fix_msg, "14")?,
            avg_px: parse_field_optional::<f64>(&fix_msg, "6")?,
            symbol: parse_field(&fix_msg, "55")?,
            transact_time: parse_field(&fix_msg, "60")?,
            order_qty: parse_field::<f64>(&fix_msg, "38")?,
            price: parse_field::<f64>(&fix_msg, "44")?,
        })
    }
}