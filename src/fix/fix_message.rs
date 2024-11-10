use std::collections::HashMap;
use crate::fix::messages::execution_report::ExecutionReport;
use crate::fix::messages::new_order::NewOrder;
use crate::fix::messages::order_cancel_request::OrderCancelRequest;
use crate::fix::messages::order_status_request::OrderStatusRequest;

pub enum FixMessage {
    NewOrder(NewOrder),
    ExecutionReport(ExecutionReport),
    OrderCancelRequest(OrderCancelRequest),
    OrderStatusRequest(OrderStatusRequest),
    Unknown,
}

pub fn parse_field<T: std::str::FromStr>(fields: &HashMap<String, String>, tag: &str) -> Result<T, String> {
    fields.get(tag)
        .ok_or_else(|| {
            let error_message = format!("Missing Tag {}", tag);
            eprintln!("{}", error_message);
            error_message
        })
        .and_then(|value| value.parse().map_err(|_| {
            let error_message = format!("Invalid Tag {}", tag);
            eprintln!("{}", error_message);
            error_message
        }))
}

pub fn parse_field_optional<T: std::str::FromStr>(fields: &HashMap<String, String>, tag: &str) -> Result<Option<T>, String> {
    match fields.get(tag) {
        Some(value) => match value.parse() {
            Ok(parsed_value) => Ok(Some(parsed_value)),
            Err(_) => {
                let error_message = format!("Invalid Tag {}", tag);
                eprintln!("{}", error_message);
                Err(error_message)
            },
        },
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_field_success() {
        let mut fields = HashMap::new();
        fields.insert("11".to_string(), "12345".to_string());

        let result: Result<String, String> = parse_field(&fields, "11");
        assert_eq!(result.unwrap(), "12345");
    }

    #[test]
    fn test_parse_field_missing_tag() {
        let fields: HashMap<String, String> = HashMap::new();

        let result: Result<String, String> = parse_field(&fields, "11");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing Tag 11");
    }

    #[test]
    fn test_parse_field_invalid_tag() {
        let mut fields = HashMap::new();
        fields.insert("11".to_string(), "abc".to_string());

        let result: Result<i32, String> = parse_field(&fields, "11");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid Tag 11");
    }

    #[test]
    fn test_parse_field_optional_success() {
        let mut fields = HashMap::new();
        fields.insert("11".to_string(), "12345".to_string());

        let result: Result<Option<String>, String> = parse_field_optional(&fields, "11");
        assert_eq!(result.unwrap(), Some("12345".to_string()));
    }

    #[test]
    fn test_parse_field_optional_missing_tag() {
        let fields: HashMap<String, String> = HashMap::new();

        let result: Result<Option<String>, String> = parse_field_optional(&fields, "11");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_field_optional_invalid_tag() {
        let mut fields = HashMap::new();
        fields.insert("11".to_string(), "abc".to_string());

        let result: Result<Option<i32>, String> = parse_field_optional(&fields, "11");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid Tag 11");
    }
}