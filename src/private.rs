#![allow(clippy::missing_errors_doc)]

use base64::{engine::general_purpose::STANDARD, Engine as _};
use core::ffi::c_char;
use std::ffi::{CStr, CString};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::ContactsError;
use crate::ffi;

pub fn cstring_from_str(value: &str, context: &str) -> Result<CString, ContactsError> {
    CString::new(value).map_err(|error| {
        ContactsError::InvalidArgument(format!("{context} contains NUL byte: {error}"))
    })
}

pub fn json_cstring<T: Serialize + ?Sized>(
    value: &T,
    context: &str,
) -> Result<CString, ContactsError> {
    let json = serde_json::to_string(value).map_err(|error| {
        ContactsError::InvalidArgument(format!("failed to encode {context} as JSON: {error}"))
    })?;
    cstring_from_str(&json, context)
}

pub unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::core::cn_string_free(ptr);
    Some(string)
}

pub unsafe fn take_required_string(
    ptr: *mut c_char,
    context: &str,
) -> Result<String, ContactsError> {
    take_string(ptr).ok_or_else(|| {
        ContactsError::OperationFailed(format!("missing string payload for {context}"))
    })
}

pub unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, ContactsError> {
    let json = take_required_string(ptr, context)?;

    serde_json::from_str(&json).map_err(|error| {
        ContactsError::OperationFailed(format!(
            "failed to parse {context} JSON: {error}; payload={json}"
        ))
    })
}

pub fn decode_base64_string(value: &str, context: &str) -> Result<Vec<u8>, ContactsError> {
    STANDARD.decode(value).map_err(|error| {
        ContactsError::OperationFailed(format!(
            "failed to decode {context} base64 payload: {error}"
        ))
    })
}

pub fn encode_base64_bytes(value: &[u8]) -> String {
    STANDARD.encode(value)
}

pub mod serde_base64 {
    pub mod required {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S>(value: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&STANDARD.encode(value))
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = String::deserialize(deserializer)?;
            STANDARD.decode(value).map_err(serde::de::Error::custom)
        }
    }

    pub mod option {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        use serde::{Deserialize, Deserializer, Serializer};

        #[allow(clippy::ref_option)]
        pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(value) => serializer.serialize_some(&STANDARD.encode(value)),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = Option::<String>::deserialize(deserializer)?;
            value
                .map(|value| STANDARD.decode(value).map_err(serde::de::Error::custom))
                .transpose()
        }
    }
}
