use super::response::ResponseDiff;
use openapiv3::{ReferenceOr, Response, Responses, StatusCode};
use serde::Serialize;
use std::collections::HashMap;

type ResponseCodePair = (StatusCode, ReferenceOr<Response>);

#[derive(Debug, Serialize)]
pub struct ResponsesDiff {
    added: Vec<ResponseCodePair>,
    removed: Vec<ResponseCodePair>,
    changed: HashMap<StatusCode, ResponseDiff>,
}

impl ResponsesDiff {
    pub fn has_changes(&self) -> bool {
        // TODO
        true
    }

    pub fn from_responses(base: &Responses, head: &Responses) -> Self {
        let mut responses_added = vec![];
        let mut responses_removed = vec![];
        let mut responses_changed: HashMap<StatusCode, ResponseDiff> = HashMap::new();

        for (status_code, response) in &base.responses {
            match head.responses.get(status_code) {
                Some(head_response) => {
                    let response_diff = ResponseDiff::from_responses(&response, &head_response);

                    if response_diff.has_changes() {
                        responses_changed.insert(status_code.clone(), response_diff);
                    }
                }
                None => responses_removed.push((status_code.clone(), response.clone())),
            }
        }

        for (status_code, response) in &head.responses {
            match base.responses.get(status_code) {
                Some(_) => {}
                None => responses_added.push((status_code.clone(), response.clone())),
            }
        }

        Self {
            added: responses_added,
            removed: responses_removed,
            changed: responses_changed,
        }
    }
}
