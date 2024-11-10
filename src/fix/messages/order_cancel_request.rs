use crate::fix::fix_message::{parse_field, parse_field_optional};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OrderCancelRequest {
    pub orig_cl_ord_id: String,   // Tag 41: Original client order ID
    pub cl_ord_id: String,        // Tag 11: Client's order ID
    pub side: char,               // Tag 54: Side of the order
    pub symbol: String,           // Tag 55: Ticker symbol
    pub transact_time: String,    // Tag 60: Transaction time
    pub order_qty: Option<f64>,   // Tag 38: Order quantity
}

impl OrderCancelRequest {
    pub fn new(fix_msg: HashMap<String, String>) -> Result<Self, String> {
        if fix_msg.get("35") != Some(&"F".to_string()) {
            return Err("Invalid message type".to_string());
        }

        Ok(OrderCancelRequest {
            orig_cl_ord_id: parse_field(&fix_msg, "41")?,
            cl_ord_id: parse_field(&fix_msg, "11")?,
            side: parse_field::<char>(&fix_msg, "54")?,
            symbol: parse_field(&fix_msg, "55")?,
            transact_time: parse_field(&fix_msg, "60")?,
            order_qty: parse_field_optional::<f64>(&fix_msg, "38")?,
        })
    }
}