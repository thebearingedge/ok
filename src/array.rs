use super::{
    error::{array_error, Result},
    json::{from_json, Array, Json, JsonType},
    OkSchema, Validator,
};
use std::collections::HashMap;

pub struct ArraySchema {
    validator: Validator<Array>,
    element_schema: Option<Box<OkSchema>>,
}

impl ArraySchema {
    pub fn new() -> Self {
        ArraySchema {
            validator: Validator::new(JsonType::Array),
            element_schema: None,
        }
    }

    pub fn length(mut self, (min, max): (usize, usize)) -> Self {
        self.validator.add_test(
            format!("Expected Array with length between {} and {}.", min, max),
            move |array| Ok(array.len() >= min && array.len() <= max),
        );
        self
    }

    pub fn min_length(mut self, min: usize) -> Self {
        self.validator.add_test(
            format!("Expected Array with length of at least {}.", min),
            move |array| Ok(array.len() >= min),
        );
        self
    }

    pub fn max_length(mut self, max: usize) -> Self {
        self.validator.add_test(
            format!("Expected Array with length of at most {}.", max),
            move |array| Ok(array.len() <= max),
        );
        self
    }

    pub fn of(mut self, schema: impl OkSchema + 'static) -> Self {
        self.element_schema = Some(Box::new(schema));
        self
    }
}

impl OkSchema for ArraySchema {
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

    fn validate_at(&self, path: &str, value: Option<Json>) -> Result<Option<Json>> {
        let validated = self.validator.exec(path, value)?;
        let elements = match validated {
            None => return Ok(None),
            Some(_) if self.element_schema.is_none() => return Ok(validated),
            Some(json) => from_json::<Array>(json).unwrap(),
        };
        let mut array = vec![];
        let mut errors = HashMap::new();
        let element_schema = self.element_schema.as_ref().unwrap();
        elements
            .into_iter()
            .enumerate()
            .for_each(|(index, element)| {
                let path = format!("{}[{}]", path, index);
                match element_schema.validate_at(path.as_str(), Some(element)) {
                    Ok(value) => {
                        if errors.is_empty() {
                            array.push(value.unwrap());
                        }
                    }
                    Err(error) => {
                        errors.insert(index, error);
                    }
                };
            });
        if errors.is_empty() {
            return Ok(Some(array.into()));
        }
        Err(array_error(errors))
    }
}

pub fn array() -> ArraySchema {
    ArraySchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{
        array, boolean,
        error::{array_error, field_error, test_error, type_error},
        integer,
        json::JsonType,
        object, string, OkSchema,
    };
    use maplit::hashmap;
    use serde_json::json;

    #[test]
    fn it_validates_arrays() {
        let schema = array();
        assert_eq!(schema.validate(Some(json!([]))), Ok(Some(json!([]))));
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(type_error("", JsonType::Array))
        );
        assert_eq!(schema.validate(None), Err(type_error("", JsonType::Array)));
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(type_error("", JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!(1))),
            Err(type_error("", JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(type_error("", JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(type_error("", JsonType::Array))
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

    #[test]
    fn it_sets_a_minimum_and_maximum_length() {
        let schema = array().length((1, 3));
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz"]))),
            Ok(Some(json!(["foo", "bar", "baz"])))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(field_error(vec![test_error(
                "Expected Array with length between 1 and 3."
            )]))
        );
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz", "qux"]))),
            Err(field_error(vec![test_error(
                "Expected Array with length between 1 and 3."
            )]))
        );
    }

    #[test]
    fn it_sets_a_minimum_length() {
        let schema = array().min_length(4);
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz", "qux"]))),
            Ok(Some(json!(["foo", "bar", "baz", "qux"])))
        );
        assert_eq!(
            schema.validate(Some(json!(["foo"]))),
            Err(field_error(vec![test_error(
                "Expected Array with length of at least 4."
            )]))
        );
    }

    #[test]
    fn it_sets_a_maximum_length() {
        let schema = array().max_length(3);
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz"]))),
            Ok(Some(json!(["foo", "bar", "baz"])))
        );
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz", "qux"]))),
            Err(field_error(vec![test_error(
                "Expected Array with length of at most 3."
            )]))
        );
    }

    #[test]
    fn it_validates_arrays_of_booleans() {
        let schema = array().of(boolean());
        assert_eq!(
            schema.validate(Some(json!([true, false, true]))),
            Ok(Some(json!([true, false, true])))
        );
        assert_eq!(
            schema.validate(Some(json!([1, 2, 3]))),
            Err(array_error(hashmap! {
                0 => type_error("[0]", JsonType::Boolean),
                1 => type_error("[1]", JsonType::Boolean),
                2 => type_error("[2]", JsonType::Boolean)
            }))
        );
    }

    #[test]
    fn it_validates_arrays_of_numbers() {
        let schema = array().of(integer());
        assert_eq!(
            schema.validate(Some(json!([1, 2, 3]))),
            Ok(Some(json!([1, 2, 3])))
        );
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz"]))),
            Err(array_error(hashmap! {
                0 => type_error("[0]", JsonType::Integer),
                1 => type_error("[1]", JsonType::Integer),
                2 => type_error("[2]", JsonType::Integer)
            }))
        );
    }

    #[test]
    fn it_validates_arrays_of_strings() {
        let schema = array().of(string());
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz"]))),
            Ok(Some(json!(["foo", "bar", "baz"])))
        );
        assert_eq!(
            schema.validate(Some(json!([null, null, null]))),
            Err(array_error(hashmap! {
                0 => type_error("[0]", JsonType::String),
                1 => type_error("[1]", JsonType::String),
                2 => type_error("[2]", JsonType::String)
            }))
        );
    }

    #[test]
    fn it_validates_arrays_of_objects() {
        let schema = array().of(object());
        assert_eq!(
            schema.validate(Some(json!([{}, {}, {}]))),
            Ok(Some(json!([{}, {}, {}])))
        );
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz"]))),
            Err(array_error(hashmap! {
                0 => type_error("[0]", JsonType::Object),
                1 => type_error("[1]", JsonType::Object),
                2 => type_error("[2]", JsonType::Object)
            }))
        );
    }

    #[test]
    fn it_validates_arrays_of_arrays() {
        let schema = array().of(array());
        assert_eq!(
            schema.validate(Some(json!([[], [], []]))),
            Ok(Some(json!([[], [], []])))
        );
        assert_eq!(
            schema.validate(Some(json!([1, 2, 3]))),
            Err(array_error(hashmap! {
                0 => type_error("[0]", JsonType::Array),
                1 => type_error("[1]", JsonType::Array),
                2 => type_error("[2]", JsonType::Array)
            }))
        );
    }
}
