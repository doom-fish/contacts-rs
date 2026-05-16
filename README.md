# contacts

Safe Rust bindings for Apple's [Contacts](https://developer.apple.com/documentation/contacts) framework on macOS.

> **Status:** v0.1.0 covers the practical address-book surface for `CNContactStore`, `CNContact` / `CNMutableContact`, `CNContactFetchRequest`, `CNGroup`, `CNContainer`, and `CNSaveRequest`.

## Quick start

```rust,no_run
use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = CNContactStore::authorization_status();
    println!("contacts authorization: {status:?}");

    if !status.is_authorized() {
        return Ok(());
    }

    let store = CNContactStore::new()?;
    let request = CNContactFetchRequest::new([
        CNContactKey::GivenName,
        CNContactKey::FamilyName,
        CNContactKey::OrganizationName,
        CNContactKey::EmailAddresses,
    ])
    .with_sort_order(CNContactSortOrder::GivenName);

    for contact in store.enumerate_contacts_limited(&request, 5)? {
        println!("{}", contact.display_name());
    }

    Ok(())
}
```

## Highlights

- `CNContactStore::authorization_status`, `request_access`, `default_container_identifier`
- Contact enumeration, limited fetches, and identifier-based fetches with `CNContactFetchRequest`
- `CNContact` snapshots covering names, organization, email addresses, phone numbers, postal addresses, URL addresses, and birthdays
- `CNMutableContact` plus `CNSaveRequest` helpers for add / update / delete flows
- `CNGroup` and `CNContainer` listing

## Authorization

`Contacts.framework` access is gated by the user's privacy settings. The included smoke example never requests permission; it reports the current status and only enumerates contacts when access is already granted.

## Smoke example

Run the framework smoke test with:

```bash
cargo run --all-features --example 01_contacts_store_smoke
```

Expected success footer:

```text
✅ contacts store OK
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
