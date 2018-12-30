mod boolean;
mod error;
mod json;
mod number;
mod object;
mod schema;
mod string;
mod validator;

use self::validator::Validator;

pub use self::boolean::boolean;
pub use self::number::{float, integer, unsigned};
pub use self::object::object;
pub use self::schema::OkSchema;
pub use self::string::string;
