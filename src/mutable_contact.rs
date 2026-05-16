use serde::{Deserialize, Serialize};

use crate::contact::{CNContact, CNContactType};
use crate::contact_relation::CNContactRelation;
use crate::properties::{
    CNDateComponents, CNInstantMessageAddress, CNLabeledValue, CNPhoneNumber, CNPostalAddress,
    CNSocialProfile,
};

/// Mutable Rust representation of `CNMutableContact`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNMutableContact {
    pub identifier: Option<String>,
    pub contact_type: Option<CNContactType>,
    pub name_prefix: Option<String>,
    pub given_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: Option<String>,
    pub previous_family_name: Option<String>,
    pub name_suffix: Option<String>,
    pub nickname: Option<String>,
    pub organization_name: Option<String>,
    pub department_name: Option<String>,
    pub job_title: Option<String>,
    pub phonetic_given_name: Option<String>,
    pub phonetic_middle_name: Option<String>,
    pub phonetic_family_name: Option<String>,
    pub phonetic_organization_name: Option<String>,
    pub note: Option<String>,
    #[serde(default, with = "crate::private::serde_base64::option")]
    pub image_data: Option<Vec<u8>>,
    #[serde(default)]
    pub clear_image_data: bool,
    pub phone_numbers: Option<Vec<CNLabeledValue<CNPhoneNumber>>>,
    pub email_addresses: Option<Vec<CNLabeledValue<String>>>,
    pub postal_addresses: Option<Vec<CNLabeledValue<CNPostalAddress>>>,
    pub dates: Option<Vec<CNLabeledValue<CNDateComponents>>>,
    pub url_addresses: Option<Vec<CNLabeledValue<String>>>,
    pub contact_relations: Option<Vec<CNLabeledValue<CNContactRelation>>>,
    pub social_profiles: Option<Vec<CNLabeledValue<CNSocialProfile>>>,
    pub instant_message_addresses: Option<Vec<CNLabeledValue<CNInstantMessageAddress>>>,
    pub birthday: Option<CNDateComponents>,
    #[serde(default)]
    pub clear_birthday: bool,
    pub non_gregorian_birthday: Option<CNDateComponents>,
    #[serde(default)]
    pub clear_non_gregorian_birthday: bool,
}

impl CNMutableContact {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn with_given_name(mut self, value: impl Into<String>) -> Self {
        self.given_name = Some(value.into());
        self
    }

    pub fn with_family_name(mut self, value: impl Into<String>) -> Self {
        self.family_name = Some(value.into());
        self
    }

    pub fn with_organization_name(mut self, value: impl Into<String>) -> Self {
        self.organization_name = Some(value.into());
        self
    }

    pub fn with_note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn with_phone_numbers(mut self, values: Vec<CNLabeledValue<CNPhoneNumber>>) -> Self {
        self.phone_numbers = Some(values);
        self
    }

    pub fn with_email_addresses(mut self, values: Vec<CNLabeledValue<String>>) -> Self {
        self.email_addresses = Some(values);
        self
    }

    pub fn with_postal_addresses(mut self, values: Vec<CNLabeledValue<CNPostalAddress>>) -> Self {
        self.postal_addresses = Some(values);
        self
    }

    pub fn with_contact_relations(
        mut self,
        values: Vec<CNLabeledValue<CNContactRelation>>,
    ) -> Self {
        self.contact_relations = Some(values);
        self
    }

    pub fn with_social_profiles(mut self, values: Vec<CNLabeledValue<CNSocialProfile>>) -> Self {
        self.social_profiles = Some(values);
        self
    }

    pub fn with_instant_message_addresses(
        mut self,
        values: Vec<CNLabeledValue<CNInstantMessageAddress>>,
    ) -> Self {
        self.instant_message_addresses = Some(values);
        self
    }

    pub fn with_birthday(mut self, value: CNDateComponents) -> Self {
        self.birthday = Some(value);
        self.clear_birthday = false;
        self
    }

    pub fn clear_birthday(mut self) -> Self {
        self.birthday = None;
        self.clear_birthday = true;
        self
    }

    pub fn with_non_gregorian_birthday(mut self, value: CNDateComponents) -> Self {
        self.non_gregorian_birthday = Some(value);
        self.clear_non_gregorian_birthday = false;
        self
    }

    pub fn clear_non_gregorian_birthday(mut self) -> Self {
        self.non_gregorian_birthday = None;
        self.clear_non_gregorian_birthday = true;
        self
    }

    pub fn with_image_data(mut self, value: Vec<u8>) -> Self {
        self.image_data = Some(value);
        self.clear_image_data = false;
        self
    }

    pub fn clear_image_data(mut self) -> Self {
        self.image_data = None;
        self.clear_image_data = true;
        self
    }

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
            self.identifier
                .clone()
                .unwrap_or_else(|| "<new contact>".to_owned())
        }
    }
}

impl From<CNContact> for CNMutableContact {
    fn from(value: CNContact) -> Self {
        Self {
            identifier: Some(value.identifier),
            contact_type: value.contact_type,
            name_prefix: value.name_prefix,
            given_name: value.given_name,
            middle_name: value.middle_name,
            family_name: value.family_name,
            previous_family_name: value.previous_family_name,
            name_suffix: value.name_suffix,
            nickname: value.nickname,
            organization_name: value.organization_name,
            department_name: value.department_name,
            job_title: value.job_title,
            phonetic_given_name: value.phonetic_given_name,
            phonetic_middle_name: value.phonetic_middle_name,
            phonetic_family_name: value.phonetic_family_name,
            phonetic_organization_name: value.phonetic_organization_name,
            note: value.note,
            image_data: value.image_data,
            clear_image_data: false,
            phone_numbers: Some(value.phone_numbers),
            email_addresses: Some(value.email_addresses),
            postal_addresses: Some(value.postal_addresses),
            dates: Some(value.dates),
            url_addresses: Some(value.url_addresses),
            contact_relations: Some(value.contact_relations),
            social_profiles: Some(value.social_profiles),
            instant_message_addresses: Some(value.instant_message_addresses),
            birthday: value.birthday,
            clear_birthday: false,
            non_gregorian_birthday: value.non_gregorian_birthday,
            clear_non_gregorian_birthday: false,
        }
    }
}
