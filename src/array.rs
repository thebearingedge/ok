use super::{
    error::{ValidationError, ValidationResult},
    json::{from_json, Array, Json, JsonType},
    OkSchema, Validator,
};

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
            "length",
            format!("<label> must have length between {} and {}.", min, max),
            move |array| Ok(array.len() >= min && array.len() <= max),
        );
        self
    }

    pub fn min_length(mut self, min: usize) -> Self {
        self.validator.add_test(
            "min_length",
            format!("<label> must contain at least {} elements.", min),
            move |array| Ok(array.len() >= min),
        );
        self
    }

    pub fn max_length(mut self, max: usize) -> Self {
        self.validator.add_test(
            "max_length",
            format!("<label> may contain at most {} elements.", max),
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

    fn validate_at(
        &self,
        path: &str,
        value: Option<Json>,
        all_errors: &mut Vec<ValidationError>,
    ) -> ValidationResult<Option<Json>> {
        let mut errors = vec![];
        let validated = self.validator.exec(path, value, &mut errors);
        let elements = match validated {
            Ok(None) => return Ok(None),
            Ok(Some(json)) => {
                if self.element_schema.is_none() {
                    return Ok(Some(json));
                }
                from_json::<Array>(json).unwrap()
            }
            Err(_) => return Err(all_errors.append(&mut errors)),
        };
        let mut array = vec![];
        let element_schema = self.element_schema.as_ref().unwrap();
        elements
            .into_iter()
            .enumerate()
            .for_each(|(index, element)| {
                let path = format!("{}[{}]", path, index);
                if let Ok(validated) =
                    element_schema.validate_at(path.as_str(), Some(element), &mut errors)
                {
                    if errors.is_empty() {
                        array.push(validated.unwrap());
                    }
                }
            });
        if errors.is_empty() {
            return Ok(Some(array.into()));
        }
        Err(all_errors.append(&mut errors))
    }
}

pub fn array() -> ArraySchema {
    ArraySchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{
        array, boolean,
        error::{json_error, test_error, type_error},
        integer,
        json::JsonType,
        object, string, OkSchema,
    };
    use serde_json::json;

    #[test]
    fn it_validates_arrays() {
        let schema = array();
        assert_eq!(schema.validate(json!([])), Ok(json!([])));
        assert_eq!(
            schema.validate(json!(null)),
            Err(json_error(vec![type_error("", "", JsonType::Array)]))
        );
        assert_eq!(
            schema.validate(json!({})),
            Err(json_error(vec![type_error("", "", JsonType::Array)]))
        );
        assert_eq!(
            schema.validate(json!(1)),
            Err(json_error(vec![type_error("", "", JsonType::Array)]))
        );
        assert_eq!(
            schema.validate(json!(true)),
            Err(json_error(vec![type_error("", "", JsonType::Array)]))
        );
        assert_eq!(
            schema.validate(json!("foo")),
            Err(json_error(vec![type_error("", "", JsonType::Array)]))
        );
    }

    #[test]
    fn it_sets_a_minimum_and_maximum_length() {
        let schema = array().label("My Array").length((1, 3));
        assert_eq!(
            schema.validate(json!(["foo", "bar", "baz"])),
            Ok(json!(["foo", "bar", "baz"]))
        );
        assert_eq!(
            schema.validate(json!([])),
            Err(json_error(vec![test_error(
                "length",
                "",
                "My Array must have length between 1 and 3."
            )]))
        );
        assert_eq!(
            schema.validate(json!(["foo", "bar", "baz", "qux"])),
            Err(json_error(vec![test_error(
                "length",
                "",
                "My Array must have length between 1 and 3."
            )]))
        );
    }

    #[test]
    fn it_sets_a_minimum_length() {
        let schema = array().label("My Array").min_length(4);
        assert_eq!(
            schema.validate(json!(["foo", "bar", "baz", "qux"])),
            Ok(json!(["foo", "bar", "baz", "qux"]))
        );
        assert_eq!(
            schema.validate(json!(["foo"])),
            Err(json_error(vec![test_error(
                "min_length",
                "",
                "My Array must contain at least 4 elements."
            )]))
        );
    }

    #[test]
    fn it_sets_a_maximum_length() {
        let schema = array().label("My Array").max_length(3);
        assert_eq!(
            schema.validate(json!(["foo", "bar", "baz"])),
            Ok(json!(["foo", "bar", "baz"]))
        );
        assert_eq!(
            schema.validate(json!(["foo", "bar", "baz", "qux"])),
            Err(json_error(vec![test_error(
                "max_length",
                "",
                "My Array may contain at most 3 elements."
            )]))
        );
    }

    #[test]
    fn it_validates_arrays_of_booleans() {
        let schema = array().of(boolean());
        assert_eq!(
            schema.validate(json!([true, false, true])),
            Ok(json!([true, false, true]))
        );
        assert_eq!(
            schema.validate(json!([1, 2, 3])),
            Err(json_error(vec![
                type_error("[0]", "[0]", JsonType::Boolean),
                type_error("[1]", "[1]", JsonType::Boolean),
                type_error("[2]", "[2]", JsonType::Boolean)
            ]))
        );
    }

    #[test]
    fn it_validates_arrays_of_numbers() {
        let schema = array().of(integer());
        assert_eq!(schema.validate(json!([1, 2, 3])), Ok(json!([1, 2, 3])));
        assert_eq!(
            schema.validate(json!(["foo", "bar", "baz"])),
            Err(json_error(vec![
                type_error("[0]", "[0]", JsonType::Integer),
                type_error("[1]", "[1]", JsonType::Integer),
                type_error("[2]", "[2]", JsonType::Integer)
            ]))
        );
    }

    #[test]
    fn it_validates_arrays_of_strings() {
        let schema = array().of(string());
        assert_eq!(
            schema.validate(json!(["foo", "bar", "baz"])),
            Ok(json!(["foo", "bar", "baz"]))
        );
        assert_eq!(
            schema.validate(json!([null, null, null])),
            Err(json_error(vec![
                type_error("[0]", "[0]", JsonType::String),
                type_error("[1]", "[1]", JsonType::String),
                type_error("[2]", "[2]", JsonType::String)
            ]))
        );
    }

    #[test]
    fn it_validates_arrays_of_objects() {
        let schema = array().of(object());
        assert_eq!(
            schema.validate(json!([{}, {}, {}])),
            Ok(json!([{}, {}, {}]))
        );
        assert_eq!(
            schema.validate(json!(["foo", "bar", "baz"])),
            Err(json_error(vec![
                type_error("[0]", "[0]", JsonType::Object),
                type_error("[1]", "[1]", JsonType::Object),
                type_error("[2]", "[2]", JsonType::Object)
            ]))
        );
    }

    #[test]
    fn it_validates_arrays_of_arrays() {
        let schema = array().of(array());
        assert_eq!(
            schema.validate(json!([[], [], []])),
            Ok(json!([[], [], []]))
        );
        assert_eq!(
            schema.validate(json!([1, 2, 3])),
            Err(json_error(vec![
                type_error("[0]", "[0]", JsonType::Array),
                type_error("[1]", "[1]", JsonType::Array),
                type_error("[2]", "[2]", JsonType::Array)
            ]))
        );
    }
}
