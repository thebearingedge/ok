use super::json::JsonType;

#[derive(Debug, PartialEq)]
pub struct ValidationError {
    path: String,
    message: String,
    type_: &'static str,
    inner: Vec<ValidationError>,
}

pub type Result<T> = std::result::Result<T, ValidationError>;

pub type ValidationResult<T> = std::result::Result<T, ()>;

pub fn type_error<L: std::fmt::Display>(
    path: &str,
    label: L,
    jsontype_: JsonType,
) -> ValidationError {
    ValidationError {
        path: path.into(),
        type_: "type_error",
        message: format!("{} must be of type `{}`.", label, jsontype_),
        inner: vec![],
    }
}

pub fn test_error<S: Into<String>>(type_: &'static str, path: S, message: S) -> ValidationError {
    ValidationError {
        type_,
        path: path.into(),
        message: message.into(),
        inner: vec![],
    }
}

pub fn payload_error(all_errors: Vec<ValidationError>) -> ValidationError {
    ValidationError {
        path: "".into(),
        inner: all_errors,
        type_: "invalid_json",
        message: "Validation failure.".into(),
    }
}
