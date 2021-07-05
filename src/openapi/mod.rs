pub mod components;
pub mod document;
pub mod documentation;
pub mod examples;
pub mod info;
pub mod media_type;
pub mod operation;
pub mod parameter;
pub mod paths;
pub mod reference;
pub mod request_body;
pub mod responses;
pub mod schema;
pub mod security;
pub mod server;
pub mod status_code;

use components::*;
use document::*;
use documentation::*;
use examples::*;
use info::*;
use media_type::*;
use operation::*;
use parameter::*;
use paths::*;
use reference::*;
use request_body::*;
use responses::*;
use schema::*;
use security::*;
use server::*;
use status_code::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dereferenced() {
        let json = include_bytes!("../../fixtures/api.github.com.deref.json");
        let parsed: OpenAPI = serde_json::from_slice(json).expect("Failed to parse JSON");
        assert_eq!("GitHub v3 REST API", parsed.info.title);
    }

    #[test]
    fn with_references() {
        let json = include_bytes!("../../fixtures/api.github.com.json");
        let parsed: OpenAPI = serde_json::from_slice(json).expect("Failed to parse JSON");
        assert_eq!("GitHub v3 REST API", parsed.info.title);
    }
}
