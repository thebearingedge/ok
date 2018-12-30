use super::{
    error::Result,
    json::{Json, JsonType},
    OkSchema, Validator,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct NumberSchema<T: Serialize + DeserializeOwned> {
    validator: Validator<T>,
    description: Option<&'static str>,
}

impl<T: Serialize + DeserializeOwned> NumberSchema<T> {
    pub fn new(json_type: JsonType) -> Self {
        NumberSchema {
            description: None,
            validator: Validator::new(json_type),
        }
    }
}

impl<T: Serialize + DeserializeOwned> OkSchema for NumberSchema<T> {
    fn desc(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    fn optional(mut self) -> Self {
        self.validator.is_optional = true;
        self
    }

    fn nullable(mut self) -> Self {
        self.validator.is_nullable = true;
        self
    }

    fn validate(&self, value: Option<Json>) -> Result<Option<Json>> {
        self.validator.exec(value)
    }
}

pub fn integer() -> NumberSchema<i64> {
    NumberSchema::new(JsonType::Integer)
}

pub fn float() -> NumberSchema<f64> {
    NumberSchema::new(JsonType::Float)
}

pub fn unsigned() -> NumberSchema<u64> {
    NumberSchema::new(JsonType::Unsigned)
}

#[cfg(test)]
mod tests {
    use super::super::{error, float, integer, json::JsonType, unsigned, OkSchema};
    use serde_json::json;

    #[test]
    fn it_validates_integers() {
        let schema = integer();

        assert_eq!(schema.validate(Some(json!(1))), Ok(Some(json!(1))));
        assert_eq!(schema.validate(Some(json!(1.0))), Ok(Some(json!(1))));
        assert_eq!(schema.validate(Some(json!("1"))), Ok(Some(json!(1))));
        assert_eq!(schema.validate(Some(json!(-1))), Ok(Some(json!(-1))));
        assert_eq!(schema.validate(Some(json!(-1.0))), Ok(Some(json!(-1))));
        assert_eq!(schema.validate(Some(json!("-1"))), Ok(Some(json!(-1))));
        assert_eq!(
            schema.validate(Some(json!(1.1))),
            Err(error::type_error(JsonType::Integer, JsonType::Float))
        );
        assert_eq!(
            schema.validate(Some(json!(std::i64::MAX as u64 + 1))),
            Err(error::type_error(JsonType::Integer, JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(error::type_error(JsonType::Integer, JsonType::Null))
        );
        assert_eq!(
            schema.validate(None),
            Err(error::type_error(JsonType::Integer, JsonType::None))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(error::type_error(JsonType::Integer, JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(error::type_error(JsonType::Integer, JsonType::Boolean))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(error::type_error(JsonType::Integer, JsonType::Object))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(error::type_error(JsonType::Integer, JsonType::String))
        );
    }

    #[test]
    fn it_validates_unsigned_integers() {
        let schema = unsigned();

        assert_eq!(schema.validate(Some(json!(1))), Ok(Some(json!(1))));
        assert_eq!(schema.validate(Some(json!(1.0))), Ok(Some(json!(1))));
        assert_eq!(schema.validate(Some(json!("1"))), Ok(Some(json!(1))));
        assert_eq!(
            schema.validate(Some(json!(1.1))),
            Err(error::type_error(JsonType::Unsigned, JsonType::Float))
        );
        assert_eq!(
            schema.validate(Some(json!(-1.0))),
            Err(error::type_error(JsonType::Unsigned, JsonType::Float))
        );
        assert_eq!(
            schema.validate(Some(json!(-1))),
            Err(error::type_error(JsonType::Unsigned, JsonType::Integer))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(error::type_error(JsonType::Unsigned, JsonType::Null))
        );
        assert_eq!(
            schema.validate(None),
            Err(error::type_error(JsonType::Unsigned, JsonType::None))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(error::type_error(JsonType::Unsigned, JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(error::type_error(JsonType::Unsigned, JsonType::Boolean))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(error::type_error(JsonType::Unsigned, JsonType::Object))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(error::type_error(JsonType::Unsigned, JsonType::String))
        );
    }

    #[test]
    fn it_validates_floats() {
        let schema = float();

        assert_eq!(schema.validate(Some(json!(1))), Ok(Some(json!(1.0))));
        assert_eq!(schema.validate(Some(json!(1.0))), Ok(Some(json!(1.0))));
        assert_eq!(schema.validate(Some(json!("1"))), Ok(Some(json!(1.0))));
        assert_eq!(schema.validate(Some(json!("1.0"))), Ok(Some(json!(1.0))));
        assert_eq!(schema.validate(Some(json!(-1))), Ok(Some(json!(-1.0))));
        assert_eq!(schema.validate(Some(json!(-1.0))), Ok(Some(json!(-1.0))));
        assert_eq!(schema.validate(Some(json!("-1"))), Ok(Some(json!(-1.0))));
        assert_eq!(schema.validate(Some(json!("-1.0"))), Ok(Some(json!(-1.0))));
        assert_eq!(
            schema.validate(Some(json!(std::i64::MAX as u64 + 1))),
            Err(error::type_error(JsonType::Float, JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(error::type_error(JsonType::Float, JsonType::Null))
        );
        assert_eq!(
            schema.validate(None),
            Err(error::type_error(JsonType::Float, JsonType::None))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(error::type_error(JsonType::Float, JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(error::type_error(JsonType::Float, JsonType::Boolean))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(error::type_error(JsonType::Float, JsonType::Object))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(error::type_error(JsonType::Float, JsonType::String))
        );
    }

    #[test]
    fn it_validates_optional_numbers() {
        let u = unsigned().optional();
        let i = integer().optional();
        let f = float().optional();

        assert_eq!(u.validate(None), Ok(None));
        assert_eq!(i.validate(None), Ok(None));
        assert_eq!(f.validate(None), Ok(None));
    }

    #[test]
    fn it_validates_nullable_numbers() {
        let u = unsigned().nullable();
        let i = integer().nullable();
        let f = float().nullable();

        assert_eq!(u.validate(Some(json!(null))), Ok(Some(json!(null))));
        assert_eq!(i.validate(Some(json!(null))), Ok(Some(json!(null))));
        assert_eq!(f.validate(Some(json!(null))), Ok(Some(json!(null))));
    }
}
