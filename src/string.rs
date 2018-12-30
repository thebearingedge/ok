use super::{
    error::Result,
    json::{Json, JsonType},
    OkSchema, Validator,
};

pub struct StringSchema {
    validator: Validator<String>,
    description: Option<&'static str>,
}

impl StringSchema {
    pub fn new() -> Self {
        StringSchema {
            description: None,
            validator: Validator::new(JsonType::String),
        }
    }
}

impl OkSchema for StringSchema {
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

pub fn string() -> StringSchema {
    StringSchema::new()
}

mod tests {
    use super::super::{error, json::JsonType, string, OkSchema};
    use serde_json::json;

    #[test]
    fn it_validates_strings() {
        let schema = string();

        assert_eq!(schema.validate(Some(json!("foo"))), Ok(Some(json!("foo"))));
        assert_eq!(schema.validate(Some(json!(true))), Ok(Some(json!("true"))));
        assert_eq!(schema.validate(Some(json!(1))), Ok(Some(json!("1"))));
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(error::type_error(JsonType::String, JsonType::Null))
        );
        assert_eq!(
            schema.validate(None),
            Err(error::type_error(JsonType::String, JsonType::None))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(error::type_error(JsonType::String, JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(error::type_error(JsonType::String, JsonType::Object))
        );
    }

    #[test]
    fn it_validates_optional_strings() {
        let schema = string().optional();

        assert_eq!(schema.validate(Some(json!("foo"))), Ok(Some(json!("foo"))));
        assert_eq!(schema.validate(None), Ok(None));
    }

    #[test]
    fn it_validates_nullable_strings() {
        let schema = string().nullable();

        assert_eq!(schema.validate(Some(json!("foo"))), Ok(Some(json!("foo"))));
        assert_eq!(schema.validate(Some(json!(null))), Ok(Some(json!(null))));
    }
}
