mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contact = support::sample_mutable_contact();
    println!("display name: {}", contact.display_name());
    println!("payload: {}", serde_json::to_string_pretty(&contact)?);
    Ok(())
}
