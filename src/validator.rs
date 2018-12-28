use super::{
    error::{self, Result},
    json::{from_json, to_json, Json, JsonType},
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Display;

pub struct Validator<T: Display + DeserializeOwned + Serialize> {
    json_type: JsonType,
    pub is_optional: bool,
    pub is_nullable: bool,
    pub validations: Vec<Box<Fn(&T) -> Result<Option<T>>>>,
}

impl<T: Display + DeserializeOwned + Serialize> Validator<T> {
    pub fn new(json_type: JsonType) -> Self {
        Validator {
            json_type,
            is_optional: false,
            is_nullable: false,
            validations: vec![],
        }
    }

    pub fn exec(&self, value: Option<Json>) -> Result<Option<Json>> {
        let value_type = JsonType::from(&value);
        let json = match value {
            None if self.is_optional => return Ok(None),
            Some(Json::Null) if self.is_nullable => return Ok(Some(Json::Null)),
            None => return Err(error::type_error(self.json_type, value_type)),
            Some(Json::Null) => return Err(error::type_error(self.json_type, value_type)),
            Some(json) => json,
        };
        let mut coerced = match from_json(self.json_type.coerce(json)?) {
            Ok(coerced) => coerced,
            Err(_) => return Err(error::type_error(self.json_type, value_type)),
        };

        if self.validations.is_empty() {
            return Ok(Some(to_json(coerced).unwrap()));
        }

        let mut errors = vec![];

        self.validations.iter().for_each(|validation| {
            match validation(&coerced) {
                Ok(None) => (),
                Ok(Some(validated)) => coerced = validated,
                Err(error) => errors.push(error),
            };
        });

        if errors.is_empty() {
            return Ok(Some(to_json(coerced).unwrap()));
        }

        Err(error::field_error(errors))
    }
}
