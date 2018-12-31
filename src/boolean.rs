use super::{
    error::Result,
    json::{Json, JsonType},
    OkSchema, Validator,
};

pub struct BooleanSchema {
    validator: Validator<bool>,
}

impl BooleanSchema {
    pub fn new() -> Self {
        BooleanSchema {
            validator: Validator::new(JsonType::Boolean),
        }
    }
}

impl OkSchema for BooleanSchema {
    fn label(mut self, label: &'static str) -> Self {
        self.validator.label = Some(label);
        self
    }

    fn desc(mut self, description: &'static str) -> Self {
        self.validator.description = Some(description);
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

pub fn boolean() -> BooleanSchema {
    BooleanSchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{boolean, error, json::JsonType, OkSchema};
    use serde_json::json;

    #[test]
    fn it_validates_booleans() {
        let schema = boolean();
        assert_eq!(schema.validate(Some(json!(true))), Ok(Some(json!(true))));
        assert_eq!(schema.validate(Some(json!(false))), Ok(Some(json!(false))));
        assert_eq!(schema.validate(Some(json!("true"))), Ok(Some(json!(true))));
        assert_eq!(
            schema.validate(Some(json!("false"))),
            Ok(Some(json!(false)))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(error::type_error(JsonType::Boolean, JsonType::Null))
        );
        assert_eq!(
            schema.validate(None),
            Err(error::type_error(JsonType::Boolean, JsonType::None))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(error::type_error(JsonType::Boolean, JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!(1))),
            Err(error::type_error(JsonType::Boolean, JsonType::Number))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(error::type_error(JsonType::Boolean, JsonType::Object))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(error::type_error(JsonType::Boolean, JsonType::String))
        );
    }

    #[test]
    fn it_validates_optional_booleans() {
        let schema = boolean().optional();
        assert_eq!(schema.validate(Some(json!(true))), Ok(Some(json!(true))));
        assert_eq!(schema.validate(None), Ok(None));
    }

    #[test]
    fn it_validates_nullable_booleans() {
        let schema = boolean().nullable();
        assert_eq!(schema.validate(Some(json!(true))), Ok(Some(json!(true))));
        assert_eq!(schema.validate(Some(json!(null))), Ok(Some(json!(null))));
    }
}
