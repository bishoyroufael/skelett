use crate::parser::sections::types::FileContentType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileContentLocator {
    /// File content locator with custom fields i.e uri
    #[allow(dead_code)]
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct File {
    /// File object which stores custom fields describing the file
    /// i.e name, content, ...etc
    #[allow(dead_code)]
    pub name: String,
    ///  Can be:
    ///  0. A variable that a user did set in the file
    ///  1. Vector containing strings (lines to write in file)
    //// 2. Local path to actual file on disk or a remote url to get content from
    ///  3. String representing raw content of the file  
    #[allow(dead_code)]
    pub content: Option<FileContentType>,
}
