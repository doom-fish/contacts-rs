//! Contact-store access and save-request helpers.

use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::change_notifications::{
    CNChangeHistoryEvent, CNChangeHistoryFetchRequest, CNFetchResult,
};
use crate::contact::{CNContact, CNContactKey};
use crate::container::CNContainer;
use crate::error::{CNAuthorizationStatus, ContactsError};
use crate::fetch_request::CNContactFetchRequest;
use crate::ffi;
use crate::group::{CNGroup, CNMutableGroup};
use crate::mutable_contact::CNMutableContact;
use crate::predicates::{CNContainerPredicate, CNGroupPredicate};
use crate::private::{
    cstring_from_str, decode_base64_string, json_cstring, parse_json_ptr, take_string,
};

/// Corresponds to `CNEntityType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNEntityType {
    /// The contacts type.
    Contacts,
}

impl CNEntityType {
    /// Returns the raw framework value.
    pub const fn raw_value(self) -> i32 {
        match self {
            Self::Contacts => 0,
        }
    }
}

/// Safe wrapper around `CNContactStore`.
#[derive(Debug)]
pub struct CNContactStore {
    raw: NonNull<c_void>,
}

/// Save-request operations supported by `CNSaveRequest`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CNSaveOperation {
    /// The add-contact operation.
    AddContact {
        /// The contact to add.
        contact: CNMutableContact,
        /// The optional destination container identifier.
        container_identifier: Option<String>,
    },
    /// The update-contact operation.
    UpdateContact {
        /// The contact to update.
        contact: CNMutableContact,
    },
    /// The delete-contact operation.
    DeleteContact {
        /// The contact identifier to delete.
        identifier: Option<String>,
    },
    /// The add-group operation.
    AddGroup {
        /// The group to add.
        group: CNMutableGroup,
        /// The optional destination container identifier.
        container_identifier: Option<String>,
    },
    /// The update-group operation.
    UpdateGroup {
        /// The group to update.
        group: CNMutableGroup,
    },
    /// The delete-group operation.
    DeleteGroup {
        /// The group identifier to delete.
        identifier: Option<String>,
    },
    /// The add-subgroup operation.
    AddSubgroup {
        /// The subgroup identifier to add.
        subgroup_identifier: String,
        /// The parent group identifier.
        group_identifier: String,
    },
    /// The remove-subgroup operation.
    RemoveSubgroup {
        /// The subgroup identifier to remove.
        subgroup_identifier: String,
        /// The parent group identifier.
        group_identifier: String,
    },
    /// The add-member operation.
    AddMember {
        /// The contact identifier to add.
        contact_identifier: String,
        /// The group identifier that receives the member.
        group_identifier: String,
    },
    /// The remove-member operation.
    RemoveMember {
        /// The contact identifier to remove.
        contact_identifier: String,
        /// The group identifier that loses the member.
        group_identifier: String,
    },
}

/// Safe wrapper for `CNSaveRequest`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNSaveRequest {
    /// The save operations to execute.
    pub operations: Vec<CNSaveOperation>,
    /// The optional transaction author.
    pub transaction_author: Option<String>,
    /// Whether to refetch contacts after saving.
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
    /// Creates a new `CNSaveRequest`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the contact.
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

    /// Updates an existing contact.
    pub fn update_contact(&mut self, contact: CNMutableContact) -> &mut Self {
        self.operations
            .push(CNSaveOperation::UpdateContact { contact });
        self
    }

    /// Deletes a contact.
    pub fn delete_contact(&mut self, contact: CNMutableContact) -> &mut Self {
        self.operations.push(CNSaveOperation::DeleteContact {
            identifier: contact.identifier,
        });
        self
    }

    /// Deletes a contact by identifier.
    pub fn delete_contact_by_identifier(&mut self, identifier: impl Into<String>) -> &mut Self {
        self.operations.push(CNSaveOperation::DeleteContact {
            identifier: Some(identifier.into()),
        });
        self
    }

    /// Adds the group.
    pub fn add_group(
        &mut self,
        group: CNMutableGroup,
        container_identifier: Option<String>,
    ) -> &mut Self {
        self.operations.push(CNSaveOperation::AddGroup {
            group,
            container_identifier,
        });
        self
    }

    /// Updates an existing group.
    pub fn update_group(&mut self, group: CNMutableGroup) -> &mut Self {
        self.operations.push(CNSaveOperation::UpdateGroup { group });
        self
    }

    /// Deletes a group.
    pub fn delete_group(&mut self, group: CNMutableGroup) -> &mut Self {
        self.operations.push(CNSaveOperation::DeleteGroup {
            identifier: group.identifier,
        });
        self
    }

    /// Deletes a group by identifier.
    pub fn delete_group_by_identifier(&mut self, identifier: impl Into<String>) -> &mut Self {
        self.operations.push(CNSaveOperation::DeleteGroup {
            identifier: Some(identifier.into()),
        });
        self
    }

    /// Adds the subgroup.
    pub fn add_subgroup(
        &mut self,
        subgroup_identifier: impl Into<String>,
        group_identifier: impl Into<String>,
    ) -> &mut Self {
        self.operations.push(CNSaveOperation::AddSubgroup {
            subgroup_identifier: subgroup_identifier.into(),
            group_identifier: group_identifier.into(),
        });
        self
    }

    /// Removes the subgroup.
    pub fn remove_subgroup(
        &mut self,
        subgroup_identifier: impl Into<String>,
        group_identifier: impl Into<String>,
    ) -> &mut Self {
        self.operations.push(CNSaveOperation::RemoveSubgroup {
            subgroup_identifier: subgroup_identifier.into(),
            group_identifier: group_identifier.into(),
        });
        self
    }

    /// Adds the member.
    pub fn add_member(
        &mut self,
        contact_identifier: impl Into<String>,
        group_identifier: impl Into<String>,
    ) -> &mut Self {
        self.operations.push(CNSaveOperation::AddMember {
            contact_identifier: contact_identifier.into(),
            group_identifier: group_identifier.into(),
        });
        self
    }

    /// Removes the member.
    pub fn remove_member(
        &mut self,
        contact_identifier: impl Into<String>,
        group_identifier: impl Into<String>,
    ) -> &mut Self {
        self.operations.push(CNSaveOperation::RemoveMember {
            contact_identifier: contact_identifier.into(),
            group_identifier: group_identifier.into(),
        });
        self
    }

    /// Sets the transaction author.
    pub fn with_transaction_author(mut self, author: impl Into<String>) -> Self {
        self.transaction_author = Some(author.into());
        self
    }

    /// Sets the should refetch contacts.
    pub fn with_should_refetch_contacts(mut self, should_refetch_contacts: bool) -> Self {
        self.should_refetch_contacts = should_refetch_contacts;
        self
    }
}

impl CNContactStore {
    /// Creates a new `CNContactStore`.
    pub fn new() -> Result<Self, ContactsError> {
        let raw = NonNull::new(unsafe { ffi::store::cn_store_new() }).ok_or_else(|| {
            ContactsError::OperationFailed("failed to create CNContactStore".to_owned())
        })?;
        Ok(Self { raw })
    }

    /// Returns the raw opaque pointer to the underlying `CNContactStore`.
    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    /// Returns the authorization status for contacts.
    pub fn authorization_status() -> CNAuthorizationStatus {
        Self::authorization_status_for_entity_type(CNEntityType::Contacts)
    }

    /// Returns the authorization status for the given entity type.
    pub fn authorization_status_for_entity_type(
        entity_type: CNEntityType,
    ) -> CNAuthorizationStatus {
        CNAuthorizationStatus::from_raw(unsafe {
            ffi::store::cn_authorization_status(entity_type.raw_value())
        })
    }

    /// Requests access for the given entity type.
    pub fn request_access() -> Result<bool, ContactsError> {
        Self::request_access_for_entity_type(CNEntityType::Contacts)
    }

    /// Requests access for the given entity type.
    pub fn request_access_for_entity_type(
        entity_type: CNEntityType,
    ) -> Result<bool, ContactsError> {
        let mut error = ptr::null_mut();
        let granted = unsafe { ffi::store::cn_request_access(entity_type.raw_value(), &mut error) };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe { ContactsError::from_error_ptr(error, "requestAccess failed") })
        }
    }

    /// Returns the default container identifier.
    pub fn default_container_identifier(&self) -> Option<String> {
        unsafe {
            take_string(ffi::store::cn_store_default_container_identifier(
                self.raw.as_ptr(),
            ))
        }
    }

    /// Returns the current history token.
    pub fn current_history_token(&self) -> Result<Option<Vec<u8>>, ContactsError> {
        let token = unsafe {
            take_string(ffi::store::cn_store_current_history_token(
                self.raw.as_ptr(),
            ))
        };
        token
            .map(|token| decode_base64_string(&token, "CNContactStore.currentHistoryToken"))
            .transpose()
    }

    /// Returns all groups.
    pub fn groups(&self) -> Result<Vec<CNGroup>, ContactsError> {
        self.groups_matching(None)
    }

    /// Returns the groups matching the predicate.
    pub fn groups_matching(
        &self,
        predicate: Option<&CNGroupPredicate>,
    ) -> Result<Vec<CNGroup>, ContactsError> {
        let predicate_json = predicate
            .map(|predicate| json_cstring(predicate, "CNGroupPredicate"))
            .transpose()?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::store::cn_store_groups_json(
                self.raw.as_ptr(),
                predicate_json
                    .as_ref()
                    .map_or(ptr::null(), |json| json.as_ptr()),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { ContactsError::from_error_ptr(error, "groupsMatchingPredicate failed") })
        } else {
            unsafe { parse_json_ptr(payload, "CNGroup list") }
        }
    }

    /// Returns all containers.
    pub fn containers(&self) -> Result<Vec<CNContainer>, ContactsError> {
        self.containers_matching(None)
    }

    /// Returns the containers matching the predicate.
    pub fn containers_matching(
        &self,
        predicate: Option<&CNContainerPredicate>,
    ) -> Result<Vec<CNContainer>, ContactsError> {
        let predicate_json = predicate
            .map(|predicate| json_cstring(predicate, "CNContainerPredicate"))
            .transpose()?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::store::cn_store_containers_json(
                self.raw.as_ptr(),
                predicate_json
                    .as_ref()
                    .map_or(ptr::null(), |json| json.as_ptr()),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(error, "containersMatchingPredicate failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "CNContainer list") }
        }
    }

    /// Enumerates contacts matching the fetch request.
    pub fn enumerate_contacts(
        &self,
        request: &CNContactFetchRequest,
    ) -> Result<Vec<CNContact>, ContactsError> {
        self.enumerate_contacts_limited(request, 0)
    }

    /// Enumerates contacts matching the fetch request up to the limit.
    pub fn enumerate_contacts_limited(
        &self,
        request: &CNContactFetchRequest,
        limit: usize,
    ) -> Result<Vec<CNContact>, ContactsError> {
        let request_json = json_cstring(request, "CNContactFetchRequest")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::store::cn_store_fetch_contacts_json(
                self.raw.as_ptr(),
                request_json.as_ptr(),
                limit,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { ContactsError::from_error_ptr(error, "contact enumeration failed") })
        } else {
            unsafe { parse_json_ptr(payload, "CNContact list") }
        }
    }

    /// Fetches contacts together with the current history token.
    pub fn contacts_result(
        &self,
        request: &CNContactFetchRequest,
    ) -> Result<CNFetchResult<Vec<CNContact>>, ContactsError> {
        let request_json = json_cstring(request, "CNContactFetchRequest")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::store::cn_store_fetch_contacts_result_json(
                self.raw.as_ptr(),
                request_json.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { ContactsError::from_error_ptr(error, "contact fetch result failed") })
        } else {
            unsafe { parse_json_ptr(payload, "CNFetchResult<CNContact>") }
        }
    }

    /// Fetches contacts matching the fetch request.
    pub fn fetch_contacts(
        &self,
        request: &CNContactFetchRequest,
    ) -> Result<Vec<CNContact>, ContactsError> {
        self.enumerate_contacts(request)
    }

    /// Fetches mutable contacts matching the fetch request.
    pub fn fetch_mutable_contacts(
        &self,
        request: &CNContactFetchRequest,
    ) -> Result<Vec<CNMutableContact>, ContactsError> {
        Ok(self
            .enumerate_contacts(&request.clone().with_mutable_objects(true))?
            .into_iter()
            .map(CNMutableContact::from)
            .collect())
    }

    /// Fetches a unified contact by identifier.
    pub fn unified_contact(
        &self,
        identifier: &str,
        keys_to_fetch: &[CNContactKey],
    ) -> Result<Option<CNContact>, ContactsError> {
        let identifier = cstring_from_str(identifier, "contact identifier")?;
        let keys_json = json_cstring(keys_to_fetch, "CNContactKey list")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::store::cn_store_fetch_contact_by_identifier_json(
                self.raw.as_ptr(),
                identifier.as_ptr(),
                keys_json.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(error, "unifiedContact(withIdentifier:) failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "optional CNContact") }
        }
    }

    /// Fetches the unified Me contact.
    pub fn unified_me_contact(
        &self,
        keys_to_fetch: &[CNContactKey],
    ) -> Result<Option<CNContact>, ContactsError> {
        let keys_json = json_cstring(keys_to_fetch, "CNContactKey list")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::store::cn_store_unified_me_contact_json(
                self.raw.as_ptr(),
                keys_json.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(error, "unifiedMeContact(withKeysToFetch:) failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "optional CNContact") }
        }
    }

    /// Fetches a unified mutable contact by identifier.
    pub fn unified_mutable_contact(
        &self,
        identifier: &str,
        keys_to_fetch: &[CNContactKey],
    ) -> Result<Option<CNMutableContact>, ContactsError> {
        Ok(self
            .unified_contact(identifier, keys_to_fetch)?
            .map(CNMutableContact::from))
    }

    /// Fetches change-history events for the request.
    pub fn fetch_change_history(
        &self,
        request: &CNChangeHistoryFetchRequest,
    ) -> Result<CNFetchResult<Vec<CNChangeHistoryEvent>>, ContactsError> {
        let request_json = json_cstring(request, "CNChangeHistoryFetchRequest")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::change_notifications::cn_store_fetch_change_history_json(
                self.raw.as_ptr(),
                request_json.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { ContactsError::from_error_ptr(error, "change history fetch failed") })
        } else {
            unsafe { parse_json_ptr(payload, "CNFetchResult<CNChangeHistoryEvent>") }
        }
    }

    /// Executes the save request.
    pub fn execute_save_request(&self, request: &CNSaveRequest) -> Result<(), ContactsError> {
        let request_json = json_cstring(request, "CNSaveRequest")?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::store::cn_store_execute_save_request(
                self.raw.as_ptr(),
                request_json.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { ContactsError::from_error_ptr(error, "executeSaveRequest failed") })
        }
    }

    /// Saves a new contact.
    pub fn save_contact(
        &self,
        contact: CNMutableContact,
        container_identifier: Option<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.add_contact(contact, container_identifier);
        self.execute_save_request(&request)
    }

    /// Updates an existing contact.
    pub fn update_contact(&self, contact: CNMutableContact) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.update_contact(contact);
        self.execute_save_request(&request)
    }

    /// Deletes a contact.
    pub fn delete_contact(&self, contact: CNMutableContact) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.delete_contact(contact);
        self.execute_save_request(&request)
    }

    /// Deletes a contact by identifier.
    pub fn delete_contact_by_identifier(
        &self,
        identifier: impl Into<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.delete_contact_by_identifier(identifier);
        self.execute_save_request(&request)
    }

    /// Saves a new group.
    pub fn save_group(
        &self,
        group: CNMutableGroup,
        container_identifier: Option<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.add_group(group, container_identifier);
        self.execute_save_request(&request)
    }

    /// Updates an existing group.
    pub fn update_group(&self, group: CNMutableGroup) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.update_group(group);
        self.execute_save_request(&request)
    }

    /// Deletes a group.
    pub fn delete_group(&self, group: CNMutableGroup) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.delete_group(group);
        self.execute_save_request(&request)
    }

    /// Deletes a group by identifier.
    pub fn delete_group_by_identifier(
        &self,
        identifier: impl Into<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.delete_group_by_identifier(identifier);
        self.execute_save_request(&request)
    }

    /// Adds a contact to a group.
    pub fn add_member_to_group(
        &self,
        contact_identifier: impl Into<String>,
        group_identifier: impl Into<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.add_member(contact_identifier, group_identifier);
        self.execute_save_request(&request)
    }

    /// Removes a contact from a group.
    pub fn remove_member_from_group(
        &self,
        contact_identifier: impl Into<String>,
        group_identifier: impl Into<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.remove_member(contact_identifier, group_identifier);
        self.execute_save_request(&request)
    }

    /// Adds a subgroup to a group.
    pub fn add_subgroup_to_group(
        &self,
        subgroup_identifier: impl Into<String>,
        group_identifier: impl Into<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.add_subgroup(subgroup_identifier, group_identifier);
        self.execute_save_request(&request)
    }

    /// Removes a subgroup from a group.
    pub fn remove_subgroup_from_group(
        &self,
        subgroup_identifier: impl Into<String>,
        group_identifier: impl Into<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.remove_subgroup(subgroup_identifier, group_identifier);
        self.execute_save_request(&request)
    }
}

impl Drop for CNContactStore {
    fn drop(&mut self) {
        unsafe { ffi::store::cn_store_release(self.raw.as_ptr()) };
    }
}
