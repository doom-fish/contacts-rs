use contacts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = CNContactStore::new()?;
    match store.containers() {
        Ok(containers) => println!("container count: {}", containers.len()),
        Err(error) => println!("container listing unavailable in this environment: {error}"),
    }
    Ok(())
}
