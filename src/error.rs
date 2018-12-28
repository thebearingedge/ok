use super::json::JsonType;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    Type(String),
    Value(String),
    Field(Vec<ValidationError>),
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
