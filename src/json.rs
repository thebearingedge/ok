use super::error::{self, Result};
pub use serde_json::{
    from_value as from_json, map::Map, to_string, to_value as to_json, Value as Json,
};

pub type Object = Map<String, Json>;

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
    Unsigned,
}

impl JsonType {
    pub fn coerce(&self, json: Json) -> Result<Json> {
        match self {
            JsonType::Boolean => {
                if json.is_boolean() {
                    return Ok(json);
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
            JsonType::Object => {
                if json.is_object() {
                    return Ok(json);
                }
                Err(error::type_error(JsonType::Object, (&Some(json)).into()))
            }
            JsonType::Integer => {
                if json.is_i64() {
                    return Ok(json);
                }
                if json.is_f64() {
                    let float = json.as_f64().unwrap();
                    if float.fract() == 0.0 {
                        return Ok(to_json::<i64>(float as i64).unwrap());
                    }
                    return Err(error::type_error(JsonType::Integer, JsonType::Float));
                }
                if json.is_u64() {
                    let unsigned = json.as_u64().unwrap();
                    if unsigned <= std::i64::MAX as u64 {
                        return Ok(json);
                    }
                    return Err(error::type_error(JsonType::Integer, JsonType::Unsigned));
                }
                if json.is_string() {
                    let string = json.as_str().unwrap();
                    return match string.parse::<i64>() {
                        Ok(integer) => Ok(to_json(integer).unwrap()),
                        Err(_) => Err(error::type_error(JsonType::Integer, JsonType::String)),
                    };
                }
                Err(error::type_error(JsonType::Integer, (&Some(json)).into()))
            }
            JsonType::Unsigned => {
                if json.is_u64() {
                    return Ok(json);
                }
                if json.is_f64() {
                    let float = json.as_f64().unwrap();
                    if float >= 0.0 && float.fract() == 0.0 {
                        return Ok(to_json::<u64>(float as u64).unwrap());
                    }
                    return Err(error::type_error(JsonType::Unsigned, JsonType::Float));
                }
                if json.is_i64() {
                    let integer = json.as_i64().unwrap();
                    if integer >= 0 {
                        return Ok(to_json::<u64>(integer as u64).unwrap());
                    }
                    return Err(error::type_error(JsonType::Unsigned, JsonType::Integer));
                }
                if json.is_string() {
                    let string = json.as_str().unwrap();
                    return match string.parse::<u64>() {
                        Ok(unsigned) => Ok(to_json(unsigned).unwrap()),
                        Err(_) => Err(error::type_error(JsonType::Unsigned, JsonType::String)),
                    };
                }
                Err(error::type_error(JsonType::Unsigned, (&Some(json)).into()))
            }
            JsonType::Float => {
                if json.is_f64() || json.is_i64() {
                    return Ok(json);
                }
                if json.is_u64() {
                    let unsigned = json.as_u64().unwrap();
                    if unsigned <= std::f64::MAX as u64 {
                        return Ok(to_json::<f64>(unsigned as f64).unwrap());
                    }
                    return Err(error::type_error(JsonType::Float, JsonType::Unsigned));
                }
                if json.is_string() {
                    let string = json.as_str().unwrap();
                    return match string.parse::<f64>() {
                        Ok(float) => Ok(to_json(float).unwrap()),
                        Err(_) => Err(error::type_error(JsonType::Float, JsonType::String)),
                    };
                }
                Err(error::type_error(JsonType::Float, (&Some(json)).into()))
            }
            JsonType::String => {
                if json.is_string() {
                    return Ok(json);
                }
                if json.is_boolean() || json.is_number() {
                    return Ok(to_json(json.to_string()).unwrap());
                }
                Err(error::type_error(JsonType::String, (&Some(json)).into()))
            }
            _ => Ok(json),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            JsonType::Array => "Array",
            JsonType::Boolean => "Boolean",
            JsonType::Float => "Float",
            JsonType::Integer => "Integer",
            JsonType::None => "none",
            JsonType::Null => "null",
            JsonType::Number => "Number",
            JsonType::Object => "Object",
            JsonType::String => "String",
            JsonType::Unsigned => "Unsigned Integer",
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
        write!(f, "{}", self.as_str())
    }
}
