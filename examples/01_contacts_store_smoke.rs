use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = CNContactStore::authorization_status();
    println!("contacts authorization: {status:?}");

    let store = CNContactStore::new()?;
    println!(
        "default container: {:?}",
        store.default_container_identifier()
    );

    if status.is_authorized() {
        let request = CNContactFetchRequest::new([
            CNContactKey::GivenName,
            CNContactKey::FamilyName,
            CNContactKey::OrganizationName,
        ])
        .with_sort_order(CNContactSortOrder::GivenName);

        match store.enumerate_contacts_limited(&request, 5) {
            Ok(contacts) => {
                for (index, contact) in contacts.into_iter().enumerate() {
                    println!("{}. {}", index + 1, contact.display_name());
                }
            }
            Err(error) => {
                println!("contact enumeration unavailable in this environment: {error}");
            }
        }
    } else {
        println!("contacts access not granted; skipping enumeration");
    }

    println!("✅ contacts store OK");
    Ok(())
}
