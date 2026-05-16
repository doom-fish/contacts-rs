#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn cn_contact_localized_string_for_key(
        key_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_labeled_value_localized_string_for_label(label: *const c_char) -> *mut c_char;
    pub fn cn_postal_address_localized_string_for_key(
        key_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_instant_message_localized_string_for_key(
        key_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_instant_message_localized_string_for_service(service: *const c_char) -> *mut c_char;
    pub fn cn_social_profile_localized_string_for_key(
        key_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_social_profile_localized_string_for_service(service: *const c_char) -> *mut c_char;
}
