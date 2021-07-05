use super::reference::ReferenceOr;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// The Schema Object allows the definition of input and output data types.
/// These types can be objects, but also primitives and arrays.
/// This object is an extended subset of the
/// [JSON Schema Specification Wright Draft 00](http://json-schema.org/).
/// For more information about the properties, see
/// [JSON Schema Core](https://tools.ietf.org/html/draft-wright-json-schema-00) and
/// [JSON Schema Validation](https://tools.ietf.org/html/draft-wright-json-schema-validation-00).
/// Unless stated otherwise, the property definitions follow the JSON Schema.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#schemaObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub schema_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<Option<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, Schema>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "readOnly")]
    pub read_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,

    /// Value can be boolean or object. Inline or referenced schema MUST be of a
    /// [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#schemaObject)
    /// and not a standard JSON Schema.
    ///
    /// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#properties>.
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "additionalProperties"
    )]
    pub additional_properties: Option<AdditionalProperties>,

    /// A free-form property to include an example of an instance for this schema.
    /// To represent examples that cannot be naturally represented in JSON or YAML,
    /// a string value can be used to contain the example with escaping where necessary.
    /// NOTE: According to [spec], _Primitive data types in the OAS are based on the
    ///       types supported by the JSON Schema Specification Wright Draft 00._
    ///       This suggest using
    ///       [`serde_json::Value`](https://docs.serde.rs/serde_json/value/enum.Value.html). [spec][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#data-types]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::value::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The default value represents what would be assumed by the consumer of the input as the value
    /// of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the
    /// defined type for the Schema Object defined at the same level. For example, if type is
    /// `string`, then `default` can be `"foo"` but cannot be `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Value>,

    /// Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    /// JSON Schema.
    /// [allOf](https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#allof)
    #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<ReferenceOr<Schema>>>,

    /// Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    /// JSON Schema.
    /// [oneOf](https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#oneof)
    #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<ReferenceOr<Schema>>>,

    /// Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    /// JSON Schema.
    /// [anyOf](https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#anyof)
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<ReferenceOr<Schema>>>,

    /// Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    /// JSON Schema.
    /// [not](https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#not)
    #[serde(rename = "not", skip_serializing_if = "Option::is_none")]
    pub not: Option<Vec<ReferenceOr<Schema>>>,

    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,

    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,

    /// [Specification extensions](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.2.md#specificationExtensions)
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Any(bool),
    Schema(Box<ReferenceOr<Schema>>),
}
