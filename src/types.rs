//! Compatibility re-exports for the crate's public types.

pub use crate::change_notifications::{
    contact_store_did_change_notification_name, CNChangeHistoryEvent, CNChangeHistoryEventVisitor,
    CNChangeHistoryFetchRequest, CNFetchResult,
};
pub use crate::constants::{
    contact_property_not_fetched_exception_name, contacts_error_domain, CNContactRelationLabel,
    CNContainerKey, CNDateLabel, CNEmailAddressLabel, CNErrorUserInfoKey, CNGroupKey,
    CNInstantMessageService, CNLabeledValueLabel, CNPhoneNumberLabel, CNSocialProfileService,
    CNUrlAddressLabel,
};
pub use crate::contact::{CNContact, CNContactKey, CNContactSortOrder, CNContactType};
pub use crate::contact_relation::CNContactRelation;
pub use crate::container::{CNContainer, CNContainerType};
pub use crate::error::{CNAuthorizationStatus, CNErrorCode, ContactsError, NSErrorInfo};
pub use crate::fetch_request::{
    CNAdditionalKeyDescriptor, CNContactFetchRequest, CNFetchRequest, CNKeyDescriptor,
};
pub use crate::format_and_print::{
    CNAttributedString, CNAttributedStringRun, CNContactDisplayNameOrder, CNContactFormatter,
    CNContactFormatterStyle, CNPostalAddressFormatter, CNPostalAddressFormatterStyle,
};
pub use crate::group::{CNGroup, CNMutableGroup};
pub use crate::mutable_contact::CNMutableContact;
pub use crate::predicates::{CNContactPredicate, CNContainerPredicate, CNGroupPredicate};
pub use crate::properties::{
    CNContactProperty, CNDateComponents, CNInstantMessageAddress, CNInstantMessageAddressKey,
    CNLabeledValue, CNMutablePostalAddress, CNPhoneNumber, CNPostalAddress, CNPostalAddressKey,
    CNSocialProfile, CNSocialProfileKey,
};
pub use crate::store::{CNContactStore, CNEntityType, CNSaveOperation, CNSaveRequest};
pub use crate::user_defaults::CNContactsUserDefaults;
pub use crate::vcard_serialization::CNContactVCardSerialization;
