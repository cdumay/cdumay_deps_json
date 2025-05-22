use cdumay_core::ErrorConverter;
use cdumay_error_json::convert_result;
use serde_json::Value;
use std::collections::BTreeMap;

#[test]
fn test_convert_result_with_context() {
    let result: Result<Value, serde_json::Error> = serde_json::from_str("invalid json");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));

    let converted = convert_result!(result, context, "Test error");
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert!(err.message().contains("Test error"));
}

#[test]
fn test_convert_result_without_text() {
    let result: Result<Value, serde_json::Error> = serde_json::from_str("invalid json");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));
    let converted = convert_result!(result, context);
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert!(err.message().contains("expected value at"));
}

#[test]
fn test_convert_result_minimal() {
    let result: Result<Value, serde_json::Error> = serde_json::from_str("invalid json");
    let converted = convert_result!(result);
    assert!(converted.is_err());
}

#[test]
fn test_convert_result_success() {
    let result: Result<Value, serde_json::Error> = serde_json::from_str("{}");
    let converted = convert_result!(result);
    assert!(converted.is_ok());
}
