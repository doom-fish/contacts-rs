use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = CNContactStore::authorization_status();
    println!("contacts authorization: {status:?}");

    let store = CNContactStore::new()?;

    if status.is_authorized() {
        let request = CNContactFetchRequest::new([
            CNContactKey::GivenName,
            CNContactKey::FamilyName,
            CNContactKey::OrganizationName,
        ])
        .with_sort_order(CNContactSortOrder::GivenName);

        for (index, contact) in store
            .enumerate_contacts_limited(&request, 5)?
            .into_iter()
            .enumerate()
        {
            println!("{}. {}", index + 1, contact.display_name());
        }
    } else {
        println!("contacts access not granted; skipping enumeration");
    }

    println!("✅ contacts store OK");
    Ok(())
}
