//! Async API for the Contacts framework.
//!
//! This module provides [`Future`]-based wrappers for `CNContactStore` APIs
//! that use completion handlers on the Swift/Objective-C side.  All futures
//! are **executor-agnostic** and work with any async runtime (Tokio,
//! async-std, smol, `pollster`, …).
//!
//! Enable this module with the `async` Cargo feature:
//!
//! ```toml
//! [dependencies]
//! contacts = { version = "0.3", features = ["async"] }
//! ```
//!
//! ## Available types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`AsyncCNContactStore`] | Async contact-store operations |
//! | [`RequestAccessFuture`] | Future for `requestAccess(for:completionHandler:)` |
//! | [`EnumerateContactsFuture`] | Future for `enumerateContacts(with:usingBlock:)` |
//!
//! ## Async API surface
//!
//! | Apple API | Rust Future | Notes |
//! |-----------|-------------|-------|
//! | `CNContactStore.requestAccess(for:completionHandler:)` | [`RequestAccessFuture`] | Completion handler → `Future<bool>` |
//! | `CNContactStore.enumerateContacts(with:usingBlock:)` | [`EnumerateContactsFuture`] | Block → `Future<Vec<CNContact>>` (collects all); see note below |
//!
//! ### `enumerateContacts` — Future vs Stream
//!
//! The current implementation collects **all** matching contacts into a
//! `Vec<CNContact>` before resolving.  This is suitable for most use-cases.
//!
//! If you need incremental processing (e.g. millions of contacts, or early
//! termination) a `Stream`-based Tier-2 wrapper is the right approach and
//! will be provided in a future release.
//!
//! ## APIs deferred to Tier 2 (Streams)
//!
//! `CNContactStore` change notifications (`NotificationCenter`) are a
//! multi-fire observer pattern — they belong in a `Stream` wrapper, not a
//! one-shot `Future`.  Use the synchronous
//! [`crate::store::CNContactStore::fetch_change_history`] API today and
//! watch for the Tier-2 release.
//!
//! ## Sync-only APIs (no async wrapper needed)
//!
//! - `CNContactStore.containers(matching:)` — synchronous; use
//!   [`crate::store::CNContactStore::containers`].
//! - `CNContactStore.fetchChangeHistory(_:)` — synchronous; use
//!   [`crate::store::CNContactStore::fetch_change_history`].
//!
//! ## Example
//!
//! ```rust,no_run
//! use contacts::async_api::AsyncCNContactStore;
//! use contacts::store::CNEntityType;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     pollster::block_on(async {
//!         let granted = AsyncCNContactStore::request_access(CNEntityType::Contacts).await?;
//!         println!("access granted: {granted}");
//!         Ok(())
//!     })
//! }
//! ```

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};
use std::{
    ffi::c_void,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    contact::CNContact,
    error::ContactsError,
    fetch_request::CNContactFetchRequest,
    private::{json_cstring, parse_json_ptr},
    store::{CNContactStore, CNEntityType},
};

// ============================================================================
// requestAccess(for:completionHandler:) — RequestAccessFuture
// ============================================================================

/// FFI callback fired by the Swift bridge when `requestAccess` completes.
extern "C" fn request_access_cb(granted: bool, error: *const i8, ctx: *mut c_void) {
    if error.is_null() {
        unsafe { AsyncCompletion::complete_ok(ctx, granted) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<bool>::complete_err(ctx, msg) };
    }
}

/// [`Future`] returned by [`AsyncCNContactStore::request_access`].
///
/// Resolves to `Ok(true)` when the user grants access, `Ok(false)` when
/// denied, or `Err(ContactsError)` if the request itself fails.
pub struct RequestAccessFuture {
    inner: AsyncCompletionFuture<bool>,
}

impl std::fmt::Debug for RequestAccessFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestAccessFuture").finish_non_exhaustive()
    }
}

impl Future for RequestAccessFuture {
    type Output = Result<bool, ContactsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(ContactsError::OperationFailed))
    }
}

// ============================================================================
// enumerateContacts(with:usingBlock:) — EnumerateContactsFuture
// ============================================================================

/// FFI callback fired by the Swift bridge when contact enumeration completes.
///
/// The `result_json` pointer is heap-allocated in Swift via `cnrCString`; it
/// is consumed (and freed) by `parse_json_ptr`.
extern "C" fn enumerate_contacts_cb(
    result_json: *mut core::ffi::c_char,
    error: *const i8,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Vec<CNContact>>::complete_err(ctx, msg) };
    } else if !result_json.is_null() {
        match unsafe { parse_json_ptr::<Vec<CNContact>>(result_json, "CNContact list") } {
            Ok(contacts) => unsafe { AsyncCompletion::complete_ok(ctx, contacts) },
            Err(e) => unsafe {
                AsyncCompletion::<Vec<CNContact>>::complete_err(ctx, e.to_string());
            },
        }
    } else {
        unsafe {
            AsyncCompletion::<Vec<CNContact>>::complete_err(
                ctx,
                "Unknown error: both result and error were null".to_string(),
            );
        }
    }
}

/// [`Future`] returned by [`AsyncCNContactStore::enumerate_contacts`] and
/// [`AsyncCNContactStore::enumerate_contacts_limited`].
///
/// Resolves to `Ok(Vec<CNContact>)` with all matching contacts, or
/// `Err(ContactsError)` on failure.
pub struct EnumerateContactsFuture {
    inner: AsyncCompletionFuture<Vec<CNContact>>,
}

impl std::fmt::Debug for EnumerateContactsFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumerateContactsFuture")
            .finish_non_exhaustive()
    }
}

impl Future for EnumerateContactsFuture {
    type Output = Result<Vec<CNContact>, ContactsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(ContactsError::OperationFailed))
    }
}

// ============================================================================
// AsyncCNContactStore — public entry point
// ============================================================================

/// Async operations on `CNContactStore`.
///
/// All methods are **executor-agnostic** — they work with any async runtime.
/// Use `pollster::block_on` to run them synchronously in non-async contexts.
#[derive(Debug, Clone, Copy)]
pub struct AsyncCNContactStore;

impl AsyncCNContactStore {
    /// Asynchronously request access to the user's contacts for the given
    /// entity type.
    ///
    /// Wraps `CNContactStore.requestAccess(for:completionHandler:)`.
    ///
    /// Returns `Ok(true)` if the user grants access, `Ok(false)` if they
    /// deny it.  Returns `Err` only when the framework itself fails (very
    /// rare after the first prompt).
    ///
    /// # Errors
    ///
    /// [`ContactsError::OperationFailed`] if the framework returns an error.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use contacts::async_api::AsyncCNContactStore;
    /// use contacts::store::CNEntityType;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let granted = pollster::block_on(
    ///     AsyncCNContactStore::request_access(CNEntityType::Contacts)
    /// )?;
    /// println!("granted: {granted}");
    /// # Ok(()) }
    /// ```
    pub fn request_access(entity_type: CNEntityType) -> RequestAccessFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::store::cn_request_access_async(
                entity_type.raw_value(),
                request_access_cb,
                ctx,
            );
        }
        RequestAccessFuture { inner: future }
    }

    /// Asynchronously enumerate all contacts matching the fetch request.
    ///
    /// Wraps `CNContactStore.enumerateContacts(with:usingBlock:)` — runs in a
    /// background Swift `Task` and collects all results before resolving.
    ///
    /// For streaming / early-termination use-cases, a Tier-2 `Stream` wrapper
    /// will be added in a future release.
    ///
    /// # Errors
    ///
    /// [`ContactsError::OperationFailed`] if enumeration fails or access is
    /// denied.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use contacts::async_api::AsyncCNContactStore;
    /// use contacts::fetch_request::CNContactFetchRequest;
    /// use contacts::contact::CNContactKey;
    /// use contacts::store::CNContactStore;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let store = CNContactStore::new()?;
    /// let request = CNContactFetchRequest::new([CNContactKey::GivenName, CNContactKey::FamilyName]);
    /// let contacts = pollster::block_on(
    ///     AsyncCNContactStore::enumerate_contacts(&store, &request)
    /// )?;
    /// println!("{} contacts", contacts.len());
    /// # Ok(()) }
    /// ```
    pub fn enumerate_contacts(
        store: &CNContactStore,
        request: &CNContactFetchRequest,
    ) -> EnumerateContactsFuture {
        Self::enumerate_contacts_limited(store, request, 0)
    }

    /// Asynchronously enumerate contacts with an upper bound on the number
    /// of results.
    ///
    /// Same as [`enumerate_contacts`](Self::enumerate_contacts) but stops
    /// after `limit` contacts (pass `0` for no limit).
    ///
    /// # Errors
    ///
    /// [`ContactsError::OperationFailed`] if enumeration fails or access is
    /// denied.
    pub fn enumerate_contacts_limited(
        store: &CNContactStore,
        request: &CNContactFetchRequest,
        limit: usize,
    ) -> EnumerateContactsFuture {
        let request_json = match json_cstring(request, "CNContactFetchRequest") {
            Ok(s) => s,
            Err(e) => {
                // Return a future that immediately resolves to the error.
                let (future, ctx) = AsyncCompletion::create();
                unsafe {
                    AsyncCompletion::<Vec<CNContact>>::complete_err(ctx, e.to_string());
                }
                return EnumerateContactsFuture { inner: future };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::store::cn_enumerate_contacts_async(
                store.as_ptr(),
                request_json.as_ptr(),
                limit,
                enumerate_contacts_cb,
                ctx,
            );
        }
        EnumerateContactsFuture { inner: future }
    }
}
