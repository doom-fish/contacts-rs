mod support;

use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contact = support::sample_contact()?;
    println!("display name: {}", contact.display_name());
    println!(
        "has given name: {}",
        contact.is_key_available(CNContactKey::GivenName)
    );
    println!(
        "localized key: {}",
        CNContact::localized_string_for_key(CNContactKey::GivenName)?
    );
    Ok(())
}
