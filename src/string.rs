use super::{
    error::{ValidationError, ValidationResult},
    json::{Json, JsonType},
    OkSchema, Validator,
};
use regex::Regex;

pub struct StringSchema {
    validator: Validator<String>,
}

impl StringSchema {
    pub fn new() -> Self {
        StringSchema {
            validator: Validator::new(JsonType::String),
        }
    }

    pub fn length(mut self, (min, max): (usize, usize)) -> Self {
        self.validator.add_test(
            "length",
            format!(
                "<label> must be between {} and {} characters long.",
                min, max
            ),
            move |string| Ok(string.len() >= min && string.len() <= max),
        );
        self
    }

    pub fn min_length(mut self, min: usize) -> Self {
        self.validator.add_test(
            "min_length",
            format!("<label> must be at least {} characters long.", min),
            move |string| Ok(string.len() >= min),
        );
        self
    }

    pub fn max_length(mut self, max: usize) -> Self {
        self.validator.add_test(
            "max_length",
            format!("<label> must be at most {} characters long.", max),
            move |string| Ok(string.len() <= max),
        );
        self
    }

    pub fn matches(mut self, pattern: &str) -> Self {
        let regex = Regex::new(pattern).unwrap();
        self.validator.add_test(
            "matches",
            format!("<label> must match the pattern '{}'.", regex.as_str()),
            move |string| Ok(regex.is_match(string)),
        );
        self
    }

    pub fn regex(mut self, regex: Regex) -> Self {
        self.validator.add_test(
            "matches",
            format!("<label> must match the pattern '{}'.", regex.as_str()),
            move |string| Ok(regex.is_match(string)),
        );
        self
    }

    pub fn trim(mut self) -> Self {
        self.validator
            .add_transform(|string| string.trim().to_string());
        self
    }

    pub fn uppercase(mut self) -> Self {
        self.validator
            .add_transform(|string| string.to_uppercase().to_string());
        self
    }

    pub fn lowercase(mut self) -> Self {
        self.validator
            .add_transform(|string| string.to_lowercase().to_string());
        self
    }
}

impl OkSchema for StringSchema {
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

pub fn string() -> StringSchema {
    StringSchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{
        error::{json_error, test_error, type_error},
        json::JsonType,
        string, OkSchema,
    };
    use regex::RegexBuilder;
    use serde_json::json;

    #[test]
    fn it_validates_strings() {
        let schema = string();
        assert_eq!(schema.validate(json!("foo")), Ok(json!("foo")));
        assert_eq!(schema.validate(json!(true)), Ok(json!("true")));
        assert_eq!(schema.validate(json!(1)), Ok(json!("1")));
        assert_eq!(
            schema.validate(json!(null)),
            Err(json_error(vec![type_error("", "", JsonType::String)]))
        );
        assert_eq!(
            schema.validate(json!([])),
            Err(json_error(vec![type_error("", "", JsonType::String)]))
        );
        assert_eq!(
            schema.validate(json!({})),
            Err(json_error(vec![type_error("", "", JsonType::String)]))
        );
    }

    #[test]
    fn it_sets_a_minimum_and_maximum_length() {
        let schema = string().label("My String").length((1, 3));
        assert_eq!(schema.validate(json!("foo")), Ok(json!("foo")));
        assert_eq!(
            schema.validate(json!("")),
            Err(json_error(vec![test_error(
                "length",
                "",
                "My String must be between 1 and 3 characters long."
            )]))
        );
        assert_eq!(
            schema.validate(json!("quux")),
            Err(json_error(vec![test_error(
                "length",
                "",
                "My String must be between 1 and 3 characters long."
            )]))
        );
    }

    #[test]
    fn it_sets_a_minimum_length() {
        let schema = string().label("My String").min_length(4);
        assert_eq!(schema.validate(json!("quux")), Ok(json!("quux")));
        assert_eq!(
            schema.validate(json!("qux")),
            Err(json_error(vec![test_error(
                "min_length",
                "",
                "My String must be at least 4 characters long."
            )]))
        );
    }

    #[test]
    fn it_sets_a_maximum_length() {
        let schema = string().label("My String").max_length(3);
        assert_eq!(schema.validate(json!("qux")), Ok(json!("qux")));
        assert_eq!(
            schema.validate(json!("quux")),
            Err(json_error(vec![test_error(
                "max_length",
                "",
                "My String must be at most 3 characters long."
            )]))
        );
    }

    #[test]
    fn it_sets_a_regex_matches() {
        let schema = string().label("My String").matches("(?i)^foo");
        assert_eq!(schema.validate(json!("Foobar")), Ok(json!("Foobar")));
        assert_eq!(
            schema.validate(json!("Barfoo")),
            Err(json_error(vec![test_error(
                "matches",
                "",
                "My String must match the pattern '(?i)^foo'."
            )]))
        )
    }

    #[test]
    fn it_sets_a_regex_object() {
        let regex = RegexBuilder::new("(?i)^foo").build().unwrap();
        let schema = string().label("My String").regex(regex);
        assert_eq!(schema.validate(json!("Foobar")), Ok(json!("Foobar")));
        assert_eq!(
            schema.validate(json!("Barfoo")),
            Err(json_error(vec![test_error(
                "matches",
                "",
                "My String must match the pattern '(?i)^foo'."
            )]))
        )
    }

    #[test]
    fn it_trims_strings() {
        let schema = string().trim();
        assert_eq!(schema.validate(json!("  foo  ")), Ok(json!("foo")));
    }

    #[test]
    fn it_uppercases_strings() {
        let schema = string().uppercase();
        assert_eq!(schema.validate(json!("foo")), Ok(json!("FOO")));
    }

    #[test]
    fn it_lowercases_strings() {
        let schema = string().lowercase();
        assert_eq!(schema.validate(json!("FOO")), Ok(json!("foo")));
    }

}
