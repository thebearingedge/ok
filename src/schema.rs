use super::{
    error::{json_error, Result, ValidationError, ValidationResult},
    json::Json,
};

pub trait OkSchema {
    fn label(self, label: &'static str) -> Self
    where
        Self: Sized;

    fn desc(self, description: &'static str) -> Self
    where
        Self: Sized;

    fn optional(self) -> Self
    where
        Self: Sized;

    fn nullable(self) -> Self
    where
        Self: Sized;

    fn validate_at(
        &self,
        path: &str,
        value: Option<Json>,
        all_errors: &mut Vec<ValidationError>,
    ) -> ValidationResult<Option<Json>>;

    fn validate(&self, json: Json) -> Result<Json> {
        let mut errors = vec![];
        match self.validate_at("", Some(json), &mut errors) {
            Ok(value) => Ok(value.unwrap()),
            Err(_) => Err(json_error(errors)),
        }
    }
}
