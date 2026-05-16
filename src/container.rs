use serde::{Deserialize, Serialize};

/// Corresponds to `CNContainerType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContainerType {
    Unassigned,
    Local,
    Exchange,
    CardDav,
}

/// Safe value wrapper for `CNContainer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNContainer {
    pub identifier: String,
    pub name: String,
    pub container_type: CNContainerType,
}

impl CNContainer {
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
