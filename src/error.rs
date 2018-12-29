use super::json::JsonType;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    Type(String),
    Value(String),
    Field(Vec<ValidationError>),
    Object(HashMap<String, ValidationError>),
}

pub type Result<T> = std::result::Result<T, ValidationError>;

pub fn type_error(expected: JsonType, actual: JsonType) -> ValidationError {
    ValidationError::Type(format!("Expected {}, but got {}.", expected, actual))
}

pub fn value_error<M: Into<String>>(message: M) -> ValidationError {
    ValidationError::Value(message.into())
}

pub fn field_error(errors: Vec<ValidationError>) -> ValidationError {
    ValidationError::Field(errors)
}

pub fn object_error(field_errors: HashMap<String, ValidationError>) -> ValidationError {
    ValidationError::Object(field_errors)
}
