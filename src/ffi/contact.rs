#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn cn_contact_item_provider_readable_type_identifiers_json(
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_contact_item_provider_writable_type_identifiers_json(
        contact_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_contact_item_provider_data_from_contact_json(
        contact_json: *const c_char,
        type_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_contact_from_item_provider_data_base64(
        base64_data: *const c_char,
        type_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
