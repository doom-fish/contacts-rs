#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cargo_common_metadata,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_if_let_else,
    clippy::return_self_not_must_use,
    clippy::struct_excessive_bools
)]

pub mod change_notifications;
pub mod contact;
pub mod contact_relation;
pub mod container;
pub mod error;
pub mod fetch_request;
mod ffi;
pub mod format_and_print;
pub mod group;
pub mod mutable_contact;
pub mod predicates;
mod private;
pub mod properties;
pub mod store;
pub mod types;
pub mod vcard_serialization;

pub use change_notifications::{
    contact_store_did_change_notification_name, CNChangeHistoryEvent, CNChangeHistoryFetchRequest,
    CNFetchResult,
};
pub use contact::{CNContact, CNContactKey, CNContactSortOrder, CNContactType};
pub use contact_relation::CNContactRelation;
pub use container::{CNContainer, CNContainerType};
pub use error::{CNAuthorizationStatus, ContactsError, NSErrorInfo};
pub use fetch_request::{CNAdditionalKeyDescriptor, CNContactFetchRequest};
pub use format_and_print::{
    CNAttributedString, CNAttributedStringRun, CNContactDisplayNameOrder, CNContactFormatter,
    CNContactFormatterStyle, CNPostalAddressFormatter, CNPostalAddressFormatterStyle,
};
pub use group::{CNGroup, CNMutableGroup};
pub use mutable_contact::CNMutableContact;
pub use predicates::{CNContactPredicate, CNContainerPredicate, CNGroupPredicate};
pub use properties::{
    CNContactProperty, CNDateComponents, CNInstantMessageAddress, CNInstantMessageAddressKey,
    CNLabeledValue, CNPhoneNumber, CNPostalAddress, CNPostalAddressKey, CNSocialProfile,
    CNSocialProfileKey,
};
pub use store::{CNContactStore, CNSaveOperation, CNSaveRequest};
pub use vcard_serialization::CNContactVCardSerialization;

/// Common imports.
pub mod prelude {
    pub use crate::change_notifications::{
        contact_store_did_change_notification_name, CNChangeHistoryEvent,
        CNChangeHistoryFetchRequest, CNFetchResult,
    };
    pub use crate::contact::{CNContact, CNContactKey, CNContactSortOrder, CNContactType};
    pub use crate::contact_relation::CNContactRelation;
    pub use crate::container::{CNContainer, CNContainerType};
    pub use crate::error::{CNAuthorizationStatus, ContactsError, NSErrorInfo};
    pub use crate::fetch_request::{CNAdditionalKeyDescriptor, CNContactFetchRequest};
    pub use crate::format_and_print::{
        CNAttributedString, CNAttributedStringRun, CNContactDisplayNameOrder, CNContactFormatter,
        CNContactFormatterStyle, CNPostalAddressFormatter, CNPostalAddressFormatterStyle,
    };
    pub use crate::group::{CNGroup, CNMutableGroup};
    pub use crate::mutable_contact::CNMutableContact;
    pub use crate::predicates::{CNContactPredicate, CNContainerPredicate, CNGroupPredicate};
    pub use crate::properties::{
        CNContactProperty, CNDateComponents, CNInstantMessageAddress, CNInstantMessageAddressKey,
        CNLabeledValue, CNPhoneNumber, CNPostalAddress, CNPostalAddressKey, CNSocialProfile,
        CNSocialProfileKey,
    };
    pub use crate::store::{CNContactStore, CNSaveOperation, CNSaveRequest};
    pub use crate::vcard_serialization::CNContactVCardSerialization;
}
