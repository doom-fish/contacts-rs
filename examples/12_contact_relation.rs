mod support;

use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relation = CNContactRelation::new("Jordan Rivers");
    let labeled = support::sample_labeled_relation();
    let contact = CNMutableContact::new()
        .with_given_name("Taylor")
        .with_contact_relations(vec![labeled]);

    println!("relation: {}", relation.name);
    println!(
        "contact payload: {}",
        serde_json::to_string_pretty(&contact)?
    );
    Ok(())
}
