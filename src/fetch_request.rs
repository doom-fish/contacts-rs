use serde::{Deserialize, Serialize};

use crate::change_notifications::CNChangeHistoryFetchRequest;
use crate::contact::{CNContactKey, CNContactSortOrder};
use crate::format_and_print::CNContactFormatterStyle;
use crate::predicates::CNContactPredicate;

/// Additional descriptor sources accepted by Contacts fetch APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNAdditionalKeyDescriptor {
    ComparatorKeys,
    FormatterRequiredKeys { style: CNContactFormatterStyle },
    FormatterNameOrder,
    FormatterDelimiter,
    VCardRequiredKeys,
}

/// Safe wrapper for the generic `CNKeyDescriptor` protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNKeyDescriptor {
    ContactKey {
        key: CNContactKey,
    },
    Additional {
        descriptor: CNAdditionalKeyDescriptor,
    },
}

impl CNKeyDescriptor {
    pub const fn contact_key(key: CNContactKey) -> Self {
        Self::ContactKey { key }
    }

    pub const fn additional(descriptor: CNAdditionalKeyDescriptor) -> Self {
        Self::Additional { descriptor }
    }
}

impl From<CNContactKey> for CNKeyDescriptor {
    fn from(key: CNContactKey) -> Self {
        Self::contact_key(key)
    }
}

impl From<CNAdditionalKeyDescriptor> for CNKeyDescriptor {
    fn from(descriptor: CNAdditionalKeyDescriptor) -> Self {
        Self::additional(descriptor)
    }
}

/// Safe wrapper for `CNContactFetchRequest`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNContactFetchRequest {
    pub keys_to_fetch: Vec<CNContactKey>,
    #[serde(default)]
    pub extra_descriptors: Vec<CNAdditionalKeyDescriptor>,
    #[serde(default)]
    pub predicate: Option<CNContactPredicate>,
    pub mutable_objects: bool,
    pub unify_results: bool,
    pub sort_order: CNContactSortOrder,
}

impl CNContactFetchRequest {
    pub fn new(keys_to_fetch: impl IntoIterator<Item = CNContactKey>) -> Self {
        Self {
            keys_to_fetch: keys_to_fetch.into_iter().collect(),
            extra_descriptors: Vec::new(),
            predicate: None,
            mutable_objects: false,
            unify_results: true,
            sort_order: CNContactSortOrder::None,
        }
    }

    pub fn with_key_descriptor(mut self, descriptor: CNKeyDescriptor) -> Self {
        self.push_key_descriptor(descriptor);
        self
    }

    pub fn with_key_descriptors(
        mut self,
        descriptors: impl IntoIterator<Item = CNKeyDescriptor>,
    ) -> Self {
        for descriptor in descriptors {
            self.push_key_descriptor(descriptor);
        }
        self
    }

    pub fn with_descriptor(mut self, descriptor: CNAdditionalKeyDescriptor) -> Self {
        self.extra_descriptors.push(descriptor);
        self
    }

    pub fn with_descriptors(
        mut self,
        descriptors: impl IntoIterator<Item = CNAdditionalKeyDescriptor>,
    ) -> Self {
        self.extra_descriptors.extend(descriptors);
        self
    }

    pub fn with_predicate(mut self, predicate: CNContactPredicate) -> Self {
        self.predicate = Some(predicate);
        self
    }

    pub fn with_mutable_objects(mut self, mutable_objects: bool) -> Self {
        self.mutable_objects = mutable_objects;
        self
    }

    pub fn with_unify_results(mut self, unify_results: bool) -> Self {
        self.unify_results = unify_results;
        self
    }

    pub fn with_sort_order(mut self, sort_order: CNContactSortOrder) -> Self {
        self.sort_order = sort_order;
        self
    }

    pub fn key_descriptors(&self) -> Vec<CNKeyDescriptor> {
        self.keys_to_fetch
            .iter()
            .copied()
            .map(CNKeyDescriptor::from)
            .chain(
                self.extra_descriptors
                    .iter()
                    .cloned()
                    .map(CNKeyDescriptor::from),
            )
            .collect()
    }

    fn push_key_descriptor(&mut self, descriptor: CNKeyDescriptor) {
        match descriptor {
            CNKeyDescriptor::ContactKey { key } => self.keys_to_fetch.push(key),
            CNKeyDescriptor::Additional { descriptor } => self.extra_descriptors.push(descriptor),
        }
    }
}

/// Type-erased wrapper for `CNFetchRequest` subclasses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "request", rename_all = "camelCase")]
pub enum CNFetchRequest {
    Contact(CNContactFetchRequest),
    ChangeHistory(CNChangeHistoryFetchRequest),
}

impl CNFetchRequest {
    pub fn as_contact(&self) -> Option<&CNContactFetchRequest> {
        match self {
            Self::Contact(request) => Some(request),
            Self::ChangeHistory(_) => None,
        }
    }

    pub fn as_change_history(&self) -> Option<&CNChangeHistoryFetchRequest> {
        match self {
            Self::Contact(_) => None,
            Self::ChangeHistory(request) => Some(request),
        }
    }
}

impl From<CNContactFetchRequest> for CNFetchRequest {
    fn from(request: CNContactFetchRequest) -> Self {
        Self::Contact(request)
    }
}

impl From<CNChangeHistoryFetchRequest> for CNFetchRequest {
    fn from(request: CNChangeHistoryFetchRequest) -> Self {
        Self::ChangeHistory(request)
    }
}
