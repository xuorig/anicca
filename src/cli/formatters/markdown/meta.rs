use super::extensions::ExtensionsPrinter;
use crate::diff::Diff;

pub struct MetaPrinter<'a> {
    pub diff: &'a Diff,
}

impl<'a> MetaPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        result.push_str("## General\n\n");

        if let Some(version_change) = &self.diff.version {
            result.push_str(
                format!(
                    "  - OpenAPI spec version changed from `{}` to `{}`.\n",
                    version_change.from, version_change.to
                )
                .as_str(),
            );
        }

        result.push('\n');

        if let Some(info_diff) = &self.diff.info {
            result.push_str("API info has changed:\n");

            if let Some(title_diff) = &info_diff.title {
                result.push_str(
                    format!(
                        "  - Title changed from `{}` to `{}`.\n",
                        title_diff.from, title_diff.to
                    )
                    .as_str(),
                );
            }

            if let Some(desc_diff) = &info_diff.description {
                result.push_str(
                    format!(
                        "  - Description changed from `{}` to `{}`.\n",
                        desc_diff.from.clone().unwrap_or(String::from("null")),
                        desc_diff.to.clone().unwrap_or(String::from("null"))
                    )
                    .as_str(),
                );
            }

            if let Some(terms) = &info_diff.terms_of_service {
                result.push_str(
                    format!(
                        "  - Terms of service changed from `{}` to `{}`.\n",
                        terms.from.clone().unwrap_or(String::from("null")),
                        terms.to.clone().unwrap_or(String::from("null"))
                    )
                    .as_str(),
                );
            }

            if let Some(version) = &info_diff.version {
                result.push_str(
                    format!(
                        "  - Version changed from `{}` to `{}`.\n",
                        version.from, version.to
                    )
                    .as_str(),
                );
            }

            if let Some(contact_diff) = &info_diff.contact {
                if contact_diff.added.is_some() {
                    result.push_str("  - API contact info was added.\n");
                } else if contact_diff.removed.is_some() {
                    result.push_str("  - API contact info was removed.\n");
                } else {
                    result.push_str("  - API contact info has changed:\n");
                }

                if let Some(contact_name) = &contact_diff.name {
                    result.push_str(
                        format!(
                            "    - Name changed from `{}` to `{}`.\n",
                            contact_name.from.clone().unwrap_or(String::from("null")),
                            contact_name.to.clone().unwrap_or(String::from("null"))
                        )
                        .as_str(),
                    );
                }

                if let Some(contact_url) = &contact_diff.url {
                    result.push_str(
                        format!(
                            "    - Url changed from `{}` to `{}`.\n",
                            contact_url.from.clone().unwrap_or(String::from("null")),
                            contact_url.to.clone().unwrap_or(String::from("null"))
                        )
                        .as_str(),
                    );
                }

                if let Some(contact_email) = &contact_diff.email {
                    result.push_str(
                        format!(
                            "    - Email changed from `{}` to `{}`.\n",
                            contact_email.from.clone().unwrap_or(String::from("null")),
                            contact_email.to.clone().unwrap_or(String::from("null"))
                        )
                        .as_str(),
                    );
                }

                if let Some(extensions_diff) = &contact_diff.extensions {
                    let extensions_printed_diff = ExtensionsPrinter {
                        extensions: extensions_diff,
                        indent: 4,
                    }
                    .print();
                    result.push_str(&extensions_printed_diff);
                }
            }

            if let Some(license_diff) = &info_diff.license {
                if license_diff.added.is_some() {
                    result.push_str("  - API license info was added.\n");
                } else if license_diff.removed.is_some() {
                    result.push_str("  - API license info was removed.\n");
                } else {
                    result.push_str("  - API license info has changed:\n");
                }

                if let Some(license_name) = &license_diff.name {
                    result.push_str(
                        format!(
                            "    - Name changed from `{}` to `{}`.\n",
                            license_name.from, license_name.to
                        )
                        .as_str(),
                    );
                }

                if let Some(license_url) = &license_diff.url {
                    result.push_str(
                        format!(
                            "    - URL changed from `{}` to `{}`.\n",
                            license_url.from.clone().unwrap_or(String::from("null")),
                            license_url.from.clone().unwrap_or(String::from("null")),
                        )
                        .as_str(),
                    );
                }

                if let Some(extensions_diff) = &license_diff.extensions {
                    let extensions_printed_diff = ExtensionsPrinter {
                        extensions: extensions_diff,
                        indent: 4,
                    }
                    .print();
                    result.push_str(&extensions_printed_diff);
                }
            }

            if let Some(extensions_diff) = &info_diff.extensions {
                let extensions_printed_diff = ExtensionsPrinter {
                    extensions: extensions_diff,
                    indent: 2,
                }
                .print();
                result.push_str(&extensions_printed_diff);
            }
        }

        result
    }
}
