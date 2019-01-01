use super::{
    array::ArraySchema,
    boolean::BooleanSchema,
    error::{object_error, Result},
    json::{from_json, Json, JsonType, Object},
    number::NumberSchema,
    string::StringSchema,
    OkSchema, Validator,
};
use std::collections::HashMap;

pub struct ObjectSchema {
    validator: Validator<Object>,
    property_schemas: HashMap<String, Box<OkSchema>>,
}

impl ObjectSchema {
    pub fn new() -> Self {
        ObjectSchema {
            property_schemas: HashMap::new(),
            validator: Validator::new(JsonType::Object),
        }
    }

    pub fn key(mut self, key: &str, schema: impl OkSchema + 'static) -> Self {
        self.property_schemas.insert(key.into(), Box::new(schema));
        self
    }

    pub fn boolean(self, key: &str, build: fn(BooleanSchema) -> BooleanSchema) -> Self {
        self.key(key, build(BooleanSchema::new()))
    }

    pub fn integer(self, key: &str, build: fn(NumberSchema<i64>) -> NumberSchema<i64>) -> Self {
        self.key(key, build(NumberSchema::new(JsonType::Integer)))
    }

    pub fn float(self, key: &str, build: fn(NumberSchema<f64>) -> NumberSchema<f64>) -> Self {
        self.key(key, build(NumberSchema::new(JsonType::Float)))
    }

    pub fn unsigned(self, key: &str, build: fn(NumberSchema<u64>) -> NumberSchema<u64>) -> Self {
        self.key(key, build(NumberSchema::new(JsonType::Unsigned)))
    }

    pub fn string(self, key: &str, build: fn(StringSchema) -> StringSchema) -> Self {
        self.key(key, build(StringSchema::new()))
    }

    pub fn object(self, key: &str, build: fn(ObjectSchema) -> ObjectSchema) -> Self {
        self.key(key, build(ObjectSchema::new()))
    }

    pub fn array(self, key: &str, build: fn(ArraySchema) -> ArraySchema) -> Self {
        self.key(key, build(ArraySchema::new()))
    }
}

impl OkSchema for ObjectSchema {
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

    fn validate_at(&self, path: &str, value: Option<Json>) -> Result<Option<Json>> {
        let validated = self.validator.exec(path, value)?;
        let mut fields = match validated {
            None => return Ok(None),
            Some(_) if self.property_schemas.is_empty() => return Ok(validated),
            Some(json) => from_json::<Object>(json).unwrap(),
        };
        let mut object = Object::new();
        let mut errors = HashMap::new();
        self.property_schemas.iter().for_each(|(key, schema)| {
            let path = if path == "" {
                key.to_string()
            } else {
                format!("{}.{}", path, key)
            };
            match schema.validate_at(path.as_str(), fields.remove(key)) {
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
        Err(object_error(errors))
    }
}

pub fn object() -> ObjectSchema {
    ObjectSchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{
        error::{object_error, type_error},
        json::JsonType,
        object, OkSchema,
    };
    use maplit::hashmap;
    use serde_json::json;

    #[test]
    fn it_validates_objects() {
        let schema = object();
        assert_eq!(schema.validate(Some(json!({}))), Ok(Some(json!({}))));
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(type_error("", JsonType::Object))
        );
        assert_eq!(schema.validate(None), Err(type_error("", JsonType::Object)));
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(type_error("", JsonType::Object))
        );
        assert_eq!(
            schema.validate(Some(json!(1))),
            Err(type_error("", JsonType::Object))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(type_error("", JsonType::Object))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(type_error("", JsonType::Object))
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
            Err(object_error(hashmap! {
                "foo".into() => type_error("foo", JsonType::Boolean),
                "bar".into() => type_error("bar", JsonType::Boolean)
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
            Err(object_error(hashmap! {
                "foo".into() => type_error("foo", JsonType::Integer),
                "bar".into() => type_error("bar", JsonType::Float),
                "baz".into() => type_error("baz", JsonType::Unsigned)
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
            Err(object_error(hashmap! {
                "foo".into() => type_error("foo", JsonType::String),
                "baz".into() => type_error("baz", JsonType::String)
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
            Err(object_error(hashmap! {
                "foo".into() => type_error("foo", JsonType::Object)
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
            Err(object_error(hashmap! {
                "foo".into() => type_error("foo", JsonType::Array)
            }))
        );
    }
}
