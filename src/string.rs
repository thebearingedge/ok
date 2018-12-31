use super::{
    error::{self, Result},
    json::{Json, JsonType},
    OkSchema, Validator,
};
use regex::Regex;

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

    pub fn length(mut self, (minimum, maximum): (usize, usize)) -> Self {
        self.validator.append(move |string| {
            if string.len() >= minimum && string.len() <= maximum {
                return Ok(None);
            }
            Err(error::value_error(format!(
                "Expected String with length between {} and {}.",
                minimum, maximum
            )))
        });
        self
    }

    pub fn min_length(mut self, minimum: usize) -> Self {
        self.validator.append(move |string| {
            if string.len() >= minimum {
                return Ok(None);
            }
            Err(error::value_error(format!(
                "Expected String with length of at least {}.",
                minimum
            )))
        });
        self
    }

    pub fn max_length(mut self, maximum: usize) -> Self {
        self.validator.append(move |string| {
            if string.len() <= maximum {
                return Ok(None);
            }
            Err(error::value_error(format!(
                "Expected String with length of at most {}.",
                maximum
            )))
        });
        self
    }

    pub fn trim(mut self) -> Self {
        self.validator
            .append(|string| Ok(Some(string.trim().to_string())));
        self
    }

    pub fn uppercase(mut self) -> Self {
        self.validator
            .append(|string| Ok(Some(string.to_uppercase().to_string())));
        self
    }

    pub fn lowercase(mut self) -> Self {
        self.validator
            .append(|string| Ok(Some(string.to_lowercase().to_string())));
        self
    }

    pub fn pattern(mut self, pattern: &'static str) -> Self {
        let regex = Regex::new(pattern).unwrap();
        self.validator.append(move |string| {
            if regex.is_match(string) {
                return Ok(None);
            }
            Err(error::value_error(format!(
                "Expected String to match pattern '{}'.",
                regex.as_str()
            )))
        });
        self
    }

    pub fn regex(mut self, regex: Regex) -> Self {
        self.validator.append(move |string| {
            if regex.is_match(string) {
                return Ok(None);
            }
            Err(error::value_error(format!(
                "Expected String to match pattern '{}'.",
                regex.as_str()
            )))
        });
        self
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

#[cfg(test)]
mod tests {
    use super::super::{error, json::JsonType, string, OkSchema};
    use regex::RegexBuilder;
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

    #[test]
    fn it_sets_a_minimum_and_maximum_length() {
        let schema = string().length((1, 3));
        assert_eq!(schema.validate(Some(json!("foo"))), Ok(Some(json!("foo"))));
        assert_eq!(
            schema.validate(Some(json!(""))),
            Err(error::field_error(vec![error::value_error(
                "Expected String with length between 1 and 3."
            )]))
        );
        assert_eq!(
            schema.validate(Some(json!("quux"))),
            Err(error::field_error(vec![error::value_error(
                "Expected String with length between 1 and 3."
            )]))
        );
    }

    #[test]
    fn it_sets_a_minimum_length() {
        let schema = string().min_length(4);
        assert_eq!(
            schema.validate(Some(json!("quux"))),
            Ok(Some(json!("quux")))
        );
        assert_eq!(
            schema.validate(Some(json!("qux"))),
            Err(error::field_error(vec![error::value_error(
                "Expected String with length of at least 4."
            )]))
        );
    }

    #[test]
    fn it_sets_a_maximum_length() {
        let schema = string().max_length(3);
        assert_eq!(schema.validate(Some(json!("qux"))), Ok(Some(json!("qux"))));
        assert_eq!(
            schema.validate(Some(json!("quux"))),
            Err(error::field_error(vec![error::value_error(
                "Expected String with length of at most 3."
            )]))
        );
    }

    #[test]
    fn it_trims_strings() {
        let schema = string().trim();
        assert_eq!(
            schema.validate(Some(json!("  foo  "))),
            Ok(Some(json!("foo")))
        );
    }

    #[test]
    fn it_uppercases_strings() {
        let schema = string().uppercase();
        assert_eq!(schema.validate(Some(json!("foo"))), Ok(Some(json!("FOO"))));
    }

    #[test]
    fn it_lowercases_strings() {
        let schema = string().lowercase();
        assert_eq!(schema.validate(Some(json!("FOO"))), Ok(Some(json!("foo"))));
    }

    #[test]
    fn it_sets_a_regex_pattern() {
        let schema = string().pattern("(?i)^foo");
        assert_eq!(
            schema.validate(Some(json!("Foobar"))),
            Ok(Some(json!("Foobar")))
        );
        assert_eq!(
            schema.validate(Some(json!("Barfoo"))),
            Err(error::field_error(vec![error::value_error(
                "Expected String to match pattern '(?i)^foo'."
            )]))
        )
    }

    #[test]
    fn it_sets_a_regex_object() {
        let regex = RegexBuilder::new("(?i)^foo").build().unwrap();
        let schema = string().regex(regex);
        assert_eq!(
            schema.validate(Some(json!("Foobar"))),
            Ok(Some(json!("Foobar")))
        );
        assert_eq!(
            schema.validate(Some(json!("Barfoo"))),
            Err(error::field_error(vec![error::value_error(
                "Expected String to match pattern '(?i)^foo'."
            )]))
        )
    }
}
