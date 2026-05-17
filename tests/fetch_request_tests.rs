use contacts::prelude::*;

#[test]
fn fetch_request_serializes_predicate_and_descriptor() {
    let request = CNContactFetchRequest::new([CNContactKey::GivenName, CNContactKey::FamilyName])
        .with_predicate(CNContactPredicate::matching_name("Taylor"))
        .with_key_descriptor(CNKeyDescriptor::from(
            CNContactFormatter::descriptor_for_required_keys_for_style(
                CNContactFormatterStyle::FullName,
            ),
        ));

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("matchingName"));
    assert!(json.contains("formatterRequiredKeys"));
}

#[test]
fn generic_fetch_request_preserves_specific_contact_request() {
    let raw_key = CNContactKey::GivenName.value().unwrap();
    let request = CNContactFetchRequest::new([])
        .with_key_descriptors([
            CNKeyDescriptor::from(CNContactKey::GivenName),
            CNKeyDescriptor::from(CNContactKey::FamilyName),
            CNKeyDescriptor::from(CNContactFormatter::descriptor_for_required_keys_for_style(
                CNContactFormatterStyle::FullName,
            )),
            CNKeyDescriptor::raw(raw_key.clone()),
        ])
        .with_sort_order(CNContactSortOrder::GivenName);

    assert_eq!(
        request.key_descriptors(),
        vec![
            CNKeyDescriptor::contact_key(CNContactKey::GivenName),
            CNKeyDescriptor::contact_key(CNContactKey::FamilyName),
            CNKeyDescriptor::additional(
                CNContactFormatter::descriptor_for_required_keys_for_style(
                    CNContactFormatterStyle::FullName,
                ),
            ),
            CNKeyDescriptor::raw(raw_key),
        ],
    );

    let generic_request = CNFetchRequest::from(request.clone());
    assert_eq!(generic_request.as_contact(), Some(&request));
    assert!(generic_request.as_change_history().is_none());
}
