use serde::{Deserialize, Serialize};

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
}
