//! Group snapshots and mutable group builders.

use serde::{Deserialize, Serialize};

/// Safe value wrapper for `CNGroup`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNGroup {
    /// The identifier.
    pub identifier: String,
    /// The name.
    pub name: String,
}

impl CNGroup {
    /// Creates a new `CNGroup`.
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
    /// The identifier.
    pub identifier: Option<String>,
    /// The name.
    pub name: Option<String>,
}

impl CNMutableGroup {
    /// Creates a new `CNMutableGroup`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the identifier.
    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    /// Sets the name.
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
