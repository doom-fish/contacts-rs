use core::ffi::c_void;
use std::ptr::{self, NonNull};

use crate::error::{CNAuthorizationStatus, ContactsError};
use crate::ffi;
use crate::private::{cstring_from_str, json_cstring, parse_json_ptr, take_string};
use crate::types::{
    CNContact, CNContactFetchRequest, CNContactKey, CNContainer, CNGroup, CNMutableContact,
    CNSaveRequest,
};

/// Safe wrapper around `CNContactStore`.
#[derive(Debug)]
pub struct CNContactStore {
    raw: NonNull<c_void>,
}

impl CNContactStore {
    pub fn new() -> Result<Self, ContactsError> {
        let raw = NonNull::new(unsafe { ffi::cn_store_new() }).ok_or_else(|| {
            ContactsError::OperationFailed("failed to create CNContactStore".to_owned())
        })?;
        Ok(Self { raw })
    }

    pub fn authorization_status() -> CNAuthorizationStatus {
        CNAuthorizationStatus::from_raw(unsafe { ffi::cn_authorization_status() })
    }

    pub fn request_access() -> Result<bool, ContactsError> {
        let mut error = ptr::null_mut();
        let granted = unsafe { ffi::cn_request_access(&mut error) };
        if error.is_null() {
            Ok(granted)
        } else {
            Err(unsafe { ContactsError::from_error_ptr(error, "requestAccess failed") })
        }
    }

    pub fn default_container_identifier(&self) -> Option<String> {
        unsafe {
            take_string(ffi::cn_store_default_container_identifier(
                self.raw.as_ptr(),
            ))
        }
    }

    pub fn groups(&self) -> Result<Vec<CNGroup>, ContactsError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::cn_store_groups_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { ContactsError::from_error_ptr(error, "groupsMatchingPredicate failed") })
        } else {
            unsafe { parse_json_ptr(payload, "CNGroup list") }
        }
    }

    pub fn containers(&self) -> Result<Vec<CNContainer>, ContactsError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::cn_store_containers_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(error, "containersMatchingPredicate failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "CNContainer list") }
        }
    }

    pub fn enumerate_contacts(
        &self,
        request: &CNContactFetchRequest,
    ) -> Result<Vec<CNContact>, ContactsError> {
        self.enumerate_contacts_limited(request, 0)
    }

    pub fn enumerate_contacts_limited(
        &self,
        request: &CNContactFetchRequest,
        limit: usize,
    ) -> Result<Vec<CNContact>, ContactsError> {
        let request_json = json_cstring(request, "CNContactFetchRequest")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::cn_store_fetch_contacts_json(
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

    pub fn fetch_contacts(
        &self,
        request: &CNContactFetchRequest,
    ) -> Result<Vec<CNContact>, ContactsError> {
        self.enumerate_contacts(request)
    }

    pub fn fetch_mutable_contacts(
        &self,
        request: &CNContactFetchRequest,
    ) -> Result<Vec<CNMutableContact>, ContactsError> {
        let request = request.clone().with_mutable_objects(true);
        let request_json = json_cstring(&request, "CNContactFetchRequest")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::cn_store_fetch_contacts_json(
                self.raw.as_ptr(),
                request_json.as_ptr(),
                0,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { ContactsError::from_error_ptr(error, "mutable contact fetch failed") })
        } else {
            unsafe { parse_json_ptr(payload, "CNMutableContact list") }
        }
    }

    pub fn unified_contact(
        &self,
        identifier: &str,
        keys_to_fetch: &[CNContactKey],
    ) -> Result<Option<CNContact>, ContactsError> {
        let identifier = cstring_from_str(identifier, "contact identifier")?;
        let keys_json = json_cstring(keys_to_fetch, "CNContactKey list")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::cn_store_fetch_contact_by_identifier_json(
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

    pub fn unified_mutable_contact(
        &self,
        identifier: &str,
        keys_to_fetch: &[CNContactKey],
    ) -> Result<Option<CNMutableContact>, ContactsError> {
        Ok(self
            .unified_contact(identifier, keys_to_fetch)?
            .map(CNMutableContact::from))
    }

    pub fn execute_save_request(&self, request: &CNSaveRequest) -> Result<(), ContactsError> {
        let request_json = json_cstring(request, "CNSaveRequest")?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::cn_store_execute_save_request(self.raw.as_ptr(), request_json.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { ContactsError::from_error_ptr(error, "executeSaveRequest failed") })
        }
    }

    pub fn save_contact(
        &self,
        contact: CNMutableContact,
        container_identifier: Option<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.add_contact(contact, container_identifier);
        self.execute_save_request(&request)
    }

    pub fn update_contact(&self, contact: CNMutableContact) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.update_contact(contact);
        self.execute_save_request(&request)
    }

    pub fn delete_contact(&self, contact: CNMutableContact) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.delete_contact(contact);
        self.execute_save_request(&request)
    }

    pub fn delete_contact_by_identifier(
        &self,
        identifier: impl Into<String>,
    ) -> Result<(), ContactsError> {
        let mut request = CNSaveRequest::new();
        request.delete_contact(CNMutableContact {
            identifier: Some(identifier.into()),
            ..CNMutableContact::default()
        });
        self.execute_save_request(&request)
    }
}

impl Drop for CNContactStore {
    fn drop(&mut self) {
        unsafe { ffi::cn_store_release(self.raw.as_ptr()) };
    }
}
