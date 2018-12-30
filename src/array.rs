use super::{
    error::{self, Result},
    json::{Array, Json, JsonType},
    OkSchema, Validator,
};
use std::collections::HashMap;

pub struct ArraySchema {
    validator: Validator<Array>,
    description: Option<&'static str>,
    element_schema: Option<Box<OkSchema>>,
}

impl ArraySchema {
    pub fn new() -> Self {
        ArraySchema {
            description: None,
            validator: Validator::new(JsonType::Array),
            element_schema: None,
        }
    }

    pub fn length(mut self, (minimum, maximum): (usize, usize)) -> Self {
        self.validator.append(move |array| {
            if array.len() >= minimum && array.len() <= maximum {
                return Ok(None);
            }
            Err(error::value_error(format!(
                "Expected Array with length between {} and {}.",
                minimum, maximum
            )))
        });
        self
    }

    pub fn min_length(mut self, minimum: usize) -> Self {
        self.validator.append(move |array| {
            if array.len() >= minimum {
                return Ok(None);
            }
            Err(error::value_error(format!(
                "Expected Array with length of at least {}.",
                minimum
            )))
        });
        self
    }

    pub fn max_length(mut self, maximum: usize) -> Self {
        self.validator.append(move |array| {
            if array.len() <= maximum {
                return Ok(None);
            }
            Err(error::value_error(format!(
                "Expected Array with length of at most {}.",
                maximum
            )))
        });
        self
    }

    pub fn of(mut self, schema: impl OkSchema + 'static) -> Self {
        self.element_schema = Some(Box::new(schema));
        self
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
        let validated = self.validator.exec(value)?;
        if self.element_schema.is_none() {
            return Ok(validated);
        }
        let elements = match validated {
            None => return Ok(None),
            Some(json) => match json {
                Json::Array(elements) => elements,
                _ => return Ok(Some(json)),
            },
        };
        let mut array = vec![];
        let mut errors = HashMap::new();
        let element_schema = self.element_schema.as_ref().unwrap();
        elements
            .into_iter()
            .enumerate()
            .for_each(|(index, element)| {
                match element_schema.validate(Some(element)) {
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
        Err(error::array_error(errors))
    }
}

pub fn array() -> ArraySchema {
    ArraySchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{array, boolean, error, json::JsonType, OkSchema};
    use maplit::hashmap;
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

    #[test]
    fn it_sets_a_minimum_and_maximum_length() {
        let schema = array().length((1, 3));

        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz"]))),
            Ok(Some(json!(["foo", "bar", "baz"])))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(error::field_error(vec![error::value_error(
                "Expected Array with length between 1 and 3."
            )]))
        );
        assert_eq!(
            schema.validate(Some(json!(["foo", "bar", "baz", "qux"]))),
            Err(error::field_error(vec![error::value_error(
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
            Err(error::field_error(vec![error::value_error(
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
            Err(error::field_error(vec![error::value_error(
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
            Err(error::array_error(hashmap! {
                0 => error::type_error(JsonType::Boolean, JsonType::Number),
                1 => error::type_error(JsonType::Boolean, JsonType::Number),
                2 => error::type_error(JsonType::Boolean, JsonType::Number)
            }))
        );
    }
}
