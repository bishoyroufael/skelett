use crate::parser::sections::types::FilesType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    // Project containing description of the project
    #[allow(dead_code)]
    pub structure: Option<ProjectStructure>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectStructure {
    // Project structure which defines the files and directories
    // that shall be created
    #[allow(dead_code)]
    pub directories: Option<Vec<String>>,
    // Can be either a vector of Strings(filenames), File object, or mixed
    #[allow(dead_code)]
    pub files: Option<FilesType>,
}
