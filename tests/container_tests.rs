use contacts::prelude::*;

#[test]
fn container_constructor_sets_fields() {
    let container = CNContainer::new("container-id", "Default", CNContainerType::Local);
    assert_eq!(container.identifier, "container-id");
    assert_eq!(container.container_type, CNContainerType::Local);
}
