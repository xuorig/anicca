use super::common::{OptionalStringDiff, StringDiff};
use super::extensions::ExtensionsDiff;
use crate::openapi::{Contact, Info, License};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct InfoDiff {
    pub title: Option<StringDiff>,
    pub description: Option<OptionalStringDiff>,
    pub terms_of_service: Option<OptionalStringDiff>,
    pub contact: Option<ContactDiff>,
    pub license: Option<LicenseDiff>,
    pub version: Option<StringDiff>,
    pub extensions: Option<ExtensionsDiff>,
}

impl InfoDiff {
    pub fn has_changes(&self) -> bool {
        true
    }

    pub fn from_info(base: &Info, head: &Info) -> Self {
        let mut diff = Self::default();

        diff.title = StringDiff::from_strings(base.title.clone(), head.title.clone());
        diff.description = OptionalStringDiff::from_strings(&base.description, &head.description);
        diff.terms_of_service =
            OptionalStringDiff::from_strings(&base.terms_of_service, &head.terms_of_service);

        let contact_diff = ContactDiff::from_contacts(&base.contact, &head.contact);
        if contact_diff.has_changes() {
            diff.contact = Some(contact_diff);
        }

        let license_diff = LicenseDiff::from_licences(&base.license, &head.license);
        if license_diff.has_changes() {
            diff.license = Some(license_diff);
        }

        let extensions_diff = ExtensionsDiff::from_extensions(&base.extensions, &head.extensions);
        if extensions_diff.has_changes() {
            diff.extensions = Some(extensions_diff);
        }

        diff
    }
}

#[derive(Debug, Serialize, Default)]
pub struct ContactDiff {
    pub added: Option<Contact>,
    pub removed: Option<Contact>,
    pub name: Option<OptionalStringDiff>,
    pub url: Option<OptionalStringDiff>,
    pub email: Option<OptionalStringDiff>,
    pub extensions: Option<ExtensionsDiff>,
}

impl ContactDiff {
    pub fn has_changes(&self) -> bool {
        self.added.is_some()
            || self.removed.is_some()
            || self.name.is_some()
            || self.url.is_some()
            || self.email.is_some()
            || self.extensions.is_some()
    }

    pub fn from_contacts(base: &Option<Contact>, head: &Option<Contact>) -> Self {
        match base {
            Some(base) => match head {
                Some(head) => {
                    let mut diff = Self::default();

                    diff.name = OptionalStringDiff::from_strings(&base.name, &head.name);
                    diff.url = OptionalStringDiff::from_strings(&base.url, &head.url);
                    diff.email = OptionalStringDiff::from_strings(&base.email, &head.email);

                    let extensions_diff =
                        ExtensionsDiff::from_extensions(&base.extensions, &head.extensions);
                    if extensions_diff.has_changes() {
                        diff.extensions = Some(extensions_diff);
                    }

                    diff
                }
                None => Self {
                    removed: Some(base.clone()),
                    ..Default::default()
                },
            },
            None => match head {
                Some(head) => Self {
                    added: Some(head.clone()),
                    ..Default::default()
                },
                None => Self::default(),
            },
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct LicenseDiff {
    pub added: Option<License>,
    pub removed: Option<License>,
    pub name: Option<StringDiff>,
    pub url: Option<OptionalStringDiff>,
    pub extensions: Option<ExtensionsDiff>,
}

impl LicenseDiff {
    pub fn has_changes(&self) -> bool {
        false
    }

    pub fn from_licences(base: &Option<License>, head: &Option<License>) -> Self {
        match base {
            Some(base) => match head {
                Some(head) => {
                    let mut diff = Self::default();

                    diff.name = StringDiff::from_strings(base.name.clone(), head.name.clone());
                    diff.url = OptionalStringDiff::from_strings(&base.url, &head.url);

                    let extensions_diff =
                        ExtensionsDiff::from_extensions(&base.extensions, &head.extensions);
                    if extensions_diff.has_changes() {
                        diff.extensions = Some(extensions_diff);
                    }

                    diff
                }
                None => Self {
                    removed: Some(base.clone()),
                    ..Default::default()
                },
            },
            None => match head {
                Some(head) => Self {
                    added: Some(head.clone()),
                    ..Default::default()
                },
                None => Self::default(),
            },
        }
    }
}
