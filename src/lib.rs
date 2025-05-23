//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_json on crates.io](https://img.shields.io/crates/v/cdumay_error_json)](https://crates.io/crates/cdumay_error_json)
//! [![cdumay_error_json on docs.rs](https://docs.rs/cdumay_error_json/badge.svg)](https://docs.rs/cdumay_error_json)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_json)
//!
//! A utility crate that converts `serde_json::Error` into structured, typed errors using the [`cdumay_core`](https://docs.rs/cdumay_core/) framework. This ensures consistent error handling, easier debugging, and informative error reporting across your Rust applications.
//!
//! ## Features
//!
//! - Categorizes `serde_json::Error` into specific error types (`Syntax`, `IO`, `Data`, `EOF`)
//! - Each error type is associated with a custom code, HTTP status, and descriptive message
//! - Structured output for APIs, logging systems, and observability platforms
//! - Includes context metadata via `BTreeMap`
//! - Provides a convenient `convert_result!` macro for error conversion
//!
//! ## Usage
//!
//! Using the `JsonErrorConverter` directly:
//! ```rust
//! use cdumay_core::{Error, ErrorConverter};
//! use serde_json::Value;
//! use std::collections::BTreeMap;
//! use cdumay_error_json::JsonErrorConverter;
//!
//! fn parse_json(input: &str) -> Result<Value, Error> {
//!     serde_json::from_str::<Value>(input).map_err(|e| {
//!        let mut ctx = BTreeMap::new();
//!        ctx.insert("input".to_string(), serde_value::Value::String(input.to_string()));
//!        JsonErrorConverter::convert(&e, "Failed to parse JSON".to_string(), ctx)
//!    })
//! }
//! ```
//!
//! Using the `convert_result!` macro:
//! ```rust
//! use cdumay_error_json::convert_result;
//! use serde_json::Value;
//! use std::collections::BTreeMap;
//! use cdumay_core::{Error, ErrorConverter};
//!
//! fn parse_json(input: &str) -> Result<Value, Error> {
//!     // Basic usage with just the result
//!     convert_result!(serde_json::from_str::<Value>(input));
//!
//!     // With custom context
//!     let mut ctx = BTreeMap::new();
//!     ctx.insert("input".to_string(), serde_value::Value::String(input.to_string()));
//!     convert_result!(serde_json::from_str::<Value>(input), ctx.clone());
//!
//!     // With custom context and message
//!     convert_result!(serde_json::from_str::<Value>(input), ctx, "Failed to parse JSON")
//! }
//! ```
#[macro_use]
mod macros;

use cdumay_core::{Error, ErrorConverter, define_errors, define_kinds};
use serde_json::error::Category;
use std::collections::BTreeMap;

define_kinds! {
    JsonSyntax = (400, "Syntax Error"),
    JsonData = (400, "Invalid JSON data"),
    JsonEof = (500, "Reached the end of the input data"),
    JsonIo = (500, "IO Error"),
}

define_errors! {
    IoError = JsonIo,
    SyntaxError = JsonSyntax,
    DataError = JsonData,
    EofError = JsonEof
}

/// A utility struct for handling JSON errors and converting them into standardized error types.
pub struct JsonErrorConverter;

impl ErrorConverter for JsonErrorConverter {
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
            Category::Io => IoError::new().with_message(text).with_details(context).into(),
            Category::Syntax => SyntaxError::new().with_message(text).with_details(context).into(),
            Category::Data => DataError::new().with_message(text).with_details(context).into(),
            Category::Eof => EofError::new().with_message(text).with_details(context).into(),
        }
    }
}
