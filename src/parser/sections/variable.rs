use derivative::Derivative;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub struct Variable {
    /// Variable with custom fields i.e prompt
    #[allow(dead_code)]
    pub prompt: Option<String>,
}
