use super::{
    array::ArraySchema,
    boolean::BooleanSchema,
    error::{self, Result},
    json::{Json, JsonType, Object},
    number::NumberSchema,
    string::StringSchema,
    OkSchema, Validator,
};
use std::collections::HashMap;

pub struct ObjectSchema {
    description: Option<&'static str>,
    validator: Validator<Object>,
    field_schemas: HashMap<String, Box<OkSchema>>,
}

impl ObjectSchema {
    pub fn new() -> Self {
        ObjectSchema {
            description: None,
            validator: Validator::new(JsonType::Object),
            field_schemas: HashMap::new(),
        }
    }

    pub fn boolean<K: Into<String>>(
        mut self,
        key: K,
        builder: fn(BooleanSchema) -> BooleanSchema,
    ) -> Self {
        let schema = BooleanSchema::new();
        self.field_schemas
            .insert(key.into(), Box::new(builder(schema)));
        self
    }

    pub fn integer<K: Into<String>>(
        mut self,
        key: K,
        builder: fn(NumberSchema<i64>) -> NumberSchema<i64>,
    ) -> Self {
        let schema = NumberSchema::new(JsonType::Integer);
        self.field_schemas
            .insert(key.into(), Box::new(builder(schema)));
        self
    }

    pub fn float<K: Into<String>>(
        mut self,
        key: K,
        builder: fn(NumberSchema<f64>) -> NumberSchema<f64>,
    ) -> Self {
        let schema = NumberSchema::new(JsonType::Float);
        self.field_schemas
            .insert(key.into(), Box::new(builder(schema)));
        self
    }

    pub fn unsigned<K: Into<String>>(
        mut self,
        key: K,
        builder: fn(NumberSchema<u64>) -> NumberSchema<u64>,
    ) -> Self {
        let schema = NumberSchema::new(JsonType::Unsigned);
        self.field_schemas
            .insert(key.into(), Box::new(builder(schema)));
        self
    }

    pub fn string<K: Into<String>>(
        mut self,
        key: K,
        builder: fn(StringSchema) -> StringSchema,
    ) -> Self {
        let schema = StringSchema::new();
        self.field_schemas
            .insert(key.into(), Box::new(builder(schema)));
        self
    }

    pub fn object<K: Into<String>>(
        mut self,
        key: K,
        builder: fn(ObjectSchema) -> ObjectSchema,
    ) -> Self {
        let schema = ObjectSchema::new();
        self.field_schemas
            .insert(key.into(), Box::new(builder(schema)));
        self
    }

    pub fn array<K: Into<String>>(
        mut self,
        key: K,
        builder: fn(ArraySchema) -> ArraySchema,
    ) -> Self {
        let schema = ArraySchema::new();
        self.field_schemas
            .insert(key.into(), Box::new(builder(schema)));
        self
    }
}

impl OkSchema for ObjectSchema {
    fn desc(mut self, description: &'static str) -> Self {
        self.description = Some(description);
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

    fn validate(&self, value: Option<Json>) -> Result<Option<Json>> {
        let validated = self.validator.exec(value)?;
        if self.field_schemas.is_empty() {
            return Ok(validated);
        }
        let mut fields = match validated {
            None => return Ok(None),
            Some(json) => match json {
                Json::Object(fields) => fields,
                _ => return Ok(Some(json)),
            },
        };
        let mut object = Object::new();
        let mut errors = HashMap::new();
        self.field_schemas.iter().for_each(|(key, schema)| {
            match schema.validate(fields.remove(key)) {
                Ok(None) => (),
                Ok(Some(value)) => {
                    if errors.is_empty() {
                        object.insert(key.to_string(), value);
                    }
                }
                Err(error) => {
                    errors.insert(key.to_string(), error);
                }
            };
        });
        if errors.is_empty() {
            return Ok(Some(object.into()));
        }
        Err(error::object_error(errors))
    }
}

pub fn object() -> ObjectSchema {
    ObjectSchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{error, json::JsonType, object, OkSchema};
    use maplit::hashmap;
    use serde_json::json;

    #[test]
    fn it_validates_objects() {
        let schema = object();

        assert_eq!(schema.validate(Some(json!({}))), Ok(Some(json!({}))));
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(error::type_error(JsonType::Object, JsonType::Null))
        );
        assert_eq!(
            schema.validate(None),
            Err(error::type_error(JsonType::Object, JsonType::None))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(error::type_error(JsonType::Object, JsonType::Array))
        );
        assert_eq!(
            schema.validate(Some(json!(1))),
            Err(error::type_error(JsonType::Object, JsonType::Number))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(error::type_error(JsonType::Object, JsonType::Boolean))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(error::type_error(JsonType::Object, JsonType::String))
        );
    }

    #[test]
    fn it_validates_optional_objects() {
        let schema = object().optional();

        assert_eq!(schema.validate(Some(json!({}))), Ok(Some(json!({}))));
        assert_eq!(schema.validate(None), Ok(None));
    }

    #[test]
    fn it_validates_nullable_objects() {
        let schema = object().nullable();

        assert_eq!(schema.validate(Some(json!({}))), Ok(Some(json!({}))));
        assert_eq!(schema.validate(Some(json!(null))), Ok(Some(json!(null))));
    }

    #[test]
    fn it_validates_boolean_fields() {
        let schema = object()
            .boolean("foo", |field| field.desc("A Boolean value."))
            .boolean("bar", |field| field.desc("Another Boolean value."));

        assert_eq!(
            schema.validate(Some(json!({ "foo": true, "bar": false }))),
            Ok(Some(json!({ "foo": true, "bar": false })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": "bar", "baz": true }))),
            Err(error::object_error(hashmap! {
                "foo".into() => error::type_error(JsonType::Boolean, JsonType::String),
                "bar".into() => error::type_error(JsonType::Boolean, JsonType::None)
            }))
        );
    }

    #[test]
    fn it_validates_number_fields() {
        let schema = object()
            .integer("foo", |field| field.desc("An integer."))
            .float("bar", |field| field.desc("A float."))
            .unsigned("baz", |field| field.desc("An unsigned."));

        assert_eq!(
            schema.validate(Some(json!({ "foo": 1, "bar": 2.0, "baz": 3 }))),
            Ok(Some(json!({ "foo": 1, "bar": 2.0, "baz": 3 })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": "", "baz": null }))),
            Err(error::object_error(hashmap! {
                "foo".into() => error::type_error(JsonType::Integer, JsonType::String),
                "bar".into() => error::type_error(JsonType::Float, JsonType::None),
                "baz".into() => error::type_error(JsonType::Unsigned, JsonType::Null)
            }))
        );
    }

    #[test]
    fn it_validates_string_fields() {
        let schema = object()
            .string("foo", |field| field.desc("A String value."))
            .string("baz", |field| field.desc("Another String value."));

        assert_eq!(
            schema.validate(Some(json!({ "foo": "bar", "baz": "qux" }))),
            Ok(Some(json!({ "foo": "bar", "baz": "qux" })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": null, "baz": [] }))),
            Err(error::object_error(hashmap! {
                "foo".into() => error::type_error(JsonType::String, JsonType::Null),
                "baz".into() => error::type_error(JsonType::String, JsonType::Array)
            }))
        );
    }

    #[test]
    fn it_validates_object_fields() {
        let schema = object().object("foo", |field| field.desc("A nested Object."));

        assert_eq!(
            schema.validate(Some(json!({ "foo": {} }))),
            Ok(Some(json!({ "foo": {} })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": true }))),
            Err(error::object_error(hashmap! {
                "foo".into() => error::type_error(JsonType::Object, JsonType::Boolean)
            }))
        );
    }

    #[test]
    fn it_validates_array_fields() {
        let schema = object().array("foo", |field| field.desc("A nested Object."));

        assert_eq!(
            schema.validate(Some(json!({ "foo": [] }))),
            Ok(Some(json!({ "foo": [] })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": true }))),
            Err(error::object_error(hashmap! {
                "foo".into() => error::type_error(JsonType::Array, JsonType::Boolean)
            }))
        );
    }
}
