use super::{
    error::{type_error, Result, ValidationError, ValidationResult},
    json::{from_json, to_json, Json, JsonType},
    Test,
};
use serde::{de::DeserializeOwned, ser::Serialize};

pub struct Validator<T: DeserializeOwned + Serialize> {
    pub jsontype_: JsonType,
    pub label: Option<&'static str>,
    pub description: Option<&'static str>,
    pub is_optional: bool,
    pub is_nullable: bool,
    pub tests: Vec<Test<T>>,
    pub transforms: Vec<fn(T) -> T>,
}

impl<T: DeserializeOwned + Serialize> Validator<T> {
    pub fn new(jsontype_: JsonType) -> Self {
        Validator {
            jsontype_,
            label: None,
            description: None,
            is_optional: false,
            is_nullable: false,
            tests: vec![],
            transforms: vec![],
        }
    }

    pub fn add_test<M, F>(&mut self, type_: &'static str, message: M, test: F)
    where
        M: Into<String>,
        F: Fn(&T) -> Result<bool> + 'static,
    {
        self.tests.push(Test::new(type_, message, test));
    }

    pub fn add_transform(&mut self, transform: fn(T) -> T) {
        self.transforms.push(transform);
    }

    pub fn exec(
        &self,
        path: &str,
        value: Option<Json>,
        all_errors: &mut Vec<ValidationError>,
    ) -> ValidationResult<Option<Json>> {
        let label = self.label.unwrap_or(path);
        let coersion = match value {
            None if self.is_optional => return Ok(None),
            None => Err(type_error(path, label, self.jsontype_)),
            Some(Json::Null) if self.is_nullable => return Ok(value),
            Some(json) => self.jsontype_.coerce(path, label, json),
        };
        if coersion.is_err() {
            return Err(all_errors.push(coersion.unwrap_err()));
        }
        let json = coersion.unwrap();
        let t = self
            .transforms
            .iter()
            .fold(from_json(json).unwrap(), |t, transform| transform(t));
        let mut errors = self
            .tests
            .iter()
            .filter_map(|test| test.check(path, label, &t).err())
            .collect::<Vec<ValidationError>>();
        if errors.is_empty() {
            return Ok(Some(to_json(t).unwrap()));
        }
        Err(all_errors.append(&mut errors))
    }
}
