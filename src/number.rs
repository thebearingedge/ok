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

    pub fn not_one_of(mut self, values: Vec<N>) -> Self
    where
        N: 'static,
    {
        let message = format!(
            "<label> must not be one of the following: {}",
            values
                .iter()
                .map(|value| format!("{}", value))
                .collect::<Vec<String>>()
                .join(", ")
        );
        self.validator
            .add_test("not_one_of", message, move |number| {
                Ok(!values.iter().any(|value| value == number))
            });
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
        error::{json_error, test_error, type_error},
        float, integer,
        json::JsonType,
        unsigned, OkSchema,
    };
    use serde_json::json;

    #[test]
    fn it_validates_integers() {
        let schema = integer();
        assert_eq!(schema.validate(json!(1)), Ok(json!(1)));
        assert_eq!(schema.validate(json!(1.0)), Ok(json!(1)));
        assert_eq!(schema.validate(json!("1")), Ok(json!(1)));
        assert_eq!(schema.validate(json!(-1)), Ok(json!(-1)));
        assert_eq!(schema.validate(json!(-1.0)), Ok(json!(-1)));
        assert_eq!(schema.validate(json!("-1")), Ok(json!(-1)));
        assert_eq!(
            schema.validate(json!(1.1)),
            Err(json_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(json!(std::i64::MAX as u64 + 1)),
            Err(json_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(json!(null)),
            Err(json_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(json!([])),
            Err(json_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(json!(true)),
            Err(json_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(json!({})),
            Err(json_error(vec![type_error("", "", JsonType::Integer)]))
        );
        assert_eq!(
            schema.validate(json!("foo")),
            Err(json_error(vec![type_error("", "", JsonType::Integer)]))
        );
    }

    #[test]
    fn it_validates_unsigned_integers() {
        let schema = unsigned();
        assert_eq!(schema.validate(json!(1)), Ok(json!(1)));
        assert_eq!(schema.validate(json!(1.0)), Ok(json!(1)));
        assert_eq!(schema.validate(json!("1")), Ok(json!(1)));
        assert_eq!(
            schema.validate(json!(1.1)),
            Err(json_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(json!(-1.0)),
            Err(json_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(json!(-1)),
            Err(json_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(json!(null)),
            Err(json_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(json!([])),
            Err(json_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(json!(true)),
            Err(json_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(json!({})),
            Err(json_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
        assert_eq!(
            schema.validate(json!("foo")),
            Err(json_error(vec![type_error("", "", JsonType::Unsigned)]))
        );
    }

    #[test]
    fn it_validates_floats() {
        let schema = float();
        assert_eq!(schema.validate(json!(1)), Ok(json!(1.0)));
        assert_eq!(schema.validate(json!(1.0)), Ok(json!(1.0)));
        assert_eq!(schema.validate(json!("1")), Ok(json!(1.0)));
        assert_eq!(schema.validate(json!("1.0")), Ok(json!(1.0)));
        assert_eq!(schema.validate(json!(-1)), Ok(json!(-1.0)));
        assert_eq!(schema.validate(json!(-1.0)), Ok(json!(-1.0)));
        assert_eq!(schema.validate(json!("-1")), Ok(json!(-1.0)));
        assert_eq!(schema.validate(json!("-1.0")), Ok(json!(-1.0)));
        assert_eq!(
            schema.validate(json!(std::i64::MAX as u64 + 1)),
            Err(json_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(json!(null)),
            Err(json_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(json!([])),
            Err(json_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(json!(true)),
            Err(json_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(json!({})),
            Err(json_error(vec![type_error("", "", JsonType::Float)]))
        );
        assert_eq!(
            schema.validate(json!("foo")),
            Err(json_error(vec![type_error("", "", JsonType::Float)]))
        );
    }

    #[test]
    fn it_sets_a_minimum_value() {
        let u = unsigned().label("u64").min(5);
        let i = integer().label("i64").min(5);
        let f = float().label("f64").min(5.0);
        assert_eq!(u.validate(json!(6)), Ok(json!(6)));
        assert_eq!(i.validate(json!(6)), Ok(json!(6)));
        assert_eq!(f.validate(json!(6.0)), Ok(json!(6.0)));
        assert_eq!(
            u.validate(json!(4)),
            Err(json_error(vec![test_error(
                "min",
                "",
                "u64 must be at least 5."
            )]))
        );
        assert_eq!(
            i.validate(json!(4)),
            Err(json_error(vec![test_error(
                "min",
                "",
                "i64 must be at least 5."
            )]))
        );
        assert_eq!(
            f.validate(json!(4.0)),
            Err(json_error(vec![test_error(
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
        assert_eq!(u.validate(json!(4)), Ok(json!(4)));
        assert_eq!(i.validate(json!(4)), Ok(json!(4)));
        assert_eq!(f.validate(json!(4.0)), Ok(json!(4.0)));
        assert_eq!(
            u.validate(json!(6)),
            Err(json_error(vec![test_error(
                "max",
                "",
                "u64 must be at most 5."
            )]))
        );
        assert_eq!(
            i.validate(json!(6)),
            Err(json_error(vec![test_error(
                "max",
                "",
                "i64 must be at most 5."
            )]))
        );
        assert_eq!(
            f.validate(json!(6.0)),
            Err(json_error(vec![test_error(
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
        assert_eq!(u.validate(json!(6)), Ok(json!(6)));
        assert_eq!(i.validate(json!(6)), Ok(json!(6)));
        assert_eq!(f.validate(json!(6.0)), Ok(json!(6.0)));
        assert_eq!(
            u.validate(json!(5)),
            Err(json_error(vec![test_error(
                "greater_than",
                "",
                "u64 must be greater than 5."
            )]))
        );
        assert_eq!(
            i.validate(json!(5)),
            Err(json_error(vec![test_error(
                "greater_than",
                "",
                "i64 must be greater than 5."
            )]))
        );
        assert_eq!(
            f.validate(json!(5.0)),
            Err(json_error(vec![test_error(
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
        assert_eq!(u.validate(json!(4)), Ok(json!(4)));
        assert_eq!(i.validate(json!(4)), Ok(json!(4)));
        assert_eq!(f.validate(json!(4.0)), Ok(json!(4.0)));
        assert_eq!(
            u.validate(json!(5)),
            Err(json_error(vec![test_error(
                "less_than",
                "",
                "u64 must be less than 5."
            )]))
        );
        assert_eq!(
            i.validate(json!(5)),
            Err(json_error(vec![test_error(
                "less_than",
                "",
                "i64 must be less than 5."
            )]))
        );
        assert_eq!(
            f.validate(json!(5.0)),
            Err(json_error(vec![test_error(
                "less_than",
                "",
                "f64 must be less than 5."
            )]))
        );
    }
}
