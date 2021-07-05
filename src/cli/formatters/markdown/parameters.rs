use crate::diff::parameters::ParametersDiff;
use crate::openapi::{Parameter, ReferenceOr};

pub struct ParametersPrinter<'a> {
    pub parameters: &'a ParametersDiff,
}

impl<'a> ParametersPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        for param in &self.parameters.added {
            match param {
                ReferenceOr::Reference { reference } => {
                    result.push_str(
                        format!("  - Referenced parameter `{}` was added.\n", reference).as_str(),
                    );
                }
                ReferenceOr::Item(param) => match param {
                    Parameter::Query {
                        parameter_data,
                        allow_reserved: _,
                        style: _,
                        allow_empty_value: _,
                    } => {
                        result.push_str(
                            format!("  - Query parameter `{}` was added.\n", parameter_data.name)
                                .as_str(),
                        );
                    }
                    Parameter::Header {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!("  - Header `{}` was added.\n", parameter_data.name).as_str(),
                        );
                    }
                    Parameter::Path {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!("  - Path parameter `{}` was added.\n", parameter_data.name)
                                .as_str(),
                        );
                    }
                    Parameter::Cookie {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!("  - Cookie `{}` was added.\n", parameter_data.name).as_str(),
                        );
                    }
                },
            }
        }

        for param in &self.parameters.removed {
            match param {
                ReferenceOr::Reference { reference } => {
                    result.push_str(
                        format!("  - Referenced parameter `{}` was removed.\n", reference).as_str(),
                    );
                }
                ReferenceOr::Item(param) => match param {
                    Parameter::Query {
                        parameter_data,
                        allow_reserved: _,
                        style: _,
                        allow_empty_value: _,
                    } => {
                        result.push_str(
                            format!(
                                "  - Query parameter `{}` was removed.\n",
                                parameter_data.name
                            )
                            .as_str(),
                        );
                    }
                    Parameter::Header {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!("  - Header `{}` was removed.\n", parameter_data.name).as_str(),
                        );
                    }
                    Parameter::Path {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!(
                                "  - Path parameter `{}` was removed.\n",
                                parameter_data.name
                            )
                            .as_str(),
                        );
                    }
                    Parameter::Cookie {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!("  - Cookie `{}` was removed.\n", parameter_data.name).as_str(),
                        );
                    }
                },
            }
        }

        result
    }
}
