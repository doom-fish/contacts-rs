use serde::{Deserialize, Serialize};

use crate::contact::{CNContact, CNContactKey};
use crate::error::ContactsError;
use crate::fetch_request::CNAdditionalKeyDescriptor;
use crate::ffi;
use crate::group::CNGroup;
use crate::private::take_required_string;

/// Generic wrapper mirroring `CNFetchResult`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNFetchResult<T> {
    pub value: T,
    #[serde(with = "crate::private::serde_base64::required")]
    pub current_history_token: Vec<u8>,
}

/// Safe wrapper for `CNChangeHistoryFetchRequest`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNChangeHistoryFetchRequest {
    #[serde(default, with = "crate::private::serde_base64::option")]
    pub starting_token: Option<Vec<u8>>,
    #[serde(default)]
    pub additional_contact_keys: Vec<CNContactKey>,
    #[serde(default)]
    pub additional_key_descriptors: Vec<CNAdditionalKeyDescriptor>,
    pub should_unify_results: bool,
    pub mutable_objects: bool,
    pub include_group_changes: bool,
    #[serde(default)]
    pub excluded_transaction_authors: Vec<String>,
}

impl CNChangeHistoryFetchRequest {
    pub fn new() -> Self {
        Self {
            should_unify_results: true,
            ..Self::default()
        }
    }

    pub fn with_starting_token(mut self, token: Vec<u8>) -> Self {
        self.starting_token = Some(token);
        self
    }

    pub fn with_additional_contact_keys(
        mut self,
        keys: impl IntoIterator<Item = CNContactKey>,
    ) -> Self {
        self.additional_contact_keys = keys.into_iter().collect();
        self
    }

    pub fn with_additional_descriptors(
        mut self,
        descriptors: impl IntoIterator<Item = CNAdditionalKeyDescriptor>,
    ) -> Self {
        self.additional_key_descriptors = descriptors.into_iter().collect();
        self
    }

    pub fn with_should_unify_results(mut self, should_unify_results: bool) -> Self {
        self.should_unify_results = should_unify_results;
        self
    }

    pub fn with_mutable_objects(mut self, mutable_objects: bool) -> Self {
        self.mutable_objects = mutable_objects;
        self
    }

    pub fn with_include_group_changes(mut self, include_group_changes: bool) -> Self {
        self.include_group_changes = include_group_changes;
        self
    }

    pub fn with_excluded_transaction_authors(
        mut self,
        authors: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.excluded_transaction_authors = authors.into_iter().map(Into::into).collect();
        self
    }
}

/// Flattened Rust representation of `CNChangeHistoryEvent` subclasses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNChangeHistoryEvent {
    DropEverything,
    AddContact {
        contact: CNContact,
        container_identifier: Option<String>,
    },
    UpdateContact {
        contact: CNContact,
    },
    DeleteContact {
        contact_identifier: String,
    },
    AddGroup {
        group: CNGroup,
        container_identifier: String,
    },
    UpdateGroup {
        group: CNGroup,
    },
    DeleteGroup {
        group_identifier: String,
    },
    AddMemberToGroup {
        member: CNContact,
        group: CNGroup,
    },
    RemoveMemberFromGroup {
        member: CNContact,
        group: CNGroup,
    },
    AddSubgroupToGroup {
        subgroup: CNGroup,
        group: CNGroup,
    },
    RemoveSubgroupFromGroup {
        subgroup: CNGroup,
        group: CNGroup,
    },
}

pub fn contact_store_did_change_notification_name() -> Result<String, ContactsError> {
    let value =
        unsafe { ffi::change_notifications::cn_contact_store_did_change_notification_name() };
    if value.is_null() {
        Err(ContactsError::OperationFailed(
            "missing CNContactStoreDidChangeNotification value".to_owned(),
        ))
    } else {
        unsafe { take_required_string(value, "CNContactStoreDidChangeNotification") }
    }
}
