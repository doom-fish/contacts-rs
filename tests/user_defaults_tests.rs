use contacts::prelude::*;

#[test]
fn shared_user_defaults_expose_sort_order_and_country_code() {
    let defaults = CNContactsUserDefaults::shared().unwrap();
    let _ = defaults.sort_order();
    let country_code = defaults.country_code().unwrap();

    assert!(!country_code.is_empty());
}
