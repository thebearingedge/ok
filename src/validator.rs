use super::{
    error::{field_error, type_error, Result, ValidationError},
    json::{from_json, to_json, Json, JsonType},
    Test,
};
use serde::{de::DeserializeOwned, ser::Serialize};

pub struct Validator<T: DeserializeOwned + Serialize> {
    pub json_type: JsonType,
    pub label: Option<&'static str>,
    pub description: Option<&'static str>,
    pub is_optional: bool,
    pub is_nullable: bool,
    pub tests: Vec<Test<T>>,
    pub transforms: Vec<fn(T) -> T>,
}

impl<T: DeserializeOwned + Serialize> Validator<T> {
    pub fn new(json_type: JsonType) -> Self {
        Validator {
            json_type,
            label: None,
            description: None,
            is_optional: false,
            is_nullable: false,
            tests: vec![],
            transforms: vec![],
        }
    }

    pub fn add_test<M, V>(&mut self, message: M, test: V)
    where
        M: Into<String>,
        V: Fn(&T) -> Result<bool> + 'static,
    {
        self.tests.push(Test::new(message, test));
    }

    pub fn add_transform(&mut self, transform: fn(T) -> T) {
        self.transforms.push(transform);
    }

    pub fn exec(&self, path: &str, value: Option<Json>) -> Result<Option<Json>> {
        let label = self.label.unwrap_or(path);
        let json_type = self.json_type;
        let coerce = match value {
            None if self.is_optional => return Ok(None),
            None => Err(type_error(label, json_type)),
            Some(Json::Null) if self.is_nullable => return Ok(Some(Json::Null)),
            Some(json) => json_type.coerce(label, json),
        };
        coerce.and_then(|json| {
            let t = self
                .transforms
                .iter()
                .fold(from_json(json).unwrap(), |t, transform| transform(t));
            let errors = self
                .tests
                .iter()
                .filter_map(|test| test.check(label, &t).err())
                .collect::<Vec<ValidationError>>();
            if errors.is_empty() {
                return Ok(Some(to_json(t).unwrap()));
            }
            Err(field_error(errors))
        })
    }
}
