mod common;

use contacts::async_api::AsyncCNContactStore;
use contacts::contact::CNContactKey;
use contacts::fetch_request::CNContactFetchRequest;
use contacts::store::{CNContactStore, CNEntityType};

// ============================================================================
// RequestAccessFuture
// ============================================================================

/// Happy path: when access is already authorized `request_access` resolves to
/// `Ok(true)`.  On headless CI this is skipped gracefully.
#[test]
fn request_access_resolves_when_authorized() {
    if !CNContactStore::authorization_status().is_authorized() {
        eprintln!("SKIP: contacts not authorized on this machine");
        return;
    }

    let result =
        pollster::block_on(AsyncCNContactStore::request_access(CNEntityType::Contacts));
    match result {
        Ok(granted) => assert!(granted, "expected granted=true when already authorized"),
        Err(e) => panic!("unexpected error from request_access: {e}"),
    }
}

/// The future must be Send so it can be moved across executor threads.
#[test]
fn request_access_future_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<contacts::async_api::RequestAccessFuture>();
}

// ============================================================================
// EnumerateContactsFuture
// ============================================================================

/// Happy path: when authorized, `enumerate_contacts` collects results into a Vec.
#[test]
fn enumerate_contacts_resolves_when_authorized() {
    if !CNContactStore::authorization_status().is_authorized() {
        eprintln!("SKIP: contacts not authorized on this machine");
        return;
    }

    let store = CNContactStore::new().expect("CNContactStore::new failed");
    let request = CNContactFetchRequest::new([CNContactKey::GivenName, CNContactKey::FamilyName]);

    let result =
        pollster::block_on(AsyncCNContactStore::enumerate_contacts(&store, &request));
    assert!(result.is_ok(), "enumerate_contacts failed: {result:?}");
}

/// Limit variant: result length must not exceed the limit.
#[test]
fn enumerate_contacts_limited_respects_limit() {
    if !CNContactStore::authorization_status().is_authorized() {
        eprintln!("SKIP: contacts not authorized on this machine");
        return;
    }

    let store = CNContactStore::new().expect("CNContactStore::new failed");
    let request = CNContactFetchRequest::new([CNContactKey::GivenName]);

    let contacts = pollster::block_on(AsyncCNContactStore::enumerate_contacts_limited(
        &store, &request, 2,
    ))
    .expect("enumerate_contacts_limited failed");
    assert!(
        contacts.len() <= 2,
        "limit not respected: got {} contacts",
        contacts.len()
    );
}

/// The future must be Send.
#[test]
fn enumerate_contacts_future_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<contacts::async_api::EnumerateContactsFuture>();
}

/// Error path: an invalid JSON request (simulated by passing a bad request
/// that will fail serialization — here we just check the error variant) is
/// reported cleanly.  We can't easily produce a serialization error through
/// the public API, so we test the error path by running without authorization
/// on a fresh store (the Swift bridge returns an error when access is denied).
#[test]
fn enumerate_contacts_error_path_when_not_authorized() {
    if CNContactStore::authorization_status().is_authorized() {
        // If we're authorized we can't trigger the denial error path here.
        eprintln!("SKIP: authorized machine — denial error path not testable");
        return;
    }

    let store = CNContactStore::new().expect("CNContactStore::new failed");
    let request = CNContactFetchRequest::new([CNContactKey::GivenName]);

    let result =
        pollster::block_on(AsyncCNContactStore::enumerate_contacts(&store, &request));
    // Expect an error since the store is not authorized.
    assert!(
        result.is_err(),
        "expected error when contacts access is denied, got: {result:?}"
    );
}

/// Same denial-path check for `request_access`.
#[test]
fn request_access_when_not_authorized() {
    if CNContactStore::authorization_status().is_authorized() {
        eprintln!("SKIP: authorized machine");
        return;
    }

    // On a machine where the dialog would block (e.g. restricted/denied),
    // `requestAccess` returns Ok(false) rather than an error.
    // We cannot reliably test both branches without user interaction, so we
    // just ensure the future does not panic.
    let _result =
        pollster::block_on(AsyncCNContactStore::request_access(CNEntityType::Contacts));
    // Any result is acceptable as long as it doesn't panic.
}
