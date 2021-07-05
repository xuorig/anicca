use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Info {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    pub version: String,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// Contact information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    /// MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    /// MUST be in the format of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Inline extensions to this object.
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

/// License information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct License {
    /// REQUIRED. The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API. MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Inline extensions to this object.
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
