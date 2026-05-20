#![allow(clippy::unsafe_derive_deserialize)]
//! Property value types and localized-string helpers.

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
    /// The labeled-value identifier.
    #[serde(default)]
    pub identifier: Option<String>,
    /// The labeled-value label.
    #[serde(default)]
    pub label: Option<String>,
    /// The labeled-value payload.
    pub value: T,
}

impl<T> CNLabeledValue<T> {
    /// Creates a new labeled value.
    pub fn new(label: Option<String>, value: T) -> Self {
        Self {
            identifier: None,
            label,
            value,
        }
    }

    /// Sets the identifier.
    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    /// Sets the label.
    pub fn with_label(mut self, label: Option<String>) -> Self {
        self.label = label;
        self
    }

    /// Maps the inner value while preserving the label metadata.
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
    /// The string value.
    pub string_value: String,
}

impl CNPhoneNumber {
    /// Creates a new `CNPhoneNumber`.
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
    /// The street.
    pub street: String,
    /// The sub locality.
    pub sub_locality: String,
    /// The city.
    pub city: String,
    /// The sub administrative area.
    pub sub_administrative_area: String,
    /// The state.
    pub state: String,
    /// The postal code.
    pub postal_code: String,
    /// The country.
    pub country: String,
    /// The ISO country code.
    pub iso_country_code: String,
}

impl CNPostalAddress {
    /// Creates a new `CNPostalAddress`.
    pub fn new(street: impl Into<String>) -> Self {
        Self {
            street: street.into(),
            ..Self::default()
        }
    }
}

/// Mutable Rust representation of `CNMutablePostalAddress`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNMutablePostalAddress {
    /// The street.
    pub street: String,
    /// The sub locality.
    pub sub_locality: String,
    /// The city.
    pub city: String,
    /// The sub administrative area.
    pub sub_administrative_area: String,
    /// The state.
    pub state: String,
    /// The postal code.
    pub postal_code: String,
    /// The country.
    pub country: String,
    /// The ISO country code.
    pub iso_country_code: String,
}

impl CNMutablePostalAddress {
    /// Creates a new `CNMutablePostalAddress`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the street.
    pub fn with_street(mut self, value: impl Into<String>) -> Self {
        self.street = value.into();
        self
    }

    /// Sets the sub locality.
    pub fn with_sub_locality(mut self, value: impl Into<String>) -> Self {
        self.sub_locality = value.into();
        self
    }

    /// Sets the city.
    pub fn with_city(mut self, value: impl Into<String>) -> Self {
        self.city = value.into();
        self
    }

    /// Sets the sub administrative area.
    pub fn with_sub_administrative_area(mut self, value: impl Into<String>) -> Self {
        self.sub_administrative_area = value.into();
        self
    }

    /// Sets the state.
    pub fn with_state(mut self, value: impl Into<String>) -> Self {
        self.state = value.into();
        self
    }

    /// Sets the postal code.
    pub fn with_postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = value.into();
        self
    }

    /// Sets the country.
    pub fn with_country(mut self, value: impl Into<String>) -> Self {
        self.country = value.into();
        self
    }

    /// Sets the ISO country code.
    pub fn with_iso_country_code(mut self, value: impl Into<String>) -> Self {
        self.iso_country_code = value.into();
        self
    }
}

/// Corresponds to `CNInstantMessageAddress`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNInstantMessageAddress {
    /// The username.
    pub username: String,
    /// The service.
    pub service: String,
}

impl CNInstantMessageAddress {
    /// Creates a new `CNInstantMessageAddress`.
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
    /// The URL string.
    pub url_string: String,
    /// The username.
    pub username: String,
    /// The user identifier.
    pub user_identifier: String,
    /// The service.
    pub service: String,
}

impl CNSocialProfile {
    /// Creates a new `CNSocialProfile`.
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
    /// The era.
    pub era: Option<i32>,
    /// The year.
    pub year: Option<i32>,
    /// The month.
    pub month: Option<i32>,
    /// The day.
    pub day: Option<i32>,
    /// The hour.
    pub hour: Option<i32>,
    /// The minute.
    pub minute: Option<i32>,
    /// The second.
    pub second: Option<i32>,
    /// The is leap month.
    pub is_leap_month: Option<bool>,
    #[serde(default)]
    /// The calendar identifier.
    pub calendar_identifier: Option<String>,
}

/// Safe wrapper for `CNContactProperty` payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNContactProperty {
    /// The contact.
    pub contact: CNContact,
    /// The key.
    pub key: String,
    /// The value.
    pub value: Value,
    #[serde(default)]
    /// The identifier.
    pub identifier: Option<String>,
    #[serde(default)]
    /// The label.
    pub label: Option<String>,
}

/// Postal-address property keys handled by Contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNPostalAddressKey {
    /// The street key.
    Street,
    /// The sub locality key.
    SubLocality,
    /// The city key.
    City,
    /// The sub administrative area key.
    SubAdministrativeArea,
    /// The state key.
    State,
    /// The postal code key.
    PostalCode,
    /// The country key.
    Country,
    /// The iso country code key.
    IsoCountryCode,
}

/// Instant-message property keys handled by Contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNInstantMessageAddressKey {
    /// The username key.
    Username,
    /// The service key.
    Service,
}

/// Social-profile property keys handled by Contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNSocialProfileKey {
    /// The url string key.
    UrlString,
    /// The username key.
    Username,
    /// The user identifier key.
    UserIdentifier,
    /// The service key.
    Service,
}

impl<T> CNLabeledValue<T> {
    /// Returns the localized string for the given label.
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
    /// Returns the localized string for the given key.
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

impl From<CNPostalAddress> for CNMutablePostalAddress {
    fn from(value: CNPostalAddress) -> Self {
        Self {
            street: value.street,
            sub_locality: value.sub_locality,
            city: value.city,
            sub_administrative_area: value.sub_administrative_area,
            state: value.state,
            postal_code: value.postal_code,
            country: value.country,
            iso_country_code: value.iso_country_code,
        }
    }
}

impl From<CNMutablePostalAddress> for CNPostalAddress {
    fn from(value: CNMutablePostalAddress) -> Self {
        Self {
            street: value.street,
            sub_locality: value.sub_locality,
            city: value.city,
            sub_administrative_area: value.sub_administrative_area,
            state: value.state,
            postal_code: value.postal_code,
            country: value.country,
            iso_country_code: value.iso_country_code,
        }
    }
}

impl CNInstantMessageAddress {
    /// Returns the localized string for the given key.
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

    /// Returns the localized string for the given service.
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
    /// Returns the localized string for the given key.
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

    /// Returns the localized string for the given service.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labeled_value_builders_set_metadata() {
        let labeled_value = CNLabeledValue::new(Some("home".to_owned()), "taylor@example.com".to_owned())
            .with_identifier("email-1")
            .with_label(Some("work".to_owned()));

        assert_eq!(labeled_value.identifier.as_deref(), Some("email-1"));
        assert_eq!(labeled_value.label.as_deref(), Some("work"));
        assert_eq!(labeled_value.value, "taylor@example.com");
    }

    #[test]
    fn labeled_value_map_preserves_metadata() {
        let mapped = CNLabeledValue::new(Some("mobile".to_owned()), "+1-555-0100".to_owned())
            .with_identifier("phone-1")
            .map(CNPhoneNumber::new);

        assert_eq!(mapped.identifier.as_deref(), Some("phone-1"));
        assert_eq!(mapped.label.as_deref(), Some("mobile"));
        assert_eq!(mapped.value.string_value, "+1-555-0100");
    }

    #[test]
    fn phone_number_conversions_preserve_string_value() {
        assert_eq!(CNPhoneNumber::from("+1-555-0100").string_value, "+1-555-0100");
        assert_eq!(
            CNPhoneNumber::from(String::from("+1-555-0101")).string_value,
            "+1-555-0101"
        );
    }

    #[test]
    fn date_components_round_trip_through_json() {
        let components = CNDateComponents {
            year: Some(2026),
            month: Some(5),
            day: Some(20),
            calendar_identifier: Some("gregorian".to_owned()),
            ..CNDateComponents::default()
        };

        let encoded = serde_json::to_string(&components).unwrap();
        let decoded: CNDateComponents = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, components);
    }

    #[test]
    fn mutable_postal_address_round_trip_to_immutable_preserves_fields() {
        let mutable = CNMutablePostalAddress::new()
            .with_street("1 Infinite Loop")
            .with_city("Cupertino")
            .with_state("CA")
            .with_postal_code("95014")
            .with_country("USA")
            .with_iso_country_code("US");

        let immutable: CNPostalAddress = mutable.clone().into();
        let round_trip: CNMutablePostalAddress = immutable.into();

        assert_eq!(round_trip, mutable);
    }
}
