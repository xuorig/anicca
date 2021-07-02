use super::common::{BooleanDiff, StringDiff};
use openapiv3::{Parameter, ParameterData};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ParameterDiff {
    required: Option<BooleanDiff>,
    #[serde(rename = "in")]
    in_change: Option<StringDiff>,
}

impl ParameterDiff {
    pub fn has_changes(&self) -> bool {
        // TODO
        true
    }

    pub fn from_params(base: &Parameter, head: &Parameter) -> Self {
        let base_parameter_data = Self::parameter_data(base);
        let head_parameter_data = Self::parameter_data(head);

        Self {
            required: BooleanDiff::from_bools(
                base_parameter_data.required,
                head_parameter_data.required,
            ),
            in_change: StringDiff::from_strings(
                Self::parameter_type(base),
                Self::parameter_type(head),
            ),
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
