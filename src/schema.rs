use super::{error::Result, json::Json};

pub trait OkSchema {
    fn optional(self) -> Self;
    fn nullable(self) -> Self;
    fn validate(&self, value: Option<Json>) -> Result<Option<Json>>;
}
