use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = CNContactFetchRequest::new([CNContactKey::GivenName, CNContactKey::FamilyName])
        .with_predicate(CNContactPredicate::matching_name("Taylor"))
        .with_descriptor(CNContactFormatter::descriptor_for_required_keys_for_style(
            CNContactFormatterStyle::FullName,
        ))
        .with_sort_order(CNContactSortOrder::GivenName);

    println!("{}", serde_json::to_string_pretty(&request)?);
    Ok(())
}
