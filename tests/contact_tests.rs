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
