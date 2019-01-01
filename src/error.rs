use super::json::JsonType;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    Type(String),
    Test(String),
    Field(Vec<ValidationError>),
    Object(HashMap<String, ValidationError>),
    Array(HashMap<usize, ValidationError>),
}

pub type Result<T> = std::result::Result<T, ValidationError>;

pub fn type_error<L: std::fmt::Display>(label: L, json_type: JsonType) -> ValidationError {
    ValidationError::Type(format!("{} must be of type `{}`.", label, json_type))
}

pub fn test_error<M: Into<String>>(message: M) -> ValidationError {
    ValidationError::Test(message.into())
}

pub fn field_error(errors: Vec<ValidationError>) -> ValidationError {
    ValidationError::Field(errors)
}

pub fn object_error(field_errors: HashMap<String, ValidationError>) -> ValidationError {
    ValidationError::Object(field_errors)
}

pub fn array_error(field_errors: HashMap<usize, ValidationError>) -> ValidationError {
    ValidationError::Array(field_errors)
}
