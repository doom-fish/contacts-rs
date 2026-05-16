mod common;

use contacts::prelude::*;

#[test]
fn vcard_roundtrip_in_memory_contact() {
    let contact = common::sample_contact();
    let data =
        CNContactVCardSerialization::data_with_contacts(std::slice::from_ref(&contact)).unwrap();
    let roundtrip = CNContactVCardSerialization::contacts_with_data(&data).unwrap();

    assert!(!data.is_empty());
    assert_eq!(roundtrip.len(), 1);
}
