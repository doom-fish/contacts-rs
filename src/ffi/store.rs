#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn cn_authorization_status(entity_type: i32) -> i32;
    pub fn cn_request_access(entity_type: i32, out_error: *mut *mut c_char) -> bool;

    pub fn cn_store_new() -> *mut c_void;
    pub fn cn_store_release(store: *mut c_void);
    pub fn cn_store_default_container_identifier(store: *mut c_void) -> *mut c_char;
    pub fn cn_store_current_history_token(store: *mut c_void) -> *mut c_char;
    pub fn cn_store_groups_json(
        store: *mut c_void,
        predicate_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_store_containers_json(
        store: *mut c_void,
        predicate_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_store_fetch_contacts_json(
        store: *mut c_void,
        request_json: *const c_char,
        limit: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_store_fetch_contacts_result_json(
        store: *mut c_void,
        request_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_store_fetch_contact_by_identifier_json(
        store: *mut c_void,
        identifier: *const c_char,
        keys_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_store_unified_me_contact_json(
        store: *mut c_void,
        keys_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn cn_store_execute_save_request(
        store: *mut c_void,
        request_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;

    // Async thunks — linked only when the async feature is enabled.
    #[cfg(feature = "async")]
    pub fn cn_request_access_async(
        entity_type: i32,
        cb: extern "C" fn(bool, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
    #[cfg(feature = "async")]
    pub fn cn_enumerate_contacts_async(
        store: *mut c_void,
        request_json: *const c_char,
        limit: usize,
        cb: extern "C" fn(*mut c_char, *const c_char, *mut c_void),
        ctx: *mut c_void,
    );
}
