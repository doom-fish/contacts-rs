# contacts

Safe Rust bindings for Apple's [Contacts](https://developer.apple.com/documentation/contacts) framework on macOS.

> **Status:** v0.2.0 extends the Swift bridge and safe Rust API across Store, Contact, MutableContact, Group, Container, FetchRequest, FormatAndPrint, ChangeNotifications, Properties, Predicates, VCardSerialization, and ContactRelation.

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
    .with_sort_order(CNContactSortOrder::GivenName)
    .with_descriptor(CNContactFormatter::descriptor_for_required_keys_for_style(
        CNContactFormatterStyle::FullName,
    ));

    for contact in store.enumerate_contacts_limited(&request, 5)? {
        let display_name = CNContactFormatter::string_from_contact(
            &contact,
            CNContactFormatterStyle::FullName,
        )?
        .unwrap_or_else(|| contact.display_name());
        println!("{display_name}");
    }

    Ok(())
}
```

## Highlights

- `CNContactStore` authorization, fetch, save, container, group, and history-token helpers
- Expanded `CNContact` / `CNMutableContact` coverage for names, phonetics, notes, image data, dates, relations, social profiles, and instant-message addresses
- `CNMutableGroup`, subgroup/member save operations, and group/container predicate factories
- `CNContactFetchRequest` with predicates plus additional key descriptors for comparator, formatter, and vCard fetches
- `CNContactFormatter` and `CNPostalAddressFormatter` string / attributed-string helpers
- `CNChangeHistoryFetchRequest`, `CNFetchResult`, and `CNContactStoreDidChangeNotification` support
- `CNLabeledValue`, `CNPhoneNumber`, `CNPostalAddress`, `CNSocialProfile`, `CNInstantMessageAddress`, and `CNContactRelation` value types
- `CNContactVCardSerialization` round-tripping between `CNContact` values and vCard bytes

## Examples

The crate ships twelve numbered examples, one per logical area, including:

- `01_contacts_store_smoke`
- `07_format_and_print`
- `08_change_notifications`
- `11_vcard_serialization`

Run them all with:

```bash
for ex in examples/*.rs; do
  cargo run --example "$(basename "$ex" .rs)"
done
```

## Tests and verification

```bash
cargo clippy --all-targets -- -D warnings
cargo test
```

## Authorization

`Contacts.framework` access is gated by the user's privacy settings. Store, group, and container examples never force a permission prompt; they report current availability and keep running in restricted environments.

## Coverage audit

See [COVERAGE.md](COVERAGE.md) for the per-area implementation matrix and the explicitly deferred public APIs.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
