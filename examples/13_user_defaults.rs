use contacts::prelude::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let defaults = CNContactsUserDefaults::shared()?;
    println!("sort order: {:?}", defaults.sort_order());
    println!("country code: {}", defaults.country_code()?);

    let contact: CNContact = serde_json::from_value(json!({
        "identifier": "example-contact",
        "fetchedKeys": ["givenName", "familyName"],
        "givenName": "Taylor",
        "familyName": "Appleseed"
    }))?;
    println!(
        "item-provider writable types: {:?}",
        contact.writable_type_identifiers_for_item_provider()?
    );
    println!(
        "item-provider readable types: {:?}",
        CNContact::readable_type_identifiers_for_item_provider()?
    );
    println!(
        "record-does-not-exist raw code: {}",
        CNErrorCode::RecordDoesNotExist.raw_value()
    );

    Ok(())
}
