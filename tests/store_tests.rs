mod common;

use contacts::prelude::*;

#[test]
fn authorization_status_smoke() {
    assert_eq!(
        CNContactStore::authorization_status(),
        CNContactStore::authorization_status_for_entity_type(CNEntityType::Contacts)
    );
}

#[test]
fn save_request_builders_capture_operations() {
    let mut request = CNSaveRequest::new();
    request
        .add_contact(common::sample_mutable_contact(), None)
        .delete_contact_by_identifier("contact-id")
        .add_group(CNMutableGroup::new().with_name("Friends"), None)
        .add_member("contact-id", "group-id")
        .add_subgroup("child-group", "parent-group");

    assert_eq!(request.operations.len(), 5);
}
