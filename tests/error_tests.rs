use contacts::prelude::*;

#[test]
fn contacts_error_code_maps_known_and_unknown_values() {
    assert_eq!(CNErrorCode::from_raw(200), CNErrorCode::RecordDoesNotExist);
    assert_eq!(CNErrorCode::FeatureNotAvailable.raw_value(), 104);
    assert_eq!(CNErrorCode::from_raw(999), CNErrorCode::Unknown(999));
}
