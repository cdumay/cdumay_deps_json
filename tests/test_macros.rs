use std::collections::BTreeMap;
use serde_json::Value;
use cdumay_error_json::convert_result;
use cdumay_error::ErrorConverter;

#[test]
fn test_convert_result_with_context() {
    let result: Result<Value, serde_json::Error> = serde_json::from_str("invalid json");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));
    
    let converted = convert_result!(result, context, "Test error");
    assert!(converted.is_err());
    
    let err = converted.unwrap_err();
    assert_eq!(err.kind.message_id(), "JSON-00001");
    assert!(err.message.contains("Test error"));
}

#[test]
fn test_convert_result_without_text() {
    let result: Result<Value, serde_json::Error> = serde_json::from_str("invalid json");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));
    let converted = convert_result!(result, context);
    assert!(converted.is_err());
    
    let err = converted.unwrap_err();
    assert_eq!(err.kind.message_id(), "JSON-00001");
}

#[test]
fn test_convert_result_minimal() {
    let result: Result<Value, serde_json::Error> = serde_json::from_str("invalid json");
    let converted = convert_result!(result);
    assert!(converted.is_err());
    
    let err = converted.unwrap_err();
    assert_eq!(err.kind.message_id(), "JSON-00001");
}

#[test]
fn test_convert_result_success() {
    let result: Result<Value, serde_json::Error> = serde_json::from_str("{}");
    let converted = convert_result!(result);
    assert!(converted.is_ok());
}
