use super::{
    error::{self, Result},
    json::{Json, JsonType},
    OkSchema, Validator,
};
use serde::{de::DeserializeOwned, Serialize};

pub trait Number: Serialize + DeserializeOwned + PartialOrd + std::fmt::Display {}

impl Number for i64 {}
impl Number for u64 {}
impl Number for f64 {}

pub struct NumberSchema<N: Number> {
    validator: Validator<N>,
    description: Option<&'static str>,
}

impl<N: Number> NumberSchema<N> {
    pub fn new(json_type: JsonType) -> Self {
        NumberSchema {
            description: None,
            validator: Validator::new(json_type),
        }
    }

    pub fn min(mut self, minimum: N) -> Self
    where
        N: 'static,
    {
        let json_type = self.validator.json_type;
        self.validator.test(move |number| {
            if number >= &minimum {
                return Ok(());
            }
            Err(error::value_error(format!(
                "Expected {} value of at least {}.",
                json_type, minimum
            )))
        });
        self
    }

    pub fn max(mut self, maximum: N) -> Self
    where
        N: 'static,
    {
        let json_type = self.validator.json_type;
        self.validator.test(move |number| {
            if number <= &maximum {
                return Ok(());
            }
            Err(error::value_error(format!(
                "Expected {} value of at most {}.",
                json_type, maximum
            )))
        });
        self
    }

    pub fn greater_than(mut self, limit: N) -> Self
    where
        N: 'static,
    {
        let json_type = self.validator.json_type;
        self.validator.test(move |number| {
            if number > &limit {
                return Ok(());
            }
            Err(error::value_error(format!(
                "Expected {} value greater than {}.",
                json_type, limit
            )))
        });
        self
    }

    pub fn less_than(mut self, limit: N) -> Self
    where
        N: 'static,
    {
        let json_type = self.validator.json_type;
        self.validator.test(move |number| {
            if number < &limit {
                return Ok(());
            }
            Err(error::value_error(format!(
                "Expected {} value less than {}.",
                json_type, limit
            )))
        });
        self
    }
}

impl<N: Number> OkSchema for NumberSchema<N> {
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

    #[test]
    fn it_sets_a_minimum_value() {
        let u = unsigned().min(5);
        let i = integer().min(5);
        let f = float().min(5.0);
        assert_eq!(u.validate(Some(json!(6))), Ok(Some(json!(6))));
        assert_eq!(i.validate(Some(json!(6))), Ok(Some(json!(6))));
        assert_eq!(f.validate(Some(json!(6.0))), Ok(Some(json!(6.0))));
        assert_eq!(
            u.validate(Some(json!(4))),
            Err(error::field_error(vec![error::value_error(
                "Expected Unsigned Integer value of at least 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(4))),
            Err(error::field_error(vec![error::value_error(
                "Expected Integer value of at least 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(4.0))),
            Err(error::field_error(vec![error::value_error(
                "Expected Float value of at least 5."
            )]))
        );
    }

    #[test]
    fn it_sets_a_maximum_value() {
        let u = unsigned().max(5);
        let i = integer().max(5);
        let f = float().max(5.0);
        assert_eq!(u.validate(Some(json!(4))), Ok(Some(json!(4))));
        assert_eq!(i.validate(Some(json!(4))), Ok(Some(json!(4))));
        assert_eq!(f.validate(Some(json!(4.0))), Ok(Some(json!(4.0))));
        assert_eq!(
            u.validate(Some(json!(6))),
            Err(error::field_error(vec![error::value_error(
                "Expected Unsigned Integer value of at most 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(6))),
            Err(error::field_error(vec![error::value_error(
                "Expected Integer value of at most 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(6.0))),
            Err(error::field_error(vec![error::value_error(
                "Expected Float value of at most 5."
            )]))
        );
    }

    #[test]
    fn it_sets_a_lower_limit() {
        let u = unsigned().greater_than(5);
        let i = integer().greater_than(5);
        let f = float().greater_than(5.0);
        assert_eq!(u.validate(Some(json!(6))), Ok(Some(json!(6))));
        assert_eq!(i.validate(Some(json!(6))), Ok(Some(json!(6))));
        assert_eq!(f.validate(Some(json!(6.0))), Ok(Some(json!(6.0))));
        assert_eq!(
            u.validate(Some(json!(5))),
            Err(error::field_error(vec![error::value_error(
                "Expected Unsigned Integer value greater than 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(5))),
            Err(error::field_error(vec![error::value_error(
                "Expected Integer value greater than 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(5.0))),
            Err(error::field_error(vec![error::value_error(
                "Expected Float value greater than 5."
            )]))
        );
    }

    #[test]
    fn it_sets_an_upper_limit() {
        let u = unsigned().less_than(5);
        let i = integer().less_than(5);
        let f = float().less_than(5.0);
        assert_eq!(u.validate(Some(json!(4))), Ok(Some(json!(4))));
        assert_eq!(i.validate(Some(json!(4))), Ok(Some(json!(4))));
        assert_eq!(f.validate(Some(json!(4.0))), Ok(Some(json!(4.0))));
        assert_eq!(
            u.validate(Some(json!(5))),
            Err(error::field_error(vec![error::value_error(
                "Expected Unsigned Integer value less than 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(5))),
            Err(error::field_error(vec![error::value_error(
                "Expected Integer value less than 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(5.0))),
            Err(error::field_error(vec![error::value_error(
                "Expected Float value less than 5."
            )]))
        );
    }
}
