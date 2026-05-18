#![allow(clippy::unsafe_derive_deserialize)]
//! Contact snapshots, keys, and item-provider helpers.

use serde::{Deserialize, Serialize};

use crate::contact_relation::CNContactRelation;
use crate::error::ContactsError;
use crate::fetch_request::CNAdditionalKeyDescriptor;
use crate::ffi;
use crate::private::{
    cstring_from_str, decode_base64_string, encode_base64_bytes, json_cstring, parse_json_ptr,
    take_required_string,
};
use crate::properties::{
    CNDateComponents, CNInstantMessageAddress, CNLabeledValue, CNPhoneNumber, CNPostalAddress,
    CNSocialProfile,
};

/// Corresponds to `CNContactType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactType {
    /// The person type.
    Person,
    /// The organization type.
    Organization,
}

/// Corresponds to `CNContactSortOrder`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CNContactSortOrder {
    #[default]
    /// The none order.
    None,
    /// The user default order.
    UserDefault,
    /// The given name order.
    GivenName,
    /// The family name order.
    FamilyName,
}

impl CNContactSortOrder {
    pub(crate) const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::UserDefault,
            2 => Self::GivenName,
            3 => Self::FamilyName,
            _ => Self::None,
        }
    }
}

/// Contact keys available via `CNContact`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactKey {
    /// The identifier key.
    Identifier,
    /// The contact type key.
    ContactType,
    /// The naMe prefix key.
    NamePrefix,
    /// The given name key.
    GivenName,
    /// The middle name key.
    MiddleName,
    /// The family name key.
    FamilyName,
    /// The previous family name key.
    PreviousFamilyName,
    /// The naMe suffix key.
    NameSuffix,
    /// The nickname key.
    Nickname,
    /// The organization name key.
    OrganizationName,
    /// The department name key.
    DepartmentName,
    /// The job title key.
    JobTitle,
    /// The phonetic given name key.
    PhoneticGivenName,
    /// The phonetic middle name key.
    PhoneticMiddleName,
    /// The phonetic family name key.
    PhoneticFamilyName,
    /// The phonetic organization name key.
    PhoneticOrganizationName,
    /// The note key.
    Note,
    /// The image data key.
    ImageData,
    /// The thumbnail image data key.
    ThumbnailImageData,
    /// The image data available key.
    ImageDataAvailable,
    /// The phone numbers key.
    PhoneNumbers,
    /// The email addresses key.
    EmailAddresses,
    /// The postal addresses key.
    PostalAddresses,
    /// The dates key.
    Dates,
    /// The url addresses key.
    UrlAddresses,
    /// The contact relations key.
    ContactRelations,
    /// The social profiles key.
    SocialProfiles,
    /// The instant message addresses key.
    InstantMessageAddresses,
    /// The birthday key.
    Birthday,
    /// The non gregorian birthday key.
    NonGregorianBirthday,
}

impl CNContactKey {
    /// Returns all supported values.
    pub fn all_supported() -> Vec<Self> {
        vec![
            Self::Identifier,
            Self::ContactType,
            Self::NamePrefix,
            Self::GivenName,
            Self::MiddleName,
            Self::FamilyName,
            Self::PreviousFamilyName,
            Self::NameSuffix,
            Self::Nickname,
            Self::OrganizationName,
            Self::DepartmentName,
            Self::JobTitle,
            Self::PhoneticGivenName,
            Self::PhoneticMiddleName,
            Self::PhoneticFamilyName,
            Self::PhoneticOrganizationName,
            Self::Note,
            Self::ImageData,
            Self::ThumbnailImageData,
            Self::ImageDataAvailable,
            Self::PhoneNumbers,
            Self::EmailAddresses,
            Self::PostalAddresses,
            Self::Dates,
            Self::UrlAddresses,
            Self::ContactRelations,
            Self::SocialProfiles,
            Self::InstantMessageAddresses,
            Self::Birthday,
            Self::NonGregorianBirthday,
        ]
    }
}

/// Safe value snapshot for `CNContact`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNContact {
    /// The identifier.
    pub identifier: String,
    #[serde(default)]
    /// The fetched keys.
    pub fetched_keys: Vec<CNContactKey>,
    #[serde(default)]
    /// The contact type.
    pub contact_type: Option<CNContactType>,
    #[serde(default)]
    /// The name prefix.
    pub name_prefix: Option<String>,
    #[serde(default)]
    /// The given name.
    pub given_name: Option<String>,
    #[serde(default)]
    /// The middle name.
    pub middle_name: Option<String>,
    #[serde(default)]
    /// The family name.
    pub family_name: Option<String>,
    #[serde(default)]
    /// The previous family name.
    pub previous_family_name: Option<String>,
    #[serde(default)]
    /// The name suffix.
    pub name_suffix: Option<String>,
    #[serde(default)]
    /// The nickname.
    pub nickname: Option<String>,
    #[serde(default)]
    /// The organization name.
    pub organization_name: Option<String>,
    #[serde(default)]
    /// The department name.
    pub department_name: Option<String>,
    #[serde(default)]
    /// The job title.
    pub job_title: Option<String>,
    #[serde(default)]
    /// The phonetic given name.
    pub phonetic_given_name: Option<String>,
    #[serde(default)]
    /// The phonetic middle name.
    pub phonetic_middle_name: Option<String>,
    #[serde(default)]
    /// The phonetic family name.
    pub phonetic_family_name: Option<String>,
    #[serde(default)]
    /// The phonetic organization name.
    pub phonetic_organization_name: Option<String>,
    #[serde(default)]
    /// The note.
    pub note: Option<String>,
    #[serde(default, with = "crate::private::serde_base64::option")]
    /// The image data.
    pub image_data: Option<Vec<u8>>,
    #[serde(default, with = "crate::private::serde_base64::option")]
    /// The thumbnail image data.
    pub thumbnail_image_data: Option<Vec<u8>>,
    #[serde(default)]
    /// The image data available.
    pub image_data_available: Option<bool>,
    #[serde(default)]
    /// The phone numbers.
    pub phone_numbers: Vec<CNLabeledValue<CNPhoneNumber>>,
    #[serde(default)]
    /// The email addresses.
    pub email_addresses: Vec<CNLabeledValue<String>>,
    #[serde(default)]
    /// The postal addresses.
    pub postal_addresses: Vec<CNLabeledValue<CNPostalAddress>>,
    #[serde(default)]
    /// The dates.
    pub dates: Vec<CNLabeledValue<CNDateComponents>>,
    #[serde(default)]
    /// The URL addresses.
    pub url_addresses: Vec<CNLabeledValue<String>>,
    #[serde(default)]
    /// The contact relations.
    pub contact_relations: Vec<CNLabeledValue<CNContactRelation>>,
    #[serde(default)]
    /// The social profiles.
    pub social_profiles: Vec<CNLabeledValue<CNSocialProfile>>,
    #[serde(default)]
    /// The instant message addresses.
    pub instant_message_addresses: Vec<CNLabeledValue<CNInstantMessageAddress>>,
    #[serde(default)]
    /// The birthday.
    pub birthday: Option<CNDateComponents>,
    #[serde(default)]
    /// The non-Gregorian birthday.
    pub non_gregorian_birthday: Option<CNDateComponents>,
}

impl CNContact {
    /// Returns a display name.
    pub fn display_name(&self) -> String {
        let given = self.given_name.as_deref().unwrap_or_default();
        let family = self.family_name.as_deref().unwrap_or_default();
        let combined = format!("{given} {family}").trim().to_owned();
        if !combined.is_empty() {
            combined
        } else if let Some(organization_name) = &self.organization_name {
            organization_name.clone()
        } else if let Some(nickname) = &self.nickname {
            nickname.clone()
        } else {
            self.identifier.clone()
        }
    }

    /// Returns whether the key is available.
    pub fn is_key_available(&self, key: CNContactKey) -> bool {
        key == CNContactKey::Identifier || self.fetched_keys.contains(&key)
    }

    /// Returns whether all keys are available.
    pub fn are_keys_available(&self, keys: &[CNContactKey]) -> bool {
        keys.iter().copied().all(|key| self.is_key_available(key))
    }

    /// Returns the localized string for the given key.
    pub fn localized_string_for_key(key: CNContactKey) -> Result<String, ContactsError> {
        let key_json = json_cstring(&key, "CNContactKey")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::properties::cn_contact_localized_string_for_key(key_json.as_ptr(), &mut error)
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(error, "CNContact.localizedString(forKey:) failed")
            })
        } else {
            unsafe { take_required_string(value, "CNContact.localizedString(forKey:)") }
        }
    }

    /// Returns the comparator-keys descriptor.
    pub fn descriptor_for_all_comparator_keys() -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::ComparatorKeys
    }

    /// Returns readable item-provider type identifiers.
    pub fn readable_type_identifiers_for_item_provider() -> Result<Vec<String>, ContactsError> {
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::contact::cn_contact_item_provider_readable_type_identifiers_json(&mut error)
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNContact.readableTypeIdentifiersForItemProvider failed",
                )
            })
        } else {
            unsafe { parse_json_ptr(value, "CNContact.readableTypeIdentifiersForItemProvider") }
        }
    }

    /// Returns writable item-provider type identifiers.
    pub fn writable_type_identifiers_for_item_provider(
        &self,
    ) -> Result<Vec<String>, ContactsError> {
        let contact_json = json_cstring(self, "CNContact")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::contact::cn_contact_item_provider_writable_type_identifiers_json(
                contact_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNContact.writableTypeIdentifiersForItemProvider failed",
                )
            })
        } else {
            unsafe { parse_json_ptr(value, "CNContact.writableTypeIdentifiersForItemProvider") }
        }
    }

    /// Returns item-provider data for the contact.
    pub fn item_provider_data(&self, type_identifier: &str) -> Result<Vec<u8>, ContactsError> {
        let contact_json = json_cstring(self, "CNContact")?;
        let type_identifier = cstring_from_str(type_identifier, "NSItemProvider type identifier")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::contact::cn_contact_item_provider_data_from_contact_json(
                contact_json.as_ptr(),
                type_identifier.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNContact.loadData(withTypeIdentifier:) failed",
                )
            })
        } else {
            let base64 =
                unsafe { take_required_string(value, "CNContact.loadData(withTypeIdentifier:)") }?;
            decode_base64_string(&base64, "CNContact.loadData(withTypeIdentifier:)")
        }
    }

    /// Builds a contact from item-provider data.
    pub fn from_item_provider_data(
        data: &[u8],
        type_identifier: &str,
    ) -> Result<Self, ContactsError> {
        let base64_data =
            cstring_from_str(&encode_base64_bytes(data), "CNContact item-provider data")?;
        let type_identifier = cstring_from_str(type_identifier, "NSItemProvider type identifier")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::contact::cn_contact_from_item_provider_data_base64(
                base64_data.as_ptr(),
                type_identifier.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNContact.object(withItemProviderData:typeIdentifier:) failed",
                )
            })
        } else {
            unsafe {
                parse_json_ptr(
                    value,
                    "CNContact.object(withItemProviderData:typeIdentifier:)",
                )
            }
        }
    }
}
