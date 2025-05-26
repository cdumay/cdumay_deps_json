/// Macro to convert a [`Result<T, serde_json::Error>`] into a [`cdumay_core::Result<T>`]
#[macro_export]
macro_rules! convert_result {
    ($result:expr, $context:expr, $text:expr) => {
        $result.map_err(|err| {
            cdumay_error_json::JsonErrorConverter::convert_error(&err, Some($text.to_string()), $context)
        })
    };
    ($result:expr, $context:expr) => {
        $result.map_err(|err| {
            cdumay_error_json::JsonErrorConverter::convert_error(&err, None, $context)
        })
    };
    ($result:expr) => {
        $result.map_err(|err| {
            cdumay_error_json::JsonErrorConverter::convert_error(&err, None, std::collections::BTreeMap::new())
        })
    };
}