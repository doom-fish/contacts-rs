use serde::{Deserialize, Serialize};

/// Safe value wrapper for `CNGroup`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNGroup {
    pub identifier: String,
    pub name: String,
}

impl CNGroup {
    pub fn new(identifier: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
            name: name.into(),
        }
    }
}

/// Safe mutable wrapper for `CNMutableGroup`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNMutableGroup {
    pub identifier: Option<String>,
    pub name: Option<String>,
}

impl CNMutableGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl From<CNGroup> for CNMutableGroup {
    fn from(value: CNGroup) -> Self {
        Self {
            identifier: Some(value.identifier),
            name: Some(value.name),
        }
    }
}
