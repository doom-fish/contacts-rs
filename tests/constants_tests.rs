use contacts::prelude::*;

#[test]
fn constant_families_resolve_native_values() {
    for key in CNContactKey::all_supported() {
        assert!(!key.value().unwrap().is_empty());
    }

    for &key in CNContainerKey::all_supported() {
        assert!(!key.value().unwrap().is_empty());
    }
    for &key in CNGroupKey::all_supported() {
        assert!(!key.value().unwrap().is_empty());
    }
    for &key in CNPostalAddressKey::all_supported() {
        assert!(!key.value().unwrap().is_empty());
    }
    for &key in CNInstantMessageAddressKey::all_supported() {
        assert!(!key.value().unwrap().is_empty());
    }
    for &key in CNSocialProfileKey::all_supported() {
        assert!(!key.value().unwrap().is_empty());
    }

    for &label in CNLabeledValueLabel::all_supported() {
        assert!(!label.value().unwrap().is_empty());
    }
    for &label in CNEmailAddressLabel::all_supported() {
        assert!(!label.value().unwrap().is_empty());
    }
    for &label in CNUrlAddressLabel::all_supported() {
        assert!(!label.value().unwrap().is_empty());
    }
    for &label in CNDateLabel::all_supported() {
        assert!(!label.value().unwrap().is_empty());
    }
    for &label in CNPhoneNumberLabel::all_supported() {
        assert!(!label.value().unwrap().is_empty());
    }
    for &label in CNContactRelationLabel::all_supported() {
        assert!(!label.value().unwrap().is_empty());
    }

    for &service in CNInstantMessageService::all_supported() {
        assert!(!service.value().unwrap().is_empty());
    }
    for &service in CNSocialProfileService::all_supported() {
        assert!(!service.value().unwrap().is_empty());
    }
    for &key in CNErrorUserInfoKey::all_supported() {
        assert!(!key.value().unwrap().is_empty());
    }

    assert!(!contact_property_not_fetched_exception_name()
        .unwrap()
        .is_empty());
    assert!(!contacts_error_domain().unwrap().is_empty());

    assert!(!CNContactKey::GivenName
        .localized_string()
        .unwrap()
        .is_empty());
    assert!(!CNPostalAddressKey::Street
        .localized_string()
        .unwrap()
        .is_empty());
    assert!(!CNInstantMessageAddressKey::Username
        .localized_string()
        .unwrap()
        .is_empty());
    assert!(!CNSocialProfileKey::Username
        .localized_string()
        .unwrap()
        .is_empty());
    assert!(!CNLabeledValueLabel::Home
        .localized_string()
        .unwrap()
        .is_empty());
    assert!(!CNContactRelationLabel::Friend
        .localized_string()
        .unwrap()
        .is_empty());
    assert!(!CNInstantMessageService::Skype
        .localized_string()
        .unwrap()
        .is_empty());
    assert!(!CNSocialProfileService::Twitter
        .localized_string()
        .unwrap()
        .is_empty());
}

#[test]
fn mutable_postal_address_roundtrips() {
    let mutable = CNMutablePostalAddress::new()
        .with_street("1 Infinite Loop")
        .with_city("Cupertino")
        .with_state("CA")
        .with_postal_code("95014")
        .with_country("USA")
        .with_iso_country_code("US");

    let immutable: CNPostalAddress = mutable.clone().into();
    let roundtrip: CNMutablePostalAddress = immutable.into();

    assert_eq!(roundtrip, mutable);
}
