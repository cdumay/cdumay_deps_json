//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_json on crates.io](https://img.shields.io/crates/v/cdumay_error_json)](https://crates.io/crates/cdumay_error_json)
//! [![cdumay_error_json on docs.rs](https://docs.rs/cdumay_error_json/badge.svg)](https://docs.rs/cdumay_error_json)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_json)
//!
use cdumay_context::Context;
use cdumay_error::{define_errors, define_kinds, AsError, Error};
use serde_json::error::Category;

define_kinds! {
    JsonSyntax = ("JSON-00001", 400, "Syntax Error"),
    JsonData = ("JSON-00002", 400, "Invalid JSON data"),
    JsonEof = ("JSON-00003", 500, "Reached the end of the input data"),
    JsonIo = ("JSON-00004", 500, "Syntax Error"),
}

define_errors! {
    IoError = JsonIo,
    SyntaxError = JsonSyntax,
    DataError = JsonData,
    EofError = JsonEof
}

pub struct JsonError;
impl JsonError {
    pub fn json_error(err: &serde_json::Error, text: String, context: &mut Context) -> Error {
        match err.classify() {
            Category::Io => IoError::new().set_message(text).set_details(context.into()).into(),
            Category::Syntax => SyntaxError::new().set_message(text).set_details(context.into()).into(),
            Category::Data => DataError::new().set_message(text).set_details(context.into()).into(),
            Category::Eof => EofError::new().set_message(text).set_details(context.into()).into(),
        }
    }
}
