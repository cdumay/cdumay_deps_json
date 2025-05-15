#[macro_export]
macro_rules! convert_result {
    ($result:expr, $context:expr, $text:expr) => {
        $result.map_err(|err| {
            cdumay_error_json::JsonErrorConverter::convert_error(&err, Some($text.to_string()), $context)
        })
    };
    ($result:expr, $text:expr) => {
        $result.map_err(|err| {
            cdumay_error_json::JsonErrorConverter::convert_error(&err, Some($text.to_string()), std::collections::BTreeMap::new())
        })
    };
    ($result:expr) => {
        $result.map_err(|err| {
            cdumay_error_json::JsonErrorConverter::convert_error(&err, None, std::collections::BTreeMap::new())
        })
    };
}