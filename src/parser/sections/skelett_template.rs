use crate::parser::sections::project::Project;
use crate::parser::sections::types::VariableType;
use indexmap::map::IndexMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SkelettTemplate {
    /// Struct holding the information of the .skelett file
    /// It shall be parsed and processed to generate the template project
    #[allow(dead_code)]
    pub project: Option<Project>,
    /// Variables are a key-value pair
    /// - keys are variables names
    /// - values can be either a String or a Variable object
    #[allow(dead_code)]
    pub variables: Option<IndexMap<String, VariableType>>,
}
