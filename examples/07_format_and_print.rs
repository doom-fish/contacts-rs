mod support;

use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contact = support::sample_contact()?;
    let address = support::sample_postal_address();

    let formatted_name =
        CNContactFormatter::string_from_contact(&contact, CNContactFormatterStyle::FullName)?;
    let formatted_address = CNPostalAddressFormatter::string_from_postal_address(
        &address,
        CNPostalAddressFormatterStyle::MailingAddress,
    )?;

    println!("formatted name: {formatted_name:?}");
    println!("formatted address: {formatted_address}");
    Ok(())
}
