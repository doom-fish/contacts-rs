use serde::{Deserialize, Serialize};

/// Safe value wrapper for `CNContactRelation`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNContactRelation {
    pub name: String,
}

impl CNContactRelation {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
