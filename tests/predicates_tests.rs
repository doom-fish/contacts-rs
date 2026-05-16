use contacts::prelude::*;

#[test]
fn predicate_constructors_serialize() {
    let contact = CNContactPredicate::matching_email_address("taylor@example.com");
    let group = CNGroupPredicate::subgroups_in_group("parent-group");
    let container = CNContainerPredicate::container_of_group("group-id");

    assert!(serde_json::to_string(&contact)
        .unwrap()
        .contains("matchingEmailAddress"));
    assert!(serde_json::to_string(&group)
        .unwrap()
        .contains("subgroupsInGroupWithIdentifier"));
    assert!(serde_json::to_string(&container)
        .unwrap()
        .contains("containerOfGroupWithIdentifier"));
}
