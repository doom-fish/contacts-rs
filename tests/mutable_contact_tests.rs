mod common;

#[test]
fn builder_sets_expected_fields() {
    let contact = common::sample_mutable_contact();
    assert_eq!(contact.display_name(), "Taylor Appleseed");
    assert!(contact.note.as_deref().unwrap().contains("tests"));
    assert_eq!(contact.phone_numbers.as_ref().unwrap().len(), 1);
}

#[test]
fn clear_flags_can_be_enabled() {
    let contact = common::sample_mutable_contact()
        .clear_birthday()
        .clear_image_data();
    assert!(contact.clear_birthday);
    assert!(contact.clear_image_data);
}
