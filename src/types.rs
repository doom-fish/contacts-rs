use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactKey {
    Identifier,
    ContactType,
    GivenName,
    FamilyName,
    OrganizationName,
    EmailAddresses,
    PhoneNumbers,
    PostalAddresses,
    UrlAddresses,
    Birthday,
}

impl CNContactKey {
    pub fn all_supported() -> Vec<Self> {
        vec![
            Self::Identifier,
            Self::ContactType,
            Self::GivenName,
            Self::FamilyName,
            Self::OrganizationName,
            Self::EmailAddresses,
            Self::PhoneNumbers,
            Self::PostalAddresses,
            Self::UrlAddresses,
            Self::Birthday,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CNContactSortOrder {
    #[default]
    None,
    UserDefault,
    GivenName,
    FamilyName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactType {
    Person,
    Organization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContainerType {
    Unassigned,
    Local,
    Exchange,
    CardDav,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CNLabeledValue<T> {
    pub label: Option<String>,
    pub value: T,
}

impl<T> CNLabeledValue<T> {
    pub fn new(label: Option<String>, value: T) -> Self {
        Self { label, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
            sub_locality: String::new(),
            city: String::new(),
            sub_administrative_area: String::new(),
            state: String::new(),
            postal_code: String::new(),
            country: String::new(),
            iso_country_code: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CNDateComponents {
    pub era: Option<i32>,
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub hour: Option<i32>,
    pub minute: Option<i32>,
    pub second: Option<i32>,
    pub is_leap_month: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CNContact {
    pub identifier: String,
    pub contact_type: Option<CNContactType>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub organization_name: Option<String>,
    #[serde(default)]
    pub email_addresses: Vec<CNLabeledValue<String>>,
    #[serde(default)]
    pub phone_numbers: Vec<CNLabeledValue<String>>,
    #[serde(default)]
    pub postal_addresses: Vec<CNLabeledValue<CNPostalAddress>>,
    #[serde(default)]
    pub url_addresses: Vec<CNLabeledValue<String>>,
    pub birthday: Option<CNDateComponents>,
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
        } else {
            self.identifier.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CNMutableContact {
    pub identifier: Option<String>,
    pub contact_type: Option<CNContactType>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub organization_name: Option<String>,
    pub email_addresses: Option<Vec<CNLabeledValue<String>>>,
    pub phone_numbers: Option<Vec<CNLabeledValue<String>>>,
    pub postal_addresses: Option<Vec<CNLabeledValue<CNPostalAddress>>>,
    pub url_addresses: Option<Vec<CNLabeledValue<String>>>,
    pub birthday: Option<CNDateComponents>,
}

impl CNMutableContact {
    pub fn new() -> Self {
        Self::default()
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

    pub fn with_birthday(mut self, value: CNDateComponents) -> Self {
        self.birthday = Some(value);
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
            given_name: value.given_name,
            family_name: value.family_name,
            organization_name: value.organization_name,
            email_addresses: Some(value.email_addresses),
            phone_numbers: Some(value.phone_numbers),
            postal_addresses: Some(value.postal_addresses),
            url_addresses: Some(value.url_addresses),
            birthday: value.birthday,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CNContactFetchRequest {
    pub keys_to_fetch: Vec<CNContactKey>,
    pub mutable_objects: bool,
    pub unify_results: bool,
    pub sort_order: CNContactSortOrder,
}

impl CNContactFetchRequest {
    pub fn new(keys_to_fetch: impl IntoIterator<Item = CNContactKey>) -> Self {
        Self {
            keys_to_fetch: keys_to_fetch.into_iter().collect(),
            mutable_objects: false,
            unify_results: true,
            sort_order: CNContactSortOrder::None,
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CNGroup {
    pub identifier: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CNContainer {
    pub identifier: String,
    pub name: String,
    pub container_type: CNContainerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNSaveOperation {
    AddContact {
        contact: CNMutableContact,
        container_identifier: Option<String>,
    },
    UpdateContact {
        contact: CNMutableContact,
    },
    DeleteContact {
        contact: CNMutableContact,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CNSaveRequest {
    pub operations: Vec<CNSaveOperation>,
    pub transaction_author: Option<String>,
    pub should_refetch_contacts: bool,
}

impl Default for CNSaveRequest {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            transaction_author: None,
            should_refetch_contacts: true,
        }
    }
}

impl CNSaveRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_contact(
        &mut self,
        contact: CNMutableContact,
        container_identifier: Option<String>,
    ) -> &mut Self {
        self.operations.push(CNSaveOperation::AddContact {
            contact,
            container_identifier,
        });
        self
    }

    pub fn update_contact(&mut self, contact: CNMutableContact) -> &mut Self {
        self.operations
            .push(CNSaveOperation::UpdateContact { contact });
        self
    }

    pub fn delete_contact(&mut self, contact: CNMutableContact) -> &mut Self {
        self.operations
            .push(CNSaveOperation::DeleteContact { contact });
        self
    }

    pub fn with_transaction_author(mut self, author: impl Into<String>) -> Self {
        self.transaction_author = Some(author.into());
        self
    }

    pub fn with_should_refetch_contacts(mut self, should_refetch_contacts: bool) -> Self {
        self.should_refetch_contacts = should_refetch_contacts;
        self
    }
}
