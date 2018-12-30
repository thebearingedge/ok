use super::{
    error::Result,
    json::{Array, Json, JsonType},
    OkSchema, Validator,
};

pub struct ArraySchema {
    validator: Validator<Array>,
    description: Option<&'static str>,
}

impl ArraySchema {
    pub fn new() -> Self {
        ArraySchema {
            description: None,
            validator: Validator::new(JsonType::Array),
        }
    }
}

impl OkSchema for ArraySchema {
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

pub fn array() -> ArraySchema {
    ArraySchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{array, error, json::JsonType, OkSchema};
    use serde_json::json;

    #[test]
    fn it_validates_arrays() {
        let schema = array();

        assert_eq!(schema.validate(Some(json!([]))), Ok(Some(json!([]))));
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(error::type_error(JsonType::Array, JsonType::Null))
        );
        assert_eq!(
            schema.validate(None),
            Err(error::type_error(JsonType::Array, JsonType::None))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(error::type_error(JsonType::Array, JsonType::Object))
        );
        assert_eq!(
            schema.validate(Some(json!(1))),
            Err(error::type_error(JsonType::Array, JsonType::Number))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(error::type_error(JsonType::Array, JsonType::Boolean))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(error::type_error(JsonType::Array, JsonType::String))
        );
    }

    #[test]
    fn it_validates_optional_arrays() {
        let schema = array().optional();

        assert_eq!(schema.validate(Some(json!([]))), Ok(Some(json!([]))));
        assert_eq!(schema.validate(None), Ok(None));
    }

    #[test]
    fn it_validates_nullable_arrays() {
        let schema = array().nullable();

        assert_eq!(schema.validate(Some(json!([]))), Ok(Some(json!([]))));
        assert_eq!(schema.validate(Some(json!(null))), Ok(Some(json!(null))));
    }
}
