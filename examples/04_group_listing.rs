use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = CNContactStore::new()?;
    match store.groups() {
        Ok(groups) => println!("group count: {}", groups.len()),
        Err(error) => println!("group listing unavailable in this environment: {error}"),
    }
    Ok(())
}
