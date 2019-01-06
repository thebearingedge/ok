use super::{
    error::{ValidationError, ValidationResult},
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

    fn validate_at(
        &self,
        path: &str,
        value: Option<Json>,
        all_errors: &mut Vec<ValidationError>,
    ) -> ValidationResult<Option<Json>> {
        self.validator.exec(path, value, all_errors)
    }
}

pub fn boolean() -> BooleanSchema {
    BooleanSchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{
        boolean,
        error::{json_error, type_error},
        json::JsonType,
        OkSchema,
    };
    use serde_json::json;

    #[test]
    fn it_validates_booleans() {
        let schema = boolean();
        assert_eq!(schema.validate(json!(true)), Ok(json!(true)));
        assert_eq!(schema.validate(json!(false)), Ok(json!(false)));
        assert_eq!(schema.validate(json!("true")), Ok(json!(true)));
        assert_eq!(schema.validate(json!("false")), Ok(json!(false)));
        assert_eq!(
            schema.validate(json!(null)),
            Err(json_error(vec![type_error("", "", JsonType::Boolean)]))
        );
        assert_eq!(
            schema.validate(json!([])),
            Err(json_error(vec![type_error("", "", JsonType::Boolean)]))
        );
        assert_eq!(
            schema.validate(json!(1)),
            Err(json_error(vec![type_error("", "", JsonType::Boolean)]))
        );
        assert_eq!(
            schema.validate(json!({})),
            Err(json_error(vec![type_error("", "", JsonType::Boolean)]))
        );
        assert_eq!(
            schema.validate(json!("foo")),
            Err(json_error(vec![type_error("", "", JsonType::Boolean)]))
        );
    }
}
