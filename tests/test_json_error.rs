use std::collections::BTreeMap;
use cdumay_core::ErrorConverter;
use cdumay_error_json::JsonErrorConverter;

/// Helper to test error conversion logic.
fn test_error_conversion(input: &str, expected_kind: &'static str) {
    let ctx = BTreeMap::new();
    let result = serde_json::from_str::<serde_json::Value>(input);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let custom = JsonErrorConverter::convert_error(&err, Some("Test error".to_string()), ctx);

    assert_eq!(custom.message(), expected_kind);
}

#[test]
fn test_syntax_error() {
    // Invalid syntax: trailing comma
    test_error_conversion(r#"{"key": "value",}"#, "Test error");
}

#[test]
fn test_data_error() {
    // Data type mismatch: string expected, number provided
    #[derive(serde::Deserialize, Debug)]
    struct MyStruct {
        key: String,
    }

    let input = r#"{"key": 123}"#;
    let ctx = BTreeMap::new();
    let result = serde_json::from_str::<MyStruct>(input);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let custom = JsonErrorConverter::convert_error(&err, Some("Test data error".to_string()), ctx);
    assert_eq!(custom.message(), "Test data error");
}

#[test]
fn test_eof_error() {
    // Unexpected end of file/input
    test_error_conversion(r#"{"key": "value""#, "Test error");
}

#[test]
fn test_io_error_simulation() {
    // I/O errors are hard to simulate directly; here we simulate manually
    use std::io;

    let simulated_error = serde_json::Error::io(io::Error::new(io::ErrorKind::Other, "boom"));
    let ctx = BTreeMap::new();

    let custom = JsonErrorConverter::convert_error(&simulated_error, Some("Test IO error".to_string()), ctx);
    assert_eq!(custom.message(), "Test IO error");
}
