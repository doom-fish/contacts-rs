use base64::{engine::general_purpose::STANDARD, Engine as _};
use core::ptr;

use crate::contact::CNContact;
use crate::error::ContactsError;
use crate::fetch_request::CNAdditionalKeyDescriptor;
use crate::ffi;
use crate::private::{
    cstring_from_str, decode_base64_string, json_cstring, parse_json_ptr, take_string,
};

/// Namespace wrapper around `CNContactVCardSerialization`.
#[derive(Debug, Clone, Copy, Default)]
pub struct CNContactVCardSerialization;

impl CNContactVCardSerialization {
    pub fn descriptor_for_required_keys() -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::VCardRequiredKeys
    }

    pub fn data_with_contacts(contacts: &[CNContact]) -> Result<Vec<u8>, ContactsError> {
        let contacts_json = json_cstring(contacts, "[CNContact]")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::vcard_serialization::cn_contact_vcard_data_from_contacts_json(
                contacts_json.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNContactVCardSerialization.data(with:) failed",
                )
            })
        } else {
            let base64 = unsafe { take_string(payload) }.unwrap_or_default();
            decode_base64_string(&base64, "vCard data")
        }
    }

    pub fn contacts_with_data(data: &[u8]) -> Result<Vec<CNContact>, ContactsError> {
        let payload = cstring_from_str(&STANDARD.encode(data), "vCard data")?;
        let mut error = ptr::null_mut();
        let json = unsafe {
            ffi::vcard_serialization::cn_contact_vcard_contacts_from_base64(
                payload.as_ptr(),
                &mut error,
            )
        };
        if json.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNContactVCardSerialization.contacts(with:) failed",
                )
            })
        } else {
            unsafe { parse_json_ptr(json, "[CNContact]") }
        }
    }
}
