mod common;

use contacts::prelude::*;

#[test]
fn formatters_format_in_memory_values() {
    let contact = common::sample_contact();
    let address = common::sample_postal_address();

    let formatted_name =
        CNContactFormatter::string_from_contact(&contact, CNContactFormatterStyle::FullName)
            .unwrap();
    let formatted_address = CNPostalAddressFormatter::string_from_postal_address(
        &address,
        CNPostalAddressFormatterStyle::MailingAddress,
    )
    .unwrap();

    assert!(formatted_name.unwrap_or_default().contains("Taylor"));
    assert!(formatted_address.contains("Cupertino"));
}
