use crate::fix_message::{parse_field, parse_field_optional};
use std::collections::HashMap;

#[derive(Debug)]
pub struct NewOrder {
    pub cl_ord_id: String,              // Tag 11: Unique ID for the order from the client
    pub side: char,                     // Tag 54: Side of the order (e.g., '1' for Buy, '2' for Sell)
    pub transact_time: String,          // Tag 60: Time the order was initiated
    pub order_qty: f64,                 // Tag 38: Quantity of the order
    pub ord_type: char,                 // Tag 40: Order type (e.g., '1' for Market, '2' for Limit)
    pub price: Option<f64>,             // Tag 44: Price for limit orders, optional for market orders
    pub symbol: String,                 // Tag 55: Ticker symbol for the instrument
    pub time_in_force: Option<char>,    // Tag 59: Duration of the order (e.g., '0' for Day)
    pub account: Option<String>,        // Tag 1: Optional account identifier
    pub currency: Option<String>,       // Tag 15: Currency of the order
    pub exec_inst: Option<String>,      // Tag 18: Execution instructions, if applicable
}

impl NewOrder {
    pub fn new(fix_msg: HashMap<String, String>) -> Result<Self, String> {
        if fix_msg.get("35") != Some(&"D".to_string()) {
            return Err("Invalid message type".to_string());
        }

        Ok(NewOrder {
            cl_ord_id: parse_field(&fix_msg, "11")?,
            side: parse_field::<char>(&fix_msg, "54")?,
            transact_time: parse_field(&fix_msg, "60")?,
            order_qty: parse_field::<f64>(&fix_msg, "38")?,
            ord_type: parse_field::<char>(&fix_msg, "40")?,
            price: parse_field_optional::<f64>(&fix_msg, "44")?,
            symbol: parse_field(&fix_msg, "55")?,
            time_in_force: parse_field_optional::<char>(&fix_msg, "59")?,
            account: parse_field_optional(&fix_msg, "1")?,
            currency: parse_field_optional(&fix_msg, "15")?,
            exec_inst: parse_field_optional(&fix_msg, "18")?,
        })
    }
}