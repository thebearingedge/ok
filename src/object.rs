use super::{
    array::ArraySchema,
    boolean::BooleanSchema,
    error::{ValidationError, ValidationResult},
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

    fn validate_at(
        &self,
        path: &str,
        value: Option<Json>,
        all_errors: &mut Vec<ValidationError>,
    ) -> ValidationResult<Option<Json>> {
        let mut errors = vec![];
        let validated = self.validator.exec(path, value, &mut errors);
        let mut fields = match validated {
            Ok(None) => return Ok(None),
            Ok(Some(json)) => {
                if self.property_schemas.is_empty() {
                    return Ok(Some(json));
                }
                from_json::<Object>(json).unwrap()
            }
            Err(_) => return Err(all_errors.append(&mut errors)),
        };
        let mut object = Object::new();
        self.property_schemas.iter().for_each(|(key, schema)| {
            let path = match path {
                "" => key.to_string(),
                path => format!("{}.{}", path, key),
            };
            match schema.validate_at(path.as_str(), fields.remove(key), &mut errors) {
                Ok(None) | Err(_) => (),
                Ok(Some(value)) => {
                    if errors.is_empty() {
                        object.insert(key.to_string(), value);
                    }
                }
            };
        });
        if errors.is_empty() {
            return Ok(Some(object.into()));
        }
        Err(all_errors.append(&mut errors))
    }
}

pub fn object() -> ObjectSchema {
    ObjectSchema::new()
}

#[cfg(test)]
mod tests {
    use super::super::{
        error::{payload_error, type_error},
        json::JsonType,
        object, OkSchema,
    };
    use serde_json::json;

    #[test]
    fn it_validates_objects() {
        let schema = object();
        assert_eq!(schema.validate(Some(json!({}))), Ok(Some(json!({}))));
        assert_eq!(
            schema.validate(Some(json!(null))),
            Err(payload_error(vec![type_error("", "", JsonType::Object)]))
        );
        assert_eq!(
            schema.validate(None),
            Err(payload_error(vec![type_error("", "", JsonType::Object)]))
        );
        assert_eq!(
            schema.validate(Some(json!([]))),
            Err(payload_error(vec![type_error("", "", JsonType::Object)]))
        );
        assert_eq!(
            schema.validate(Some(json!(1))),
            Err(payload_error(vec![type_error("", "", JsonType::Object)]))
        );
        assert_eq!(
            schema.validate(Some(json!(true))),
            Err(payload_error(vec![type_error("", "", JsonType::Object)]))
        );
        assert_eq!(
            schema.validate(Some(json!("foo"))),
            Err(payload_error(vec![type_error("", "", JsonType::Object)]))
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
        let schema = object().boolean("foo", |field| field.desc("A Boolean value."));
        assert_eq!(
            schema.validate(Some(json!({ "foo": true }))),
            Ok(Some(json!({ "foo": true })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": "bar" }))),
            Err(payload_error(vec![type_error(
                "foo",
                "foo",
                JsonType::Boolean
            ),]))
        );
    }

    #[test]
    fn it_validates_number_fields() {
        let schema = object().integer("foo", |field| field.desc("An integer."));
        assert_eq!(
            schema.validate(Some(json!({ "foo": 1 }))),
            Ok(Some(json!({ "foo": 1 })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": "" }))),
            Err(payload_error(vec![type_error(
                "foo",
                "foo",
                JsonType::Integer
            )]))
        );

        let schema = object().float("foo", |field| field.desc("A float."));
        assert_eq!(
            schema.validate(Some(json!({ "foo": 1.0 }))),
            Ok(Some(json!({ "foo": 1.0 })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": "" }))),
            Err(payload_error(vec![type_error(
                "foo",
                "foo",
                JsonType::Float
            )]))
        );

        let schema = object().unsigned("foo", |field| field.desc("An unsigned."));
        assert_eq!(
            schema.validate(Some(json!({ "foo": 1 }))),
            Ok(Some(json!({ "foo": 1 })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": "" }))),
            Err(payload_error(vec![type_error(
                "foo",
                "foo",
                JsonType::Unsigned
            )]))
        );
    }

    #[test]
    fn it_validates_string_fields() {
        let schema = object().string("foo", |field| field.desc("A String value."));
        assert_eq!(
            schema.validate(Some(json!({ "foo": "bar" }))),
            Ok(Some(json!({ "foo": "bar" })))
        );
        assert_eq!(
            schema.validate(Some(json!({ "foo": null }))),
            Err(payload_error(vec![type_error(
                "foo",
                "foo",
                JsonType::String
            ),]))
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
            Err(payload_error(vec![type_error(
                "foo",
                "foo",
                JsonType::Object
            )]))
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
            Err(payload_error(vec![type_error(
                "foo",
                "foo",
                JsonType::Array
            )]))
        );
    }
}
