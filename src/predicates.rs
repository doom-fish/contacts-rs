use serde::{Deserialize, Serialize};

use crate::properties::CNPhoneNumber;

/// Mirrors the factory predicates from `CNContact+Predicates.h`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNContactPredicate {
    MatchingName { name: String },
    MatchingEmailAddress { email_address: String },
    MatchingPhoneNumber { phone_number: CNPhoneNumber },
    WithIdentifiers { identifiers: Vec<String> },
    InGroupWithIdentifier { group_identifier: String },
    InContainerWithIdentifier { container_identifier: String },
}

impl CNContactPredicate {
    pub fn matching_name(name: impl Into<String>) -> Self {
        Self::MatchingName { name: name.into() }
    }

    pub fn matching_email_address(email_address: impl Into<String>) -> Self {
        Self::MatchingEmailAddress {
            email_address: email_address.into(),
        }
    }

    pub fn matching_phone_number(phone_number: impl Into<CNPhoneNumber>) -> Self {
        Self::MatchingPhoneNumber {
            phone_number: phone_number.into(),
        }
    }

    pub fn with_identifiers(identifiers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::WithIdentifiers {
            identifiers: identifiers.into_iter().map(Into::into).collect(),
        }
    }

    pub fn in_group(group_identifier: impl Into<String>) -> Self {
        Self::InGroupWithIdentifier {
            group_identifier: group_identifier.into(),
        }
    }

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
    WithIdentifiers { identifiers: Vec<String> },
    SubgroupsInGroupWithIdentifier { parent_group_identifier: String },
    GroupsInContainerWithIdentifier { container_identifier: String },
}

impl CNGroupPredicate {
    pub fn with_identifiers(identifiers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::WithIdentifiers {
            identifiers: identifiers.into_iter().map(Into::into).collect(),
        }
    }

    pub fn subgroups_in_group(parent_group_identifier: impl Into<String>) -> Self {
        Self::SubgroupsInGroupWithIdentifier {
            parent_group_identifier: parent_group_identifier.into(),
        }
    }

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
    WithIdentifiers { identifiers: Vec<String> },
    ContainerOfContactWithIdentifier { contact_identifier: String },
    ContainerOfGroupWithIdentifier { group_identifier: String },
}

impl CNContainerPredicate {
    pub fn with_identifiers(identifiers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::WithIdentifiers {
            identifiers: identifiers.into_iter().map(Into::into).collect(),
        }
    }

    pub fn container_of_contact(contact_identifier: impl Into<String>) -> Self {
        Self::ContainerOfContactWithIdentifier {
            contact_identifier: contact_identifier.into(),
        }
    }

    pub fn container_of_group(group_identifier: impl Into<String>) -> Self {
        Self::ContainerOfGroupWithIdentifier {
            group_identifier: group_identifier.into(),
        }
    }
}
