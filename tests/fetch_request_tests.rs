use contacts::prelude::*;

#[test]
fn fetch_request_serializes_predicate_and_descriptor() {
    let request = CNContactFetchRequest::new([CNContactKey::GivenName, CNContactKey::FamilyName])
        .with_predicate(CNContactPredicate::matching_name("Taylor"))
        .with_descriptor(CNContactFormatter::descriptor_for_required_keys_for_style(
            CNContactFormatterStyle::FullName,
        ));

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("matchingName"));
    assert!(json.contains("formatterRequiredKeys"));
}
