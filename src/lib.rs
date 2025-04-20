//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_json on crates.io](https://img.shields.io/crates/v/cdumay_error_json)](https://crates.io/crates/cdumay_error_json)
//! [![cdumay_error_json on docs.rs](https://docs.rs/cdumay_error_json/badge.svg)](https://docs.rs/cdumay_error_json)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_json)
//!
//! This crate provides standardized error handling for JSON operations,
//! mapping `serde_json::Error` categories to custom error types with
//! associated codes and HTTP status codes.

use cdumay_error::{AsError, Error, define_errors, define_kinds, ErrorConverter};
use serde_json::error::Category;
use std::collections::BTreeMap;

/// Define custom error kinds with unique codes, HTTP status codes, and descriptions.
define_kinds! {
    JsonSyntax = ("JSON-00001", 400, "Syntax Error"),
    JsonData = ("JSON-00002", 400, "Invalid JSON data"),
    JsonEof = ("JSON-00003", 500, "Reached the end of the input data"),
    JsonIo = ("JSON-00004", 500, "Syntax Error"),
}

/// Define error types corresponding to the previously defined error kinds.
define_errors! {
    IoError = JsonIo,
    SyntaxError = JsonSyntax,
    DataError = JsonData,
    EofError = JsonEof
}

/// A utility struct for handling JSON errors and converting them into standardized error types.
pub struct JsonError;

impl ErrorConverter for JsonError {
    type Error = serde_json::Error;
    /// Converts a `serde_json::Error` into a standardized `Error` type based on its category.
    ///
    /// # Arguments
    ///
    /// * `err` - The `serde_json::Error` to be converted.
    /// * `text` - A descriptive message for the error.
    /// * `context` - A mutable reference to a `BTreeMap` containing additional error details.
    ///
    /// # Returns
    ///
    /// A standardized `Error` instance corresponding to the category of the provided `serde_json::Error`.
    fn convert(err: &serde_json::Error, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        match err.classify() {
            Category::Io => IoError::new().set_message(text).set_details(context).into(),
            Category::Syntax => SyntaxError::new().set_message(text).set_details(context).into(),
            Category::Data => DataError::new().set_message(text).set_details(context).into(),
            Category::Eof => EofError::new().set_message(text).set_details(context).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to test error conversion logic.
    fn test_error_conversion(input: &str, expected_kind: &'static str) {
        let ctx = BTreeMap::new();
        let result = serde_json::from_str::<serde_json::Value>(input);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let custom = JsonError::convert_error(&err, Some("Test error".to_string()), ctx);

        assert_eq!(custom.kind.message_id(), expected_kind);
    }

    #[test]
    fn test_syntax_error() {
        // Invalid syntax: trailing comma
        test_error_conversion(r#"{"key": "value",}"#, "JSON-00001");
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
        let custom = JsonError::convert_error(&err, Some("Test data error".to_string()), ctx);
        assert_eq!(custom.kind.message_id(), "JSON-00002");
    }

    #[test]
    fn test_eof_error() {
        // Unexpected end of file/input
        test_error_conversion(r#"{"key": "value""#, "JSON-00003");
    }

    #[test]
    fn test_io_error_simulation() {
        // I/O errors are hard to simulate directly; here we simulate manually
        use std::io;

        let simulated_error = serde_json::Error::io(io::Error::new(io::ErrorKind::Other, "boom"));
        let ctx = BTreeMap::new();

        let custom = JsonError::convert_error(&simulated_error, Some("Test IO error".to_string()), ctx);
        assert_eq!(custom.kind.message_id(), "JSON-00004");
    }
}
