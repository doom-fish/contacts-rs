mod support;

use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contact = support::sample_contact()?;
    let data = CNContactVCardSerialization::data_with_contacts(std::slice::from_ref(&contact))?;
    let roundtrip = CNContactVCardSerialization::contacts_with_data(&data)?;

    println!("encoded bytes: {}", data.len());
    println!("decoded contacts: {}", roundtrip.len());
    Ok(())
}
