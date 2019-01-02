use super::{
    error::{ValidationError, ValidationResult},
    json::{Json, JsonType},
    OkSchema, Validator,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct NumberSchema<N>
where
    N: Serialize + DeserializeOwned + PartialOrd + std::fmt::Display,
{
    validator: Validator<N>,
}

impl<N> NumberSchema<N>
where
    N: Serialize + DeserializeOwned + PartialOrd + std::fmt::Display,
{
    pub fn new(jsontype_: JsonType) -> Self {
        NumberSchema {
            validator: Validator::new(jsontype_),
        }
    }

    pub fn min(mut self, min: N) -> Self
    where
        N: 'static,
    {
        self.validator.add_test(
            "min",
            format!("<label> must be at least {}.", min),
            move |number| Ok(number >= &min),
        );
        self
    }

    pub fn max(mut self, max: N) -> Self
    where
        N: 'static,
    {
        self.validator.add_test(
            "max",
            format!("<label> must be at most {}.", max),
            move |number| Ok(number <= &max),
        );
        self
    }

    pub fn greater_than(mut self, limit: N) -> Self
    where
        N: 'static,
    {
        self.validator.add_test(
            "greater_than",
            format!("<label> must be greater than {}.", limit),
            move |number| Ok(number > &limit),
        );
        self
    }

    pub fn less_than(mut self, limit: N) -> Self
    where
        N: 'static,
    {
        self.validator.add_test(
            "less_than",
            format!("<label> must be less than {}.", limit),
            move |number| Ok(number < &limit),
        );
        self
    }
}

impl<N> OkSchema for NumberSchema<N>
where
    N: Serialize + DeserializeOwned + PartialOrd + std::fmt::Display,
{
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
        error::{payload_error, test_error, type_error},
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
            Err(payload_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(Some(json!(std::i64::MAX as u64 + 1))),
            Err(payload_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(payload_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(None),
            Err(payload_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(payload_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(payload_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(payload_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(payload_error(vec![type_error("", "", JsonType::Integer)]))
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
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(Some(json!(-1.0))),
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(Some(json!(-1))),
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(None),
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(payload_error(vec![type_error("", "", JsonType::Unsigned)]))
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
            Err(payload_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(payload_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(None),
            Err(payload_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(payload_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(payload_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(Some(json!({}))),
            Err(payload_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(payload_error(vec![type_error("", "", JsonType::Float)]))
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
            Err(payload_error(vec![test_error(
                "min",
                "",
                "u64 must be at least 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(4))),
            Err(payload_error(vec![test_error(
                "min",
                "",
                "i64 must be at least 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(4.0))),
            Err(payload_error(vec![test_error(
                "min",
                "",
                "f64 must be at least 5."
            )]))
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
            Err(payload_error(vec![test_error(
                "max",
                "",
                "u64 must be at most 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(6))),
            Err(payload_error(vec![test_error(
                "max",
                "",
                "i64 must be at most 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(6.0))),
            Err(payload_error(vec![test_error(
                "max",
                "",
                "f64 must be at most 5."
            )]))
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
            Err(payload_error(vec![test_error(
                "greater_than",
                "",
                "u64 must be greater than 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(5))),
            Err(payload_error(vec![test_error(
                "greater_than",
                "",
                "i64 must be greater than 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(5.0))),
            Err(payload_error(vec![test_error(
                "greater_than",
                "",
                "f64 must be greater than 5."
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
            Err(payload_error(vec![test_error(
                "less_than",
                "",
                "u64 must be less than 5."
            )]))
        );
        assert_eq!(
            i.validate(Some(json!(5))),
            Err(payload_error(vec![test_error(
                "less_than",
                "",
                "i64 must be less than 5."
            )]))
        );
        assert_eq!(
            f.validate(Some(json!(5.0))),
            Err(payload_error(vec![test_error(
                "less_than",
                "",
                "f64 must be less than 5."
            )]))
        );
    }
}
