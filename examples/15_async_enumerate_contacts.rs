//! Async enumerate-contacts example.
//!
//! Demonstrates `AsyncCNContactStore::enumerate_contacts` — the async version
//! of `CNContactStore.enumerateContacts(with:usingBlock:)`.
//!
//! Fetches up to five contacts asynchronously using `pollster::block_on`.
//! On a headless CI machine where access has not been granted this example
//! exits cleanly without printing any contact data.
//!
//! Run with:
//!
//! ```text
//! cargo run --example 15_async_enumerate_contacts --features async
//! ```

use contacts::async_api::AsyncCNContactStore;
use contacts::contact::CNContactKey;
use contacts::fetch_request::CNContactFetchRequest;
use contacts::store::{CNContactStore, CNEntityType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = CNContactStore::authorization_status();
    println!("Current authorization status: {status:?}");

    if !status.is_authorized() {
        println!("Access not authorized — skipping async enumeration (headless-safe exit).");
        return Ok(());
    }

    let store = CNContactStore::new()?;
    let request = CNContactFetchRequest::new([
        CNContactKey::GivenName,
        CNContactKey::FamilyName,
        CNContactKey::EmailAddresses,
    ]);

    // Enumerate up to 5 contacts asynchronously.
    let contacts = pollster::block_on(AsyncCNContactStore::enumerate_contacts_limited(
        &store, &request, 5,
    ))?;

    println!("Async enumeration returned {} contact(s) (limit=5):", contacts.len());
    for contact in &contacts {
        println!(
            "  {} {}",
            contact.given_name.as_deref().unwrap_or(""),
            contact.family_name.as_deref().unwrap_or(""),
        );
    }

    // Smoke-test: also verify entity-type-gated request_access returns true.
    let still_granted = pollster::block_on(AsyncCNContactStore::request_access(
        CNEntityType::Contacts,
    ))?;
    assert!(still_granted, "expected requestAccess to remain granted");

    println!("✓ async enumerate_contacts OK");
    Ok(())
}
