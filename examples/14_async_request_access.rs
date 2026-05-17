//! Async request-access example.
//!
//! Demonstrates `AsyncCNContactStore::request_access` — the async version of
//! `CNContactStore.requestAccess(for:completionHandler:)`.
//!
//! On a headless CI machine this example reports the current authorization
//! status and skips the actual access request (which would hang waiting for
//! a system permission dialog).
//!
//! Run with:
//!
//! ```text
//! cargo run --example 14_async_request_access --features async
//! ```

use contacts::async_api::AsyncCNContactStore;
use contacts::store::{CNContactStore, CNEntityType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = CNContactStore::authorization_status();
    println!("Current authorization status: {status:?}");

    if !status.is_authorized() {
        // On headless CI or machines where the dialog would block, skip the
        // actual async request and exit cleanly.
        println!("Access not yet authorized — skipping async requestAccess prompt (headless-safe exit).");
        return Ok(());
    }

    // Access is already authorized; call the async wrapper to verify it
    // returns `Ok(true)` without presenting a dialog.
    let granted = pollster::block_on(AsyncCNContactStore::request_access(CNEntityType::Contacts))?;
    println!("Async requestAccess returned granted={granted}");
    assert!(granted, "expected granted=true when already authorized");

    println!("✓ async request_access OK");
    Ok(())
}
