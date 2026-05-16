#![allow(clippy::unsafe_derive_deserialize)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::contact::CNContact;
use crate::error::ContactsError;
use crate::ffi;
use crate::private::{cstring_from_str, json_cstring, take_required_string};

/// Safe wrapper for `CNLabeledValue`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNLabeledValue<T> {
    #[serde(default)]
    pub identifier: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    pub value: T,
}

impl<T> CNLabeledValue<T> {
    pub fn new(label: Option<String>, value: T) -> Self {
        Self {
            identifier: None,
            label,
            value,
        }
    }

    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    pub fn with_label(mut self, label: Option<String>) -> Self {
        self.label = label;
        self
    }

    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> CNLabeledValue<U> {
        CNLabeledValue {
            identifier: self.identifier,
            label: self.label,
            value: map(self.value),
        }
    }
}

/// Safe wrapper for `CNPhoneNumber`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNPhoneNumber {
    pub string_value: String,
}

impl CNPhoneNumber {
    pub fn new(string_value: impl Into<String>) -> Self {
        Self {
            string_value: string_value.into(),
        }
    }
}

impl From<String> for CNPhoneNumber {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CNPhoneNumber {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// Corresponds to `CNPostalAddress`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNPostalAddress {
    pub street: String,
    pub sub_locality: String,
    pub city: String,
    pub sub_administrative_area: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub iso_country_code: String,
}

impl CNPostalAddress {
    pub fn new(street: impl Into<String>) -> Self {
        Self {
            street: street.into(),
            ..Self::default()
        }
    }
}

/// Corresponds to `CNInstantMessageAddress`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNInstantMessageAddress {
    pub username: String,
    pub service: String,
}

impl CNInstantMessageAddress {
    pub fn new(username: impl Into<String>, service: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            service: service.into(),
        }
    }
}

/// Corresponds to `CNSocialProfile`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNSocialProfile {
    pub url_string: String,
    pub username: String,
    pub user_identifier: String,
    pub service: String,
}

impl CNSocialProfile {
    pub fn new(
        url_string: impl Into<String>,
        username: impl Into<String>,
        user_identifier: impl Into<String>,
        service: impl Into<String>,
    ) -> Self {
        Self {
            url_string: url_string.into(),
            username: username.into(),
            user_identifier: user_identifier.into(),
            service: service.into(),
        }
    }
}

/// Codable representation of `DateComponents` used by Contacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNDateComponents {
    pub era: Option<i32>,
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub hour: Option<i32>,
    pub minute: Option<i32>,
    pub second: Option<i32>,
    pub is_leap_month: Option<bool>,
    #[serde(default)]
    pub calendar_identifier: Option<String>,
}

/// Safe wrapper for `CNContactProperty` payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNContactProperty {
    pub contact: CNContact,
    pub key: String,
    pub value: Value,
    #[serde(default)]
    pub identifier: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
}

/// Postal-address property keys handled by Contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNPostalAddressKey {
    Street,
    SubLocality,
    City,
    SubAdministrativeArea,
    State,
    PostalCode,
    Country,
    IsoCountryCode,
}

/// Instant-message property keys handled by Contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNInstantMessageAddressKey {
    Username,
    Service,
}

/// Social-profile property keys handled by Contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNSocialProfileKey {
    UrlString,
    Username,
    UserIdentifier,
    Service,
}

impl<T> CNLabeledValue<T> {
    pub fn localized_string_for_label(label: &str) -> Result<String, ContactsError> {
        let label = cstring_from_str(label, "CNLabeledValue label")?;
        unsafe {
            take_required_string(
                ffi::properties::cn_labeled_value_localized_string_for_label(label.as_ptr()),
                "CNLabeledValue.localizedString(forLabel:)",
            )
        }
    }
}

impl CNPostalAddress {
    pub fn localized_string_for_key(key: CNPostalAddressKey) -> Result<String, ContactsError> {
        let key_json = json_cstring(&key, "CNPostalAddressKey")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::properties::cn_postal_address_localized_string_for_key(
                key_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNPostalAddress.localizedString(forKey:) failed",
                )
            })
        } else {
            unsafe { take_required_string(value, "CNPostalAddress.localizedString(forKey:)") }
        }
    }
}

impl CNInstantMessageAddress {
    pub fn localized_string_for_key(
        key: CNInstantMessageAddressKey,
    ) -> Result<String, ContactsError> {
        let key_json = json_cstring(&key, "CNInstantMessageAddressKey")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::properties::cn_instant_message_localized_string_for_key(
                key_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNInstantMessageAddress.localizedString(forKey:) failed",
                )
            })
        } else {
            unsafe {
                take_required_string(value, "CNInstantMessageAddress.localizedString(forKey:)")
            }
        }
    }

    pub fn localized_string_for_service(service: &str) -> Result<String, ContactsError> {
        let service = cstring_from_str(service, "CNInstantMessageAddress service")?;
        unsafe {
            take_required_string(
                ffi::properties::cn_instant_message_localized_string_for_service(service.as_ptr()),
                "CNInstantMessageAddress.localizedString(forService:)",
            )
        }
    }
}

impl CNSocialProfile {
    pub fn localized_string_for_key(key: CNSocialProfileKey) -> Result<String, ContactsError> {
        let key_json = json_cstring(&key, "CNSocialProfileKey")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::properties::cn_social_profile_localized_string_for_key(
                key_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNSocialProfile.localizedString(forKey:) failed",
                )
            })
        } else {
            unsafe { take_required_string(value, "CNSocialProfile.localizedString(forKey:)") }
        }
    }

    pub fn localized_string_for_service(service: &str) -> Result<String, ContactsError> {
        let service = cstring_from_str(service, "CNSocialProfile service")?;
        unsafe {
            take_required_string(
                ffi::properties::cn_social_profile_localized_string_for_service(service.as_ptr()),
                "CNSocialProfile.localizedString(forService:)",
            )
        }
    }
}
