#![allow(
    dead_code,
    clippy::assigning_clones,
    clippy::missing_panics_doc,
    clippy::must_use_candidate
)]

use contacts::prelude::*;
use serde_json::json;

pub fn sample_contact() -> CNContact {
    serde_json::from_value(json!({
        "identifier": "example-contact",
        "fetchedKeys": [
            "contactType",
            "givenName",
            "familyName",
            "organizationName",
            "emailAddresses",
            "phoneNumbers",
            "postalAddresses",
            "contactRelations",
            "birthday"
        ],
        "contactType": "person",
        "givenName": "Taylor",
        "familyName": "Appleseed",
        "organizationName": "Example Incorporated",
        "emailAddresses": [
            {"label": "home", "value": "taylor@example.com"}
        ],
        "phoneNumbers": [
            {"label": "mobile", "value": {"stringValue": "+1-555-0100"}}
        ],
        "postalAddresses": [
            {
                "label": "work",
                "value": {
                    "street": "1 Infinite Loop",
                    "city": "Cupertino",
                    "state": "CA",
                    "postalCode": "95014",
                    "country": "USA",
                    "isoCountryCode": "US",
                    "subLocality": "",
                    "subAdministrativeArea": ""
                }
            }
        ],
        "contactRelations": [
            {"label": "manager", "value": {"name": "Jordan Rivers"}}
        ],
        "birthday": {"month": 4, "day": 1}
    }))
    .expect("sample contact JSON should deserialize")
}

pub fn sample_mutable_contact() -> CNMutableContact {
    CNMutableContact::new()
        .with_given_name("Taylor")
        .with_family_name("Appleseed")
        .with_organization_name("Example Incorporated")
        .with_note("Created by contacts-rs tests")
        .with_phone_numbers(vec![CNLabeledValue::new(
            Some("mobile".to_owned()),
            CNPhoneNumber::new("+1-555-0100"),
        )])
        .with_email_addresses(vec![CNLabeledValue::new(
            Some("work".to_owned()),
            "taylor@example.com".to_owned(),
        )])
        .with_contact_relations(vec![sample_labeled_relation()])
}

pub fn sample_postal_address() -> CNPostalAddress {
    let mut address = CNPostalAddress::new("1 Infinite Loop");
    address.city = "Cupertino".to_owned();
    address.state = "CA".to_owned();
    address.postal_code = "95014".to_owned();
    address.country = "USA".to_owned();
    address.iso_country_code = "US".to_owned();
    address
}

pub fn sample_labeled_relation() -> CNLabeledValue<CNContactRelation> {
    CNLabeledValue::new(
        Some("manager".to_owned()),
        CNContactRelation::new("Jordan Rivers"),
    )
}
