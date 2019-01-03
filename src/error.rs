use super::json::JsonType;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct ValidationError {
    path: String,
    message: String,
    #[serde(rename = "type")]
    type_: &'static str,
    errors: Vec<ValidationError>,
}

pub type Result<T> = std::result::Result<T, ValidationError>;

pub type ValidationResult<T> = std::result::Result<T, ()>;

pub fn type_error<L: std::fmt::Display>(
    path: &str,
    label: L,
    json_type: JsonType,
) -> ValidationError {
    ValidationError {
        path: path.into(),
        type_: "type_error",
        message: format!("{} must be of type `{}`.", label, json_type),
        errors: vec![],
    }
}

pub fn test_error<S: Into<String>>(type_: &'static str, path: S, message: S) -> ValidationError {
    ValidationError {
        type_,
        path: path.into(),
        message: message.into(),
        errors: vec![],
    }
}

pub fn json_error(all_errors: Vec<ValidationError>) -> ValidationError {
    let error_count = all_errors.len();
    let pluralized = if error_count == 1 { "error" } else { "errors" };
    let message = format!("{} validation {} occurred.", error_count, pluralized);
    ValidationError {
        message,
        path: "".into(),
        errors: all_errors,
        type_: "invalid_json",
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        error::{json_error, test_error, type_error},
        json::{to_json, JsonType},
    };
    use serde_json::json;

    #[test]
    fn it_serializes_type_errors() {
        let err = type_error("foo", "My Boolean", JsonType::Boolean);
        assert_eq!(
            to_json(err).unwrap(),
            json!({
                "type": "type_error",
                "path": "foo",
                "message": "My Boolean must be of type `Boolean`.",
                "errors": []
            })
        );
    }

    #[test]
    fn it_serializes_test_errors() {
        let err = test_error("no_good", "foo", "Validation failed for foo!");
        assert_eq!(
            to_json(err).unwrap(),
            json!({
                "type": "no_good",
                "path": "foo",
                "message": "Validation failed for foo!",
                "errors": []
            })
        );
    }

    #[test]
    fn it_serializes_json_errors() {
        let err = json_error(vec![type_error("foo", "My Boolean", JsonType::Boolean)]);
        assert_eq!(
            to_json(err).unwrap(),
            json!({
                "type": "invalid_json",
                "path": "",
                "message": "1 validation error occurred.",
                "errors": [{
                    "type": "type_error",
                    "path": "foo",
                    "message": "My Boolean must be of type `Boolean`.",
                    "errors": []
                }]
            })
        );
    }
}
