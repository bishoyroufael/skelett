use crate::parser::sections::file::{File, FileContentLocator};
use crate::parser::sections::variable::Variable;
use derivative::Derivative;
use serde::Deserialize;

use toml::Value;

#[derive(Debug, Deserialize, Clone, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
#[serde(untagged)]
pub enum VariableType {
    /// Variable can be either a simple string assignment
    String(String),
    /// Variable can be a prompt to the user
    PromptVariable(Variable),
}

impl VariableType {
    pub fn get_as_string(&self) -> Result<String, String> {
        match self {
            VariableType::String(v) => Ok(v.to_string()),
            VariableType::PromptVariable(v) => {
                Ok(v.prompt.clone().expect("Prompt variable is invalid"))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FileType {
    /// File can be defined as a string with extension
    String(String),
    /// File can be defined as a custom object with advanced fields
    FileType(File),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FilesType {
    /// Vector containing consistent values ot type FileType
    VectorTyped(Vec<FileType>),
    /// User defined files in template in mixed ways
    VectorMixed(Value),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FileContentType {
    /// File content can be passed as a raw string
    Raw(String),
    /// File content can be a vector containing lines to be printed
    Multiline(Vec<String>),
    /// File content can be brought from a uri (local, or remote)
    URI(FileContentLocator),
}
