use super::{
    error::{self, Result, ValidationError},
    json::{from_json, to_json, Json, JsonType},
};
use serde::{de::DeserializeOwned, ser::Serialize};

pub struct Validator<T: DeserializeOwned + Serialize> {
    pub json_type: JsonType,
    pub is_optional: bool,
    pub is_nullable: bool,
    pub tests: Vec<Box<Fn(&T) -> Result<()>>>,
    pub transforms: Vec<fn(T) -> T>,
}

impl<T: DeserializeOwned + Serialize> Validator<T> {
    pub fn new(json_type: JsonType) -> Self {
        Validator {
            json_type,
            is_optional: false,
            is_nullable: false,
            tests: vec![],
            transforms: vec![],
        }
    }

    pub fn test<V: Fn(&T) -> Result<()> + 'static>(&mut self, test: V) {
        self.tests.push(Box::new(test));
    }

    pub fn transform(&mut self, transform: fn(T) -> T) {
        self.transforms.push(transform);
    }

    pub fn exec(&self, value: Option<Json>) -> Result<Option<Json>> {
        let json_type = self.json_type;
        let value_type = JsonType::from(&value);
        let json = match value {
            None if self.is_optional => return Ok(None),
            None => return Err(error::type_error(json_type, value_type)),
            Some(Json::Null) if self.is_nullable => return Ok(Some(Json::Null)),
            Some(Json::Null) => return Err(error::type_error(json_type, value_type)),
            Some(json) => json_type.coerce(json)?,
        };
        let t = match from_json(json) {
            Ok(t) => self
                .transforms
                .iter()
                .fold(t, |transformed, transform| transform(transformed)),
            Err(_) => return Err(error::type_error(json_type, value_type)),
        };
        let errors = self
            .tests
            .iter()
            .filter_map(|test| test(&t).err())
            .collect::<Vec<ValidationError>>();
        if errors.is_empty() {
            return Ok(Some(to_json(t).unwrap()));
        }
        Err(error::field_error(errors))
    }
}
