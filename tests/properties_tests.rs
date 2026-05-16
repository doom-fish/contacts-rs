use contacts::prelude::*;

#[test]
fn localized_property_helpers_return_strings() {
    assert!(
        !CNLabeledValue::<String>::localized_string_for_label("home")
            .unwrap()
            .is_empty()
    );
    assert!(
        !CNPostalAddress::localized_string_for_key(CNPostalAddressKey::Street)
            .unwrap()
            .is_empty()
    );
    assert!(
        !CNSocialProfile::localized_string_for_key(CNSocialProfileKey::Username)
            .unwrap()
            .is_empty()
    );
}
