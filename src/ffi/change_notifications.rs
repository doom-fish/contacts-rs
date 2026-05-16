#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cn_contact_store_did_change_notification_name() -> *mut c_char;
    pub fn cn_store_fetch_change_history_json(
        store: *mut c_void,
        request_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
