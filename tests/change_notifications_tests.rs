use contacts::prelude::*;

#[test]
fn notification_name_is_not_empty() {
    let name = contact_store_did_change_notification_name().unwrap();
    assert!(!name.is_empty());
}

#[test]
fn change_history_request_builder_sets_flags() {
    let raw_key = CNContactKey::FamilyName.value().unwrap();
    let request = CNChangeHistoryFetchRequest::new()
        .with_key_descriptors([
            CNKeyDescriptor::from(CNContactKey::GivenName),
            CNKeyDescriptor::from(CNContact::descriptor_for_all_comparator_keys()),
            CNKeyDescriptor::raw(raw_key.clone()),
        ])
        .with_include_group_changes(true)
        .with_excluded_transaction_authors(["contacts-rs-tests"]);

    assert!(request.include_group_changes);
    assert_eq!(
        request.excluded_transaction_authors,
        vec!["contacts-rs-tests"]
    );
    assert_eq!(
        request.key_descriptors(),
        vec![
            CNKeyDescriptor::contact_key(CNContactKey::GivenName),
            CNKeyDescriptor::additional(CNContact::descriptor_for_all_comparator_keys()),
            CNKeyDescriptor::raw(raw_key),
        ],
    );
}

#[derive(Default)]
struct Recorder {
    calls: Vec<String>,
}

impl CNChangeHistoryEventVisitor for Recorder {
    fn visit_drop_everything_event(&mut self) {
        self.calls.push("dropEverything".to_owned());
    }

    fn visit_add_contact_event(&mut self, contact: &CNContact, container_identifier: Option<&str>) {
        self.calls.push(format!(
            "addContact:{}:{}",
            contact.identifier,
            container_identifier.unwrap_or_default()
        ));
    }

    fn visit_update_contact_event(&mut self, contact: &CNContact) {
        self.calls
            .push(format!("updateContact:{}", contact.identifier));
    }

    fn visit_delete_contact_event(&mut self, contact_identifier: &str) {
        self.calls
            .push(format!("deleteContact:{contact_identifier}"));
    }

    fn visit_add_group_event(&mut self, group: &CNGroup, container_identifier: &str) {
        self.calls.push(format!(
            "addGroup:{}:{}",
            group.identifier, container_identifier
        ));
    }
}

#[test]
fn change_history_event_accepts_visitor() {
    let mut recorder = Recorder::default();
    CNChangeHistoryEvent::DropEverything.accept_visitor(&mut recorder);
    CNChangeHistoryEvent::DeleteContact {
        contact_identifier: "contact-id".to_owned(),
    }
    .accept_visitor(&mut recorder);
    CNChangeHistoryEvent::AddGroup {
        group: CNGroup::new("group-id", "Friends"),
        container_identifier: "container-id".to_owned(),
    }
    .accept_visitor(&mut recorder);

    assert_eq!(
        recorder.calls,
        vec![
            "dropEverything",
            "deleteContact:contact-id",
            "addGroup:group-id:container-id",
        ]
    );
}
