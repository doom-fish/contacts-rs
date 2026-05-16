use contacts::prelude::*;

#[test]
fn mutable_group_roundtrip() {
    let group = CNGroup::new("group-id", "Friends");
    let mutable = CNMutableGroup::from(group);
    assert_eq!(mutable.identifier.as_deref(), Some("group-id"));
    assert_eq!(mutable.name.as_deref(), Some("Friends"));
}
