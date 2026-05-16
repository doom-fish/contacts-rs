#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cn_contacts_user_defaults_shared() -> *mut c_void;
    pub fn cn_contacts_user_defaults_release(defaults: *mut c_void);
    pub fn cn_contacts_user_defaults_sort_order(defaults: *mut c_void) -> i32;
    pub fn cn_contacts_user_defaults_country_code(
        defaults: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
