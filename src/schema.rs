use super::{error::Result, json::Json};

pub trait OkSchema {
    fn desc(self, description: &'static str) -> Self
    where
        Self: Sized;

    fn optional(self) -> Self
    where
        Self: Sized;

    fn nullable(self) -> Self
    where
        Self: Sized;

    fn validate(&self, value: Option<Json>) -> Result<Option<Json>>;
}
