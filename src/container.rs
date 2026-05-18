//! Container snapshots and container-type helpers.

use serde::{Deserialize, Serialize};

/// Corresponds to `CNContainerType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContainerType {
    /// The unassigned type.
    Unassigned,
    /// The local type.
    Local,
    /// The exchange type.
    Exchange,
    /// The Card dav type.
    CardDav,
}

/// Safe value wrapper for `CNContainer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNContainer {
    /// The identifier.
    pub identifier: String,
    /// The name.
    pub name: String,
    /// The container type.
    pub container_type: CNContainerType,
}

impl CNContainer {
    /// Creates a new `CNContainer`.
    pub fn new(
        identifier: impl Into<String>,
        name: impl Into<String>,
        container_type: CNContainerType,
    ) -> Self {
        Self {
            identifier: identifier.into(),
            name: name.into(),
            container_type,
        }
    }
}
