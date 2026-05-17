#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn cn_copy_contacts_constant(
        symbol_name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
