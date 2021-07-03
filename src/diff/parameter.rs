use super::common::{BooleanDiff, StringDiff};
use super::schema::SchemaDiff;
use openapiv3::{Parameter, ParameterData, ParameterSchemaOrContent};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ParameterDiff {
    required: Option<BooleanDiff>,
    #[serde(rename = "in")]
    in_change: Option<StringDiff>,
    schema: Option<SchemaDiff>,
}

impl ParameterDiff {
    pub fn has_changes(&self) -> bool {
        self.required.is_some() || self.in_change.is_some() || self.schema.is_some()
    }

    pub fn from_params(base: &Parameter, head: &Parameter) -> Self {
        let base_parameter_data = Self::parameter_data(base);
        let head_parameter_data = Self::parameter_data(head);

        let schema_diff =
            if let ParameterSchemaOrContent::Schema(base_schema) = &base_parameter_data.format {
                if let ParameterSchemaOrContent::Schema(head_schema) = &head_parameter_data.format {
                    let diff = SchemaDiff::from_schemas(&base_schema, &head_schema);

                    if diff.has_changes() {
                        Some(diff)
                    } else {
                        None
                    }
                } else {
                    panic!("Parameter content is not supported yet")
                }
            } else {
                panic!("Parameter content is not supported yet")
            };

        Self {
            required: BooleanDiff::from_bools(
                base_parameter_data.required,
                head_parameter_data.required,
            ),
            in_change: StringDiff::from_strings(
                Self::parameter_type(base),
                Self::parameter_type(head),
            ),
            schema: schema_diff,
        }
    }

    pub fn parameter_data(param: &Parameter) -> &ParameterData {
        match param {
            Parameter::Query {
                parameter_data,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => parameter_data,
            Parameter::Header {
                parameter_data,
                style: _,
            } => parameter_data,
            Parameter::Path {
                parameter_data,
                style: _,
            } => parameter_data,
            Parameter::Cookie {
                parameter_data,
                style: _,
            } => parameter_data,
        }
    }

    pub fn parameter_type(param: &Parameter) -> String {
        match param {
            Parameter::Query {
                parameter_data: _,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => String::from("query"),
            Parameter::Header {
                parameter_data: _,
                style: _,
            } => String::from("header"),
            Parameter::Path {
                parameter_data: _,
                style: _,
            } => String::from("path"),
            Parameter::Cookie {
                parameter_data: _,
                style: _,
            } => String::from("cookie"),
        }
    }
}
