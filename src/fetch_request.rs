//! Fetch-request and key-descriptor types.

use serde::{Deserialize, Serialize};

use crate::change_notifications::CNChangeHistoryFetchRequest;
use crate::contact::{CNContactKey, CNContactSortOrder};
use crate::format_and_print::CNContactFormatterStyle;
use crate::predicates::CNContactPredicate;

/// Additional descriptor sources accepted by Contacts fetch APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNAdditionalKeyDescriptor {
    /// The comparator-keys descriptor.
    ComparatorKeys,
    /// The formatter-required-keys descriptor.
    FormatterRequiredKeys {
        /// The formatter style.
        style: CNContactFormatterStyle,
    },
    /// The formatter-name-order descriptor.
    FormatterNameOrder,
    /// The formatter-delimiter descriptor.
    FormatterDelimiter,
    /// The vCard-required-keys descriptor.
    VCardRequiredKeys,
}

/// Safe wrapper for the generic `CNKeyDescriptor` protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNKeyDescriptor {
    /// A contact-key descriptor.
    ContactKey {
        /// The contact key.
        key: CNContactKey,
    },
    /// An additional-key descriptor.
    Additional {
        /// The additional descriptor.
        descriptor: CNAdditionalKeyDescriptor,
    },
    /// A raw key descriptor.
    Raw {
        /// The raw descriptor value.
        value: String,
    },
}

impl CNKeyDescriptor {
    /// Creates a contact-key descriptor.
    pub const fn contact_key(key: CNContactKey) -> Self {
        Self::ContactKey { key }
    }

    /// Creates an additional-key descriptor.
    pub const fn additional(descriptor: CNAdditionalKeyDescriptor) -> Self {
        Self::Additional { descriptor }
    }

    /// Creates a raw key descriptor.
    pub fn raw(value: impl Into<String>) -> Self {
        Self::Raw {
            value: value.into(),
        }
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
    /// The keys to fetch.
    pub keys_to_fetch: Vec<CNContactKey>,
    #[serde(default)]
    /// The extra descriptors.
    pub extra_descriptors: Vec<CNAdditionalKeyDescriptor>,
    #[serde(default)]
    /// The raw key descriptors.
    pub raw_key_descriptors: Vec<String>,
    #[serde(default)]
    /// The predicate.
    pub predicate: Option<CNContactPredicate>,
    /// Whether to return mutable objects.
    pub mutable_objects: bool,
    /// Whether to unify linked contacts.
    pub unify_results: bool,
    /// The sort order.
    pub sort_order: CNContactSortOrder,
}

impl CNContactFetchRequest {
    /// Creates a new `CNContactFetchRequest`.
    pub fn new(keys_to_fetch: impl IntoIterator<Item = CNContactKey>) -> Self {
        Self {
            keys_to_fetch: keys_to_fetch.into_iter().collect(),
            extra_descriptors: Vec::new(),
            raw_key_descriptors: Vec::new(),
            predicate: None,
            mutable_objects: false,
            unify_results: true,
            sort_order: CNContactSortOrder::None,
        }
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

    /// Sets the descriptor.
    pub fn with_descriptor(mut self, descriptor: CNAdditionalKeyDescriptor) -> Self {
        self.extra_descriptors.push(descriptor);
        self
    }

    /// Sets the descriptors.
    pub fn with_descriptors(
        mut self,
        descriptors: impl IntoIterator<Item = CNAdditionalKeyDescriptor>,
    ) -> Self {
        self.extra_descriptors.extend(descriptors);
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

    /// Sets the predicate.
    pub fn with_predicate(mut self, predicate: CNContactPredicate) -> Self {
        self.predicate = Some(predicate);
        self
    }

    /// Sets whether to return mutable objects.
    pub fn with_mutable_objects(mut self, mutable_objects: bool) -> Self {
        self.mutable_objects = mutable_objects;
        self
    }

    /// Sets whether to unify linked contacts.
    pub fn with_unify_results(mut self, unify_results: bool) -> Self {
        self.unify_results = unify_results;
        self
    }

    /// Sets the sort order.
    pub fn with_sort_order(mut self, sort_order: CNContactSortOrder) -> Self {
        self.sort_order = sort_order;
        self
    }

    /// Returns the key descriptors represented by this request.
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
            CNKeyDescriptor::ContactKey { key } => self.keys_to_fetch.push(key),
            CNKeyDescriptor::Additional { descriptor } => self.extra_descriptors.push(descriptor),
            CNKeyDescriptor::Raw { value } => self.raw_key_descriptors.push(value),
        }
    }
}

/// Type-erased wrapper for `CNFetchRequest` subclasses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "request", rename_all = "camelCase")]
pub enum CNFetchRequest {
    /// A contact fetch request.
    Contact(CNContactFetchRequest),
    /// A change-history fetch request.
    ChangeHistory(CNChangeHistoryFetchRequest),
}

impl CNFetchRequest {
    /// Returns this request as a contact fetch request.
    pub fn as_contact(&self) -> Option<&CNContactFetchRequest> {
        match self {
            Self::Contact(request) => Some(request),
            Self::ChangeHistory(_) => None,
        }
    }

    /// Returns this request as a change-history fetch request.
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
