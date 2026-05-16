use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = CNChangeHistoryFetchRequest::new()
        .with_include_group_changes(true)
        .with_excluded_transaction_authors(["contacts-rs-examples"]);

    println!(
        "notification name: {}",
        contact_store_did_change_notification_name()?
    );
    println!("request: {}", serde_json::to_string_pretty(&request)?);

    if CNContactStore::authorization_status() == CNAuthorizationStatus::Authorized {
        let store = CNContactStore::new()?;
        match store.fetch_change_history(&request) {
            Ok(result) => println!("fetched {} change-history events", result.value.len()),
            Err(error) => println!("change-history fetch unavailable in this environment: {error}"),
        }
    } else {
        println!("contacts authorization not granted; skipping runtime change-history fetch");
    }

    Ok(())
}
