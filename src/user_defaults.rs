use core::ffi::c_void;
use std::ptr::NonNull;

use crate::contact::CNContactSortOrder;
use crate::error::ContactsError;
use crate::ffi;
use crate::private::take_required_string;

/// Safe wrapper for `CNContactsUserDefaults`.
#[derive(Debug)]
pub struct CNContactsUserDefaults {
    raw: NonNull<c_void>,
}

impl CNContactsUserDefaults {
    pub fn shared() -> Result<Self, ContactsError> {
        let raw = NonNull::new(unsafe { ffi::user_defaults::cn_contacts_user_defaults_shared() })
            .ok_or_else(|| {
            ContactsError::OperationFailed(
                "failed to acquire CNContactsUserDefaults.sharedDefaults()".to_owned(),
            )
        })?;
        Ok(Self { raw })
    }

    pub fn sort_order(&self) -> CNContactSortOrder {
        CNContactSortOrder::from_raw(unsafe {
            ffi::user_defaults::cn_contacts_user_defaults_sort_order(self.raw.as_ptr())
        })
    }

    pub fn country_code(&self) -> Result<String, ContactsError> {
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::user_defaults::cn_contacts_user_defaults_country_code(
                self.raw.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(error, "CNContactsUserDefaults.countryCode failed")
            })
        } else {
            unsafe { take_required_string(value, "CNContactsUserDefaults.countryCode") }
        }
    }
}

impl Drop for CNContactsUserDefaults {
    fn drop(&mut self) {
        unsafe { ffi::user_defaults::cn_contacts_user_defaults_release(self.raw.as_ptr()) };
    }
}
