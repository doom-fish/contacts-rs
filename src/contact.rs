#![allow(clippy::unsafe_derive_deserialize)]

use serde::{Deserialize, Serialize};

use crate::contact_relation::CNContactRelation;
use crate::error::ContactsError;
use crate::fetch_request::CNAdditionalKeyDescriptor;
use crate::ffi;
use crate::private::{json_cstring, take_required_string};
use crate::properties::{
    CNDateComponents, CNInstantMessageAddress, CNLabeledValue, CNPhoneNumber, CNPostalAddress,
    CNSocialProfile,
};

/// Corresponds to `CNContactType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactType {
    Person,
    Organization,
}

/// Corresponds to `CNContactSortOrder`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CNContactSortOrder {
    #[default]
    None,
    UserDefault,
    GivenName,
    FamilyName,
}

/// Contact keys available via `CNContact`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactKey {
    Identifier,
    ContactType,
    NamePrefix,
    GivenName,
    MiddleName,
    FamilyName,
    PreviousFamilyName,
    NameSuffix,
    Nickname,
    OrganizationName,
    DepartmentName,
    JobTitle,
    PhoneticGivenName,
    PhoneticMiddleName,
    PhoneticFamilyName,
    PhoneticOrganizationName,
    Note,
    ImageData,
    ThumbnailImageData,
    ImageDataAvailable,
    PhoneNumbers,
    EmailAddresses,
    PostalAddresses,
    Dates,
    UrlAddresses,
    ContactRelations,
    SocialProfiles,
    InstantMessageAddresses,
    Birthday,
    NonGregorianBirthday,
}

impl CNContactKey {
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
    pub identifier: String,
    #[serde(default)]
    pub fetched_keys: Vec<CNContactKey>,
    #[serde(default)]
    pub contact_type: Option<CNContactType>,
    #[serde(default)]
    pub name_prefix: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub middle_name: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub previous_family_name: Option<String>,
    #[serde(default)]
    pub name_suffix: Option<String>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub organization_name: Option<String>,
    #[serde(default)]
    pub department_name: Option<String>,
    #[serde(default)]
    pub job_title: Option<String>,
    #[serde(default)]
    pub phonetic_given_name: Option<String>,
    #[serde(default)]
    pub phonetic_middle_name: Option<String>,
    #[serde(default)]
    pub phonetic_family_name: Option<String>,
    #[serde(default)]
    pub phonetic_organization_name: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default, with = "crate::private::serde_base64::option")]
    pub image_data: Option<Vec<u8>>,
    #[serde(default, with = "crate::private::serde_base64::option")]
    pub thumbnail_image_data: Option<Vec<u8>>,
    #[serde(default)]
    pub image_data_available: Option<bool>,
    #[serde(default)]
    pub phone_numbers: Vec<CNLabeledValue<CNPhoneNumber>>,
    #[serde(default)]
    pub email_addresses: Vec<CNLabeledValue<String>>,
    #[serde(default)]
    pub postal_addresses: Vec<CNLabeledValue<CNPostalAddress>>,
    #[serde(default)]
    pub dates: Vec<CNLabeledValue<CNDateComponents>>,
    #[serde(default)]
    pub url_addresses: Vec<CNLabeledValue<String>>,
    #[serde(default)]
    pub contact_relations: Vec<CNLabeledValue<CNContactRelation>>,
    #[serde(default)]
    pub social_profiles: Vec<CNLabeledValue<CNSocialProfile>>,
    #[serde(default)]
    pub instant_message_addresses: Vec<CNLabeledValue<CNInstantMessageAddress>>,
    #[serde(default)]
    pub birthday: Option<CNDateComponents>,
    #[serde(default)]
    pub non_gregorian_birthday: Option<CNDateComponents>,
}

impl CNContact {
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

    pub fn is_key_available(&self, key: CNContactKey) -> bool {
        key == CNContactKey::Identifier || self.fetched_keys.contains(&key)
    }

    pub fn are_keys_available(&self, keys: &[CNContactKey]) -> bool {
        keys.iter().copied().all(|key| self.is_key_available(key))
    }

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

    pub fn descriptor_for_all_comparator_keys() -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::ComparatorKeys
    }
}
