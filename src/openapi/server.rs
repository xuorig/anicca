use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Server {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, ServerVariable>>,
    #[serde(flatten)]
    pub extensions:BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ServerVariable {
    #[serde(rename = "enum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<String>,
    pub default: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions:BTreeMap<String, serde_json::Value>,
}
