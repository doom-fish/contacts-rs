use contacts::prelude::*;

#[test]
fn notification_name_is_not_empty() {
    let name = contact_store_did_change_notification_name().unwrap();
    assert!(!name.is_empty());
}

#[test]
fn change_history_request_builder_sets_flags() {
    let request = CNChangeHistoryFetchRequest::new()
        .with_include_group_changes(true)
        .with_excluded_transaction_authors(["contacts-rs-tests"]);

    assert!(request.include_group_changes);
    assert_eq!(
        request.excluded_transaction_authors,
        vec!["contacts-rs-tests"]
    );
}
