mod common;

use contacts::prelude::*;

#[test]
fn display_name_prefers_person_name() {
    let contact = common::sample_contact();
    assert_eq!(contact.display_name(), "Taylor Appleseed");
    assert!(contact.is_key_available(CNContactKey::GivenName));
}

#[test]
fn localized_contact_key_is_not_empty() {
    let key = CNContact::localized_string_for_key(CNContactKey::GivenName).unwrap();
    assert!(!key.is_empty());
}

#[test]
fn item_provider_roundtrip_preserves_in_memory_contact() {
    let contact = common::sample_contact();
    let readable_type_identifiers =
        CNContact::readable_type_identifiers_for_item_provider().unwrap();
    let writable_type_identifiers = contact
        .writable_type_identifiers_for_item_provider()
        .unwrap();

    assert!(!readable_type_identifiers.is_empty());
    assert!(!writable_type_identifiers.is_empty());

    let type_identifier = writable_type_identifiers[0].clone();
    let payload = contact.item_provider_data(&type_identifier).unwrap();
    let roundtrip = CNContact::from_item_provider_data(&payload, &type_identifier).unwrap();

    assert_eq!(roundtrip.given_name, contact.given_name);
    assert_eq!(roundtrip.family_name, contact.family_name);
}
