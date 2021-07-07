use super::common::OptionalStringDiff;
use super::extensions::ExtensionsDiff;
use crate::openapi::Server;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ServersDiff {
    pub added: Vec<Server>,
    pub removed: Vec<Server>,
    pub changed: HashMap<String, ServerDiff>,
}

impl ServersDiff {
    pub fn from_servers(base: &[Server], head: &[Server]) -> Self {
        let mut servers_added = vec![];
        let mut servers_removed = vec![];
        let mut servers_changed: HashMap<String, ServerDiff> = HashMap::new();

        let base_map = base.iter().fold(HashMap::new(), |mut acc, server| {
            acc.insert(server.url.clone(), server.clone());
            acc
        });

        let head_map = head.iter().fold(HashMap::new(), |mut acc, server| {
            acc.insert(server.url.clone(), server.clone());
            acc
        });

        for (url, server) in &base_map {
            match head_map.get(url) {
                Some(head_server) => {
                    let server_diff = ServerDiff::from_servers(&server, &head_server);

                    if server_diff.has_changes() {
                        servers_changed.insert(url.clone(), server_diff);
                    }
                }
                None => servers_removed.push(server.clone()),
            }
        }

        for (url, server) in &head_map {
            match base_map.get(url) {
                Some(_) => {}
                None => servers_added.push(server.clone()),
            }
        }

        Self {
            added: servers_added,
            removed: servers_removed,
            changed: servers_changed,
        }
    }

    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty() || !self.changed.is_empty()
    }
}

#[derive(Debug, Serialize, Default)]
pub struct ServerDiff {
    pub description: Option<OptionalStringDiff>,
    pub extensions: Option<ExtensionsDiff>,
}

impl ServerDiff {
    pub fn from_servers(base: &Server, head: &Server) -> Self {
        let mut diff = Self::default();

        diff.description = OptionalStringDiff::from_strings(&base.description, &head.description);

        let extensions_diff = ExtensionsDiff::from_extensions(&base.extensions, &head.extensions);
        if extensions_diff.has_changes() {
            diff.extensions = Some(extensions_diff);
        }

        // TODO: server variables diff

        diff
    }

    pub fn has_changes(&self) -> bool {
        self.description.is_some() || self.extensions.is_some()
    }
}
