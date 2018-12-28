use super::error::{self, Result};
pub use serde_json::{from_value as from_json, to_value as to_json, Value as Json};

#[derive(Copy, Clone)]
pub enum JsonType {
    Array,
    Boolean,
    Float,
    Integer,
    None,
    Null,
    Number,
    Object,
    String,
}

impl JsonType {
    pub fn coerce(&self, json: Json) -> Result<Json> {
        match self {
            JsonType::Boolean => {
                if json.is_boolean() {
                    return Ok(from_json(json).unwrap());
                }
                if json.is_string() {
                    return match json.as_str().unwrap() {
                        "true" => Ok(Json::Bool(true)),
                        "false" => Ok(Json::Bool(false)),
                        _ => Err(error::type_error(JsonType::Boolean, JsonType::String)),
                    };
                }
                Err(error::type_error(JsonType::Boolean, (&Some(json)).into()))
            }
            _ => Ok(from_json(json).unwrap()),
        }
    }
}

impl From<&Option<Json>> for JsonType {
    fn from(value: &Option<Json>) -> JsonType {
        match value {
            Some(json) => match json {
                Json::Array(_) => JsonType::Array,
                Json::Bool(_) => JsonType::Boolean,
                Json::Number(_) => JsonType::Number,
                Json::Object(_) => JsonType::Object,
                Json::String(_) => JsonType::String,
                Json::Null => JsonType::Null,
            },
            None => JsonType::None,
        }
    }
}

impl std::fmt::Display for JsonType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            JsonType::Array => write!(f, "Array"),
            JsonType::Boolean => write!(f, "Boolean"),
            JsonType::Float => write!(f, "Float"),
            JsonType::Integer => write!(f, "Integer"),
            JsonType::None => write!(f, "none"),
            JsonType::Null => write!(f, "null"),
            JsonType::Number => write!(f, "Number"),
            JsonType::Object => write!(f, "Object"),
            JsonType::String => write!(f, "String"),
        }
    }
}
