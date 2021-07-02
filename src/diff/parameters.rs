use openapiv3::{Parameter, ReferenceOr};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ParametersDiff {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    added: Vec<ReferenceOr<Parameter>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    removed: Vec<ReferenceOr<Parameter>>,
}

impl ParametersDiff {
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty()
    }

    pub fn param_name(param: &Parameter) -> String {
        match param {
            Parameter::Query {
                parameter_data,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => parameter_data.name.clone(),
            Parameter::Header {
                parameter_data,
                style: _,
            } => parameter_data.name.clone(),
            Parameter::Path {
                parameter_data,
                style: _,
            } => parameter_data.name.clone(),
            Parameter::Cookie {
                parameter_data,
                style: _,
            } => parameter_data.name.clone(),
        }
    }

    pub fn from_params(
        base: &Vec<ReferenceOr<Parameter>>,
        head: &Vec<ReferenceOr<Parameter>>,
    ) -> Self {
        let mut added = vec![];
        let mut removed = vec![];

        for ref_or_param in base {
            match ref_or_param {
                ReferenceOr::Reference { reference } => {
                    let ref_match = head.iter().find(|p| match p {
                        ReferenceOr::Reference { reference: r } => r == reference,
                        ReferenceOr::Item(_) => false,
                    });

                    match ref_match {
                        Some(_param) => {
                            // Do the diff
                        }
                        None => {
                            // Removed
                            removed.push(ref_or_param.clone())
                        }
                    }
                }
                ReferenceOr::Item(param) => {
                    let param_match = head.iter().find(|p| match p {
                        ReferenceOr::Reference { reference: _ } => false,
                        ReferenceOr::Item(p) => Self::param_name(p) == Self::param_name(param),
                    });

                    match param_match {
                        Some(_param) => {
                            // Do the diff
                        }
                        None => {
                            // Removed
                            removed.push(ref_or_param.clone())
                        }
                    }
                }
            }
        }

        for ref_or_param in head {
            match ref_or_param {
                ReferenceOr::Reference { reference } => {
                    let ref_match = base.iter().find(|p| match p {
                        ReferenceOr::Reference { reference: r } => r == reference,
                        ReferenceOr::Item(_) => false,
                    });

                    match ref_match {
                        Some(_param) => {}
                        None => {
                            // added
                            added.push(ref_or_param.clone())
                        }
                    }
                }
                ReferenceOr::Item(param) => {
                    let param_match = base.iter().find(|p| match p {
                        ReferenceOr::Reference { reference: _ } => false,
                        ReferenceOr::Item(p) => Self::param_name(p) == Self::param_name(param),
                    });

                    match param_match {
                        Some(_param) => {}
                        None => {
                            // Added
                            added.push(ref_or_param.clone())
                        }
                    }
                }
            }
        }

        Self { added, removed }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indexmap::IndexMap;
    use openapiv3::{
        Operation, ParameterData, ParameterSchemaOrContent, QueryStyle, Schema, SchemaData,
        SchemaKind, StringType, Type,
    };

    #[test]
    fn added_parameter() {
        let base_operation = Operation::default();
        let mut head_operation = Operation::default();
        let my_param = Parameter::Query {
            parameter_data: ParameterData {
                name: String::from("myParam"),
                description: Some(String::from("myParam")),
                deprecated: None,
                example: None,
                examples: IndexMap::default(),
                extensions: IndexMap::default(),
                format: ParameterSchemaOrContent::Schema(ReferenceOr::Item(Schema {
                    schema_data: SchemaData::default(),
                    schema_kind: SchemaKind::Type(Type::String(StringType::default())),
                })),
                required: true,
            },
            allow_empty_value: None,
            allow_reserved: false,
            style: QueryStyle::Form,
        };

        head_operation.parameters.push(ReferenceOr::Item(my_param));

        let diff =
            ParametersDiff::from_params(&base_operation.parameters, &head_operation.parameters);

        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.removed.len(), 0);

        match diff.added.first().unwrap() {
            ReferenceOr::Item(p) => match p {
                Parameter::Query {
                    parameter_data,
                    allow_reserved: _,
                    style: _,
                    allow_empty_value: _,
                } => {
                    assert_eq!(parameter_data.name, "myParam")
                }
                _ => {
                    panic!("Unexpected parameter type")
                }
            },
            _ => {
                panic!("Unexpected parameter type")
            }
        }
    }

    #[test]
    fn removed_parameter() {
        let mut base_operation = Operation::default();
        let head_operation = Operation::default();

        let my_param = Parameter::Query {
            parameter_data: ParameterData {
                name: String::from("myParam"),
                description: Some(String::from("myParam")),
                deprecated: None,
                example: None,
                examples: IndexMap::default(),
                extensions: IndexMap::default(),
                format: ParameterSchemaOrContent::Schema(ReferenceOr::Item(Schema {
                    schema_data: SchemaData::default(),
                    schema_kind: SchemaKind::Type(Type::String(StringType::default())),
                })),
                required: true,
            },
            allow_empty_value: None,
            allow_reserved: false,
            style: QueryStyle::Form,
        };

        base_operation.parameters.push(ReferenceOr::Item(my_param));

        let diff =
            ParametersDiff::from_params(&base_operation.parameters, &head_operation.parameters);

        assert_eq!(diff.added.len(), 0);
        assert_eq!(diff.removed.len(), 1);

        match diff.removed.first().unwrap() {
            ReferenceOr::Item(p) => match p {
                Parameter::Query {
                    parameter_data,
                    allow_reserved: _,
                    style: _,
                    allow_empty_value: _,
                } => {
                    assert_eq!(parameter_data.name, "myParam")
                }
                _ => {
                    panic!("Unexpected parameter type")
                }
            },
            _ => {
                panic!("Unexpected parameter type")
            }
        }
    }
}
