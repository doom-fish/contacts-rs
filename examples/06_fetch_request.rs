use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = CNContactFetchRequest::new([])
        .with_key_descriptors([
            CNKeyDescriptor::from(CNContactKey::GivenName),
            CNKeyDescriptor::from(CNContactKey::FamilyName),
            CNKeyDescriptor::from(CNContactFormatter::descriptor_for_required_keys_for_style(
                CNContactFormatterStyle::FullName,
            )),
        ])
        .with_predicate(CNContactPredicate::matching_name("Taylor"))
        .with_sort_order(CNContactSortOrder::GivenName);

    let generic_request = CNFetchRequest::from(request);
    println!("{}", serde_json::to_string_pretty(&generic_request)?);
    Ok(())
}
