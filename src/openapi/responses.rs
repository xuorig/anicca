use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Responses {
    /// The documentation of responses other than the ones declared
    /// for specific HTTP response codes. Use this field to cover
    /// undeclared responses. A Reference Object can link to a response
    /// that the OpenAPI Object's components/responses section defines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ReferenceOr<Response>>,
    /// Any HTTP status code can be used as the property name,
    /// but only one property per code, to describe the expected
    /// response for that HTTP status code. A Reference Object
    /// can link to a response that is defined in the OpenAPI Object's
    /// components/responses section. This field MUST be enclosed in
    /// quotation marks (for example, "200") for compatibility between
    /// JSON and YAML. To define a range of response codes, this field
    /// MAY contain the uppercase wildcard character X. For example,
    /// 2XX represents all response codes between [200-299]. The following
    /// range definitions are allowed: 1XX, 2XX, 3XX, 4XX, and 5XX.
    /// If a response range is defined using an explicit code, the
    /// explicit code definition takes precedence over the range
    /// definition for that code.
    #[serde(flatten)]
    #[serde(default)]
    pub responses: BTreeMap<StatusCode, ReferenceOr<Response>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Response {
    /// REQUIRED. A short description of the response.
    /// CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,

    /// Maps a header name to its definition.
    /// RFC7230 states header names are case insensitive.
    /// If a response header is defined with the name "Content-Type",
    /// it SHALL be ignored.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, ReferenceOr<Header>>,

    /// A map containing descriptions of potential response payloads.
    /// The key is a media type or media type range and the value
    /// describes it. For responses that match multiple keys,
    /// only the most specific key is applicable. e.g. text/plain
    /// overrides text/*
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub content: BTreeMap<String, MediaType>,

    /// A map of operations links that can be followed from the response.
    /// The key of the map is a short name for the link, following
    /// the naming constraints of the names for Component Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, Link>,

    /// Inline extensions to this object.
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

/// The Link object represents a possible design-time link for a response.
/// The presence of a link does not guarantee the caller's ability to
/// successfully invoke it, rather it provides a known relationship and
/// traversal mechanism between responses and other operations.
///
/// Unlike dynamic links (i.e. links provided in the response payload),
/// the OAS linking mechanism does not require link information in the runtime response.
///
/// For computing links, and providing instructions to execute them,
/// a runtime expression is used for accessing values in an operation
/// and using them as parameters while invoking the linked operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename = "camelCase")]
pub struct Link {
    /// A description of the link.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A relative or absolute reference to an OAS operation.
    /// This field is mutually exclusive of the operationId field,
    /// and MUST point to an Operation Object. Relative operationRef
    /// values MAY be used to locate an existing Operation Object
    /// in the OpenAPI definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,
    /// The name of an existing, resolvable OAS operation,
    /// as defined with a unique operationId. This field is
    /// mutually exclusive of the operationRef field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// A literal value or {expression} to use as a request body
    /// when calling the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<String>,
    /// A map representing parameters to pass to an operation
    /// as specified with operationId or identified via operationRef.
    /// The key is the parameter name to be used, whereas the value
    /// can be a constant or an expression to be evaluated and passed
    /// to the linked operation. The parameter name can be qualified
    /// using the parameter location [{in}.]{name} for operations
    /// that use the same parameter name in different locations (e.g. path.id).
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, String>,
    /// A server object to be used by the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    /// Inline extensions to this object.
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}
