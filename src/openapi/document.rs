use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAPI {
    /// REQUIRED. This string MUST be the semantic version number of the
    /// OpenAPI Specification version that the OpenAPI document uses.
    /// The openapi field SHOULD be used by tooling specifications and
    /// clients to interpret the OpenAPI document. This is not related to
    /// the API info.version string.
    pub openapi: String,

    pub info: Info,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    pub paths: Paths,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

/// Adds metadata to a single tag that is used by the
/// Operation Object. It is not mandatory to have a
/// Tag Object per tag defined in the Operation Object instances.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Tag {
    /// REQUIRED. The name of the tag.
    pub name: String,
    /// A short description for the tag.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Inline extensions to this object.
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}
