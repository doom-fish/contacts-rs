use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contact_predicate = CNContactPredicate::matching_phone_number("+1-555-0100");
    let group_predicate = CNGroupPredicate::groups_in_container("container-id");
    let container_predicate = CNContainerPredicate::container_of_contact("contact-id");

    println!("contact: {}", serde_json::to_string(&contact_predicate)?);
    println!("group: {}", serde_json::to_string(&group_predicate)?);
    println!(
        "container: {}",
        serde_json::to_string(&container_predicate)?
    );
    Ok(())
}
