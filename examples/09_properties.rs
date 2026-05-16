use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "localized label: {}",
        CNLabeledValue::<String>::localized_string_for_label("home")?
    );
    println!(
        "postal street key: {}",
        CNPostalAddress::localized_string_for_key(CNPostalAddressKey::Street)?
    );
    println!(
        "social username key: {}",
        CNSocialProfile::localized_string_for_key(CNSocialProfileKey::Username)?
    );
    Ok(())
}
