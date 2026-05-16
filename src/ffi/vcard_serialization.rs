#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn cn_contact_vcard_data_from_contacts_json(
        contacts_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_contact_vcard_contacts_from_base64(
        base64_data: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
