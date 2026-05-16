#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn cn_contact_formatter_string_from_contact_json(
        contact_json: *const c_char,
        style_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_contact_formatter_attributed_string_from_contact_json(
        contact_json: *const c_char,
        style_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_contact_formatter_name_order_from_contact_json(
        contact_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn cn_contact_formatter_delimiter_for_contact_json(
        contact_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_postal_address_formatter_string_from_postal_address_json(
        address_json: *const c_char,
        style_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_postal_address_formatter_attributed_string_from_postal_address_json(
        address_json: *const c_char,
        style_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
