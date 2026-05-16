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

pub mod error;
mod ffi;
mod private;
pub mod store;
pub mod types;

pub use error::{CNAuthorizationStatus, ContactsError, NSErrorInfo};
pub use store::CNContactStore;
pub use types::{
    CNContact, CNContactFetchRequest, CNContactKey, CNContactSortOrder, CNContactType, CNContainer,
    CNContainerType, CNDateComponents, CNGroup, CNLabeledValue, CNMutableContact, CNPostalAddress,
    CNSaveOperation, CNSaveRequest,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::{CNAuthorizationStatus, ContactsError, NSErrorInfo};
    pub use crate::store::CNContactStore;
    pub use crate::types::{
        CNContact, CNContactFetchRequest, CNContactKey, CNContactSortOrder, CNContactType,
        CNContainer, CNContainerType, CNDateComponents, CNGroup, CNLabeledValue, CNMutableContact,
        CNPostalAddress, CNSaveOperation, CNSaveRequest,
    };
}
