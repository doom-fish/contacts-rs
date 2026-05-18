//! Predicate builders for contacts, groups, and containers.

use serde::{Deserialize, Serialize};

use crate::properties::CNPhoneNumber;

/// Mirrors the factory predicates from `CNContact+Predicates.h`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNContactPredicate {
    /// Matches contacts by name.
    MatchingName {
        /// The name to match.
        name: String,
    },
    /// Matches contacts by email address.
    MatchingEmailAddress {
        /// The email address to match.
        email_address: String,
    },
    /// Matches contacts by phone number.
    MatchingPhoneNumber {
        /// The phone number to match.
        phone_number: CNPhoneNumber,
    },
    /// Matches contacts by identifier.
    WithIdentifiers {
        /// The contact identifiers to match.
        identifiers: Vec<String>,
    },
    /// Matches contacts in a group.
    InGroupWithIdentifier {
        /// The group identifier to match.
        group_identifier: String,
    },
    /// Matches contacts in a container.
    InContainerWithIdentifier {
        /// The container identifier to match.
        container_identifier: String,
    },
}

impl CNContactPredicate {
    /// Creates a predicate matching the name.
    pub fn matching_name(name: impl Into<String>) -> Self {
        Self::MatchingName { name: name.into() }
    }

    /// Creates a predicate matching the email address.
    pub fn matching_email_address(email_address: impl Into<String>) -> Self {
        Self::MatchingEmailAddress {
            email_address: email_address.into(),
        }
    }

    /// Creates a predicate matching the phone number.
    pub fn matching_phone_number(phone_number: impl Into<CNPhoneNumber>) -> Self {
        Self::MatchingPhoneNumber {
            phone_number: phone_number.into(),
        }
    }

    /// Sets the identifiers.
    pub fn with_identifiers(identifiers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::WithIdentifiers {
            identifiers: identifiers.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates a predicate for contacts in the group.
    pub fn in_group(group_identifier: impl Into<String>) -> Self {
        Self::InGroupWithIdentifier {
            group_identifier: group_identifier.into(),
        }
    }

    /// Creates a predicate for contacts in the container.
    pub fn in_container(container_identifier: impl Into<String>) -> Self {
        Self::InContainerWithIdentifier {
            container_identifier: container_identifier.into(),
        }
    }
}

/// Mirrors the factory predicates from `CNGroup+Predicates.h`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNGroupPredicate {
    /// Matches groups by identifier.
    WithIdentifiers {
        /// The group identifiers to match.
        identifiers: Vec<String>,
    },
    /// Matches subgroups within a group.
    SubgroupsInGroupWithIdentifier {
        /// The parent group identifier to match.
        parent_group_identifier: String,
    },
    /// Matches groups in a container.
    GroupsInContainerWithIdentifier {
        /// The container identifier to match.
        container_identifier: String,
    },
}

impl CNGroupPredicate {
    /// Sets the identifiers.
    pub fn with_identifiers(identifiers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::WithIdentifiers {
            identifiers: identifiers.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates a predicate for subgroups in the group.
    pub fn subgroups_in_group(parent_group_identifier: impl Into<String>) -> Self {
        Self::SubgroupsInGroupWithIdentifier {
            parent_group_identifier: parent_group_identifier.into(),
        }
    }

    /// Creates a predicate for groups in the container.
    pub fn groups_in_container(container_identifier: impl Into<String>) -> Self {
        Self::GroupsInContainerWithIdentifier {
            container_identifier: container_identifier.into(),
        }
    }
}

/// Mirrors the factory predicates from `CNContainer+Predicates.h`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNContainerPredicate {
    /// Matches containers by identifier.
    WithIdentifiers {
        /// The container identifiers to match.
        identifiers: Vec<String>,
    },
    /// Matches the container for a contact.
    ContainerOfContactWithIdentifier {
        /// The contact identifier to match.
        contact_identifier: String,
    },
    /// Matches the container for a group.
    ContainerOfGroupWithIdentifier {
        /// The group identifier to match.
        group_identifier: String,
    },
}

impl CNContainerPredicate {
    /// Sets the identifiers.
    pub fn with_identifiers(identifiers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::WithIdentifiers {
            identifiers: identifiers.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates a predicate for the container of the contact.
    pub fn container_of_contact(contact_identifier: impl Into<String>) -> Self {
        Self::ContainerOfContactWithIdentifier {
            contact_identifier: contact_identifier.into(),
        }
    }

    /// Creates a predicate for the container of the group.
    pub fn container_of_group(group_identifier: impl Into<String>) -> Self {
        Self::ContainerOfGroupWithIdentifier {
            group_identifier: group_identifier.into(),
        }
    }
}
