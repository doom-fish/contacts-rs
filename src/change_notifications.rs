//! Change-history requests, events, and notification helpers.

use serde::{Deserialize, Serialize};

use crate::contact::{CNContact, CNContactKey};
use crate::error::ContactsError;
use crate::fetch_request::{CNAdditionalKeyDescriptor, CNKeyDescriptor};
use crate::ffi;
use crate::group::CNGroup;
use crate::private::take_required_string;

/// Generic wrapper mirroring `CNFetchResult`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNFetchResult<T> {
    /// The fetched value.
    pub value: T,
    /// The current history token.
    #[serde(with = "crate::private::serde_base64::required")]
    pub current_history_token: Vec<u8>,
}

/// Safe wrapper for `CNChangeHistoryFetchRequest`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNChangeHistoryFetchRequest {
    #[serde(default, with = "crate::private::serde_base64::option")]
    /// The starting token.
    pub starting_token: Option<Vec<u8>>,
    #[serde(default)]
    /// The additional contact keys.
    pub additional_contact_keys: Vec<CNContactKey>,
    #[serde(default)]
    /// The additional key descriptors.
    pub additional_key_descriptors: Vec<CNAdditionalKeyDescriptor>,
    #[serde(default)]
    /// The raw key descriptors.
    pub raw_key_descriptors: Vec<String>,
    /// Whether to unify results.
    pub should_unify_results: bool,
    /// Whether to return mutable objects.
    pub mutable_objects: bool,
    /// Whether to include group changes.
    pub include_group_changes: bool,
    #[serde(default)]
    /// The excluded transaction authors.
    pub excluded_transaction_authors: Vec<String>,
}

impl CNChangeHistoryFetchRequest {
    /// Creates a new `CNChangeHistoryFetchRequest`.
    pub fn new() -> Self {
        Self {
            should_unify_results: true,
            ..Self::default()
        }
    }

    /// Sets the starting token.
    pub fn with_starting_token(mut self, token: Vec<u8>) -> Self {
        self.starting_token = Some(token);
        self
    }

    /// Sets the additional contact keys.
    pub fn with_additional_contact_keys(
        mut self,
        keys: impl IntoIterator<Item = CNContactKey>,
    ) -> Self {
        self.additional_contact_keys = keys.into_iter().collect();
        self
    }

    /// Sets the key descriptor.
    pub fn with_key_descriptor(mut self, descriptor: CNKeyDescriptor) -> Self {
        self.push_key_descriptor(descriptor);
        self
    }

    /// Sets the key descriptors.
    pub fn with_key_descriptors(
        mut self,
        descriptors: impl IntoIterator<Item = CNKeyDescriptor>,
    ) -> Self {
        for descriptor in descriptors {
            self.push_key_descriptor(descriptor);
        }
        self
    }

    /// Sets the additional descriptors.
    pub fn with_additional_descriptors(
        mut self,
        descriptors: impl IntoIterator<Item = CNAdditionalKeyDescriptor>,
    ) -> Self {
        self.additional_key_descriptors = descriptors.into_iter().collect();
        self
    }

    /// Sets the raw key descriptor.
    pub fn with_raw_key_descriptor(mut self, descriptor: impl Into<String>) -> Self {
        self.raw_key_descriptors.push(descriptor.into());
        self
    }

    /// Sets the raw key descriptors.
    pub fn with_raw_key_descriptors(
        mut self,
        descriptors: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.raw_key_descriptors = descriptors.into_iter().map(Into::into).collect();
        self
    }

    /// Sets whether to unify results.
    pub fn with_should_unify_results(mut self, should_unify_results: bool) -> Self {
        self.should_unify_results = should_unify_results;
        self
    }

    /// Sets whether to return mutable objects.
    pub fn with_mutable_objects(mut self, mutable_objects: bool) -> Self {
        self.mutable_objects = mutable_objects;
        self
    }

    /// Sets whether to include group changes.
    pub fn with_include_group_changes(mut self, include_group_changes: bool) -> Self {
        self.include_group_changes = include_group_changes;
        self
    }

    /// Sets the excluded transaction authors.
    pub fn with_excluded_transaction_authors(
        mut self,
        authors: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.excluded_transaction_authors = authors.into_iter().map(Into::into).collect();
        self
    }

    /// Returns the key descriptors represented by this request.
    pub fn key_descriptors(&self) -> Vec<CNKeyDescriptor> {
        self.additional_contact_keys
            .iter()
            .copied()
            .map(CNKeyDescriptor::from)
            .chain(
                self.additional_key_descriptors
                    .iter()
                    .cloned()
                    .map(CNKeyDescriptor::from),
            )
            .chain(
                self.raw_key_descriptors
                    .iter()
                    .cloned()
                    .map(CNKeyDescriptor::raw),
            )
            .collect()
    }

    fn push_key_descriptor(&mut self, descriptor: CNKeyDescriptor) {
        match descriptor {
            CNKeyDescriptor::ContactKey { key } => self.additional_contact_keys.push(key),
            CNKeyDescriptor::Additional { descriptor } => {
                self.additional_key_descriptors.push(descriptor);
            }
            CNKeyDescriptor::Raw { value } => self.raw_key_descriptors.push(value),
        }
    }
}

/// Visitor mirroring the `CNChangeHistoryEventVisitor` protocol.
pub trait CNChangeHistoryEventVisitor {
    /// Visits a drop-everything event.
    fn visit_drop_everything_event(&mut self);
    /// Visits an add-contact event.
    fn visit_add_contact_event(&mut self, contact: &CNContact, container_identifier: Option<&str>);
    /// Visits an update-contact event.
    fn visit_update_contact_event(&mut self, contact: &CNContact);
    /// Visits a delete-contact event.
    fn visit_delete_contact_event(&mut self, contact_identifier: &str);

    /// Visits an add-group event.
    fn visit_add_group_event(&mut self, _group: &CNGroup, _container_identifier: &str) {}
    /// Visits an update-group event.
    fn visit_update_group_event(&mut self, _group: &CNGroup) {}
    /// Visits a delete-group event.
    fn visit_delete_group_event(&mut self, _group_identifier: &str) {}
    /// Visits an add-member-to-group event.
    fn visit_add_member_to_group_event(&mut self, _member: &CNContact, _group: &CNGroup) {}
    /// Visits a remove-member-from-group event.
    fn visit_remove_member_from_group_event(&mut self, _member: &CNContact, _group: &CNGroup) {}
    /// Visits an add-subgroup-to-group event.
    fn visit_add_subgroup_to_group_event(&mut self, _subgroup: &CNGroup, _group: &CNGroup) {}
    /// Visits a remove-subgroup-from-group event.
    fn visit_remove_subgroup_from_group_event(&mut self, _subgroup: &CNGroup, _group: &CNGroup) {}
}

/// Flattened Rust representation of `CNChangeHistoryEvent` subclasses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNChangeHistoryEvent {
    /// The drop everything event.
    DropEverything,
    /// The add contact event.
    AddContact {
        /// The added contact.
        contact: CNContact,
        /// The container identifier for the added contact.
        container_identifier: Option<String>,
    },
    /// The update contact event.
    UpdateContact {
        /// The updated contact.
        contact: CNContact,
    },
    /// The delete contact event.
    DeleteContact {
        /// The deleted contact identifier.
        contact_identifier: String,
    },
    /// The add group event.
    AddGroup {
        /// The added group.
        group: CNGroup,
        /// The container identifier for the added group.
        container_identifier: String,
    },
    /// The update group event.
    UpdateGroup {
        /// The updated group.
        group: CNGroup,
    },
    /// The delete group event.
    DeleteGroup {
        /// The deleted group identifier.
        group_identifier: String,
    },
    /// The add member to group event.
    AddMemberToGroup {
        /// The member that was added.
        member: CNContact,
        /// The group that received the member.
        group: CNGroup,
    },
    /// The remove member from group event.
    RemoveMemberFromGroup {
        /// The member that was removed.
        member: CNContact,
        /// The group that lost the member.
        group: CNGroup,
    },
    /// The add subgroup to group event.
    AddSubgroupToGroup {
        /// The subgroup that was added.
        subgroup: CNGroup,
        /// The parent group that received the subgroup.
        group: CNGroup,
    },
    /// The remove subgroup from group event.
    RemoveSubgroupFromGroup {
        /// The subgroup that was removed.
        subgroup: CNGroup,
        /// The parent group that lost the subgroup.
        group: CNGroup,
    },
}

impl CNChangeHistoryEvent {
    /// Dispatches this event to the provided visitor.
    pub fn accept_visitor(&self, visitor: &mut impl CNChangeHistoryEventVisitor) {
        match self {
            Self::DropEverything => visitor.visit_drop_everything_event(),
            Self::AddContact {
                contact,
                container_identifier,
            } => visitor.visit_add_contact_event(contact, container_identifier.as_deref()),
            Self::UpdateContact { contact } => visitor.visit_update_contact_event(contact),
            Self::DeleteContact { contact_identifier } => {
                visitor.visit_delete_contact_event(contact_identifier);
            }
            Self::AddGroup {
                group,
                container_identifier,
            } => visitor.visit_add_group_event(group, container_identifier),
            Self::UpdateGroup { group } => visitor.visit_update_group_event(group),
            Self::DeleteGroup { group_identifier } => {
                visitor.visit_delete_group_event(group_identifier);
            }
            Self::AddMemberToGroup { member, group } => {
                visitor.visit_add_member_to_group_event(member, group);
            }
            Self::RemoveMemberFromGroup { member, group } => {
                visitor.visit_remove_member_from_group_event(member, group);
            }
            Self::AddSubgroupToGroup { subgroup, group } => {
                visitor.visit_add_subgroup_to_group_event(subgroup, group);
            }
            Self::RemoveSubgroupFromGroup { subgroup, group } => {
                visitor.visit_remove_subgroup_from_group_event(subgroup, group);
            }
        }
    }
}

/// Returns the contact-store change notification name.
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
