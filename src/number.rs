use super::{
    error::Result,
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
}

impl<N: Number> NumberSchema<N> {
    pub fn new(json_type: JsonType) -> Self {
        NumberSchema {
            validator: Validator::new(json_type),
        }
    }

    pub fn min(mut self, min: N) -> Self
    where
        N: 'static,
    {
        self.validator.add_test(
            format!("<label> should be at least {}.", min),
            move |number| Ok(number >= &min),
        );
        self
    }

    pub fn max(mut self, max: N) -> Self
    where
        N: 'static,
    {
        self.validator.add_test(
            format!("<label> should be at most {}.", max),
            move |number| Ok(number <= &max),
        );
        self
    }

    pub fn greater_than(mut self, limit: N) -> Self
    where
        N: 'static,
    {
        self.validator.add_test(
            format!("<label> should be greater than {}.", limit),
            move |number| Ok(number > &limit),
        );
        self
    }

    pub fn less_than(mut self, limit: N) -> Self
    where
        N: 'static,
    {
        self.validator.add_test(
            format!("<label> should be less than {}.", limit),
            move |number| Ok(number < &limit),
        );
        self
    }
}

impl<N: Number> OkSchema for NumberSchema<N> {
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
        self.validator.exec(path, value)
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
    use super::super::{
        error::{field_error, test_error, type_error},
        float, integer,
        json::JsonType,
        unsigned, OkSchema,
    };
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
            Err(type_error("", JsonType::Integer))
        );
        assert_eq!(
            schema.validate(Some(json!(std::i64::MAX as u64 + 1))),
            Err(type_error("", JsonType::Integer))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(type_error("", JsonType::Integer))
        );
        assert_eq!(
            schema.validate(None),
            Err(type_error("", JsonType::Integer))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(type_error("", JsonType::Integer))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(type_error("", JsonType::Integer))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(type_error("", JsonType::Integer))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(type_error("", JsonType::Integer))
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
            Err(type_error("", JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!(-1.0))),
            Err(type_error("", JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!(-1))),
            Err(type_error("", JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(type_error("", JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(None),
            Err(type_error("", JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(type_error("", JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(type_error("", JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(type_error("", JsonType::Unsigned))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(type_error("", JsonType::Unsigned))
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
            Err(type_error("", JsonType::Float))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(type_error("", JsonType::Float))
        );
        assert_eq!(schema.validate(None), Err(type_error("", JsonType::Float)));
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(type_error("", JsonType::Float))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(type_error("", JsonType::Float))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(type_error("", JsonType::Float))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(type_error("", JsonType::Float))
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
        let u = unsigned().label("u64").min(5);
        let i = integer().label("i64").min(5);
        let f = float().label("f64").min(5.0);
        assert_eq!(u.validate(Some(json!(6))), Ok(Some(json!(6))));
        assert_eq!(i.validate(Some(json!(6))), Ok(Some(json!(6))));
        assert_eq!(f.validate(Some(json!(6.0))), Ok(Some(json!(6.0))));
        assert_eq!(
            u.validate(Some(json!(4))),
            Err(field_error(vec![test_error("u64 should be at least 5.")]))
        );
        assert_eq!(
            i.validate(Some(json!(4))),
            Err(field_error(vec![test_error("i64 should be at least 5.")]))
        );
        assert_eq!(
            f.validate(Some(json!(4.0))),
            Err(field_error(vec![test_error("f64 should be at least 5.")]))
        );
    }

    #[test]
    fn it_sets_a_maximum_value() {
        let u = unsigned().label("u64").max(5);
        let i = integer().label("i64").max(5);
        let f = float().label("f64").max(5.0);
        assert_eq!(u.validate(Some(json!(4))), Ok(Some(json!(4))));
        assert_eq!(i.validate(Some(json!(4))), Ok(Some(json!(4))));
        assert_eq!(f.validate(Some(json!(4.0))), Ok(Some(json!(4.0))));
        assert_eq!(
            u.validate(Some(json!(6))),
            Err(field_error(vec![test_error("u64 should be at most 5.")]))
        );
        assert_eq!(
            i.validate(Some(json!(6))),
            Err(field_error(vec![test_error("i64 should be at most 5.")]))
        );
        assert_eq!(
            f.validate(Some(json!(6.0))),
            Err(field_error(vec![test_error("f64 should be at most 5.")]))
        );
    }

    #[test]
    fn it_sets_a_lower_limit() {
        let u = unsigned().label("u64").greater_than(5);
        let i = integer().label("i64").greater_than(5);
        let f = float().label("f64").greater_than(5.0);
        assert_eq!(u.validate(Some(json!(6))), Ok(Some(json!(6))));
        assert_eq!(i.validate(Some(json!(6))), Ok(Some(json!(6))));
        assert_eq!(f.validate(Some(json!(6.0))), Ok(Some(json!(6.0))));
        assert_eq!(
            u.validate(Some(json!(5))),
            Err(field_error(vec![test_error(
                "u64 should be greater than 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(5))),
            Err(field_error(vec![test_error(
                "i64 should be greater than 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(5.0))),
            Err(field_error(vec![test_error(
                "f64 should be greater than 5."
            )]))
        );
    }

    #[test]
    fn it_sets_an_upper_limit() {
        let u = unsigned().label("u64").less_than(5);
        let i = integer().label("i64").less_than(5);
        let f = float().label("f64").less_than(5.0);
        assert_eq!(u.validate(Some(json!(4))), Ok(Some(json!(4))));
        assert_eq!(i.validate(Some(json!(4))), Ok(Some(json!(4))));
        assert_eq!(f.validate(Some(json!(4.0))), Ok(Some(json!(4.0))));
        assert_eq!(
            u.validate(Some(json!(5))),
            Err(field_error(vec![test_error("u64 should be less than 5.")]))
        );
        assert_eq!(
            i.validate(Some(json!(5))),
            Err(field_error(vec![test_error("i64 should be less than 5.")]))
        );
        assert_eq!(
            f.validate(Some(json!(5.0))),
            Err(field_error(vec![test_error("f64 should be less than 5.")]))
        );
    }
}
