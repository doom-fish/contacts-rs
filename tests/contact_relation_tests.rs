mod common;

#[test]
fn relation_can_be_embedded_in_labeled_value() {
    let relation = common::sample_labeled_relation();
    assert_eq!(relation.value.name, "Jordan Rivers");
    assert_eq!(relation.label.as_deref(), Some("manager"));
}
