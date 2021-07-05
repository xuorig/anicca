mod components;
mod document;
mod documentation;
mod examples;
mod info;
mod media_type;
mod operation;
mod parameter;
mod paths;
mod reference;
mod request_body;
mod responses;
mod schema;
mod security;
mod server;
mod status_code;

pub use components::*;
pub use document::*;
pub use documentation::*;
pub use examples::*;
pub use info::*;
pub use media_type::*;
pub use operation::*;
pub use parameter::*;
pub use paths::*;
pub use reference::*;
pub use request_body::*;
pub use responses::*;
pub use schema::*;
pub use security::*;
pub use server::*;
pub use status_code::*;

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
