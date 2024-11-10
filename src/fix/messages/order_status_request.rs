use crate::fix::fix_message::parse_field;
use std::collections::HashMap;

#[derive(Debug)]
pub struct OrderStatusRequest {
    pub cl_ord_id: String,        // Tag 11: Client's order ID
    pub symbol: String,           // Tag 55: Ticker symbol
    pub side: char,               // Tag 54: Side of the order
    pub transact_time: String,    // Tag 60: Transaction time
}

impl OrderStatusRequest {
    pub fn new(fix_msg: HashMap<String, String>) -> Result<Self, String> {
        if fix_msg.get("35") != Some(&"H".to_string()) {
            return Err("Invalid message type".to_string());
        }

        Ok(OrderStatusRequest {
            cl_ord_id: parse_field(&fix_msg, "11")?,
            symbol: parse_field(&fix_msg, "55")?,
            side: parse_field::<char>(&fix_msg, "54")?,
            transact_time: parse_field(&fix_msg, "60")?,
        })
    }
}