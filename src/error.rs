use core::fmt;
use std::ffi::CStr;

use serde::{Deserialize, Serialize};

use crate::ffi;

/// Authorization state reported by `CNContactStore.authorizationStatus(for:)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CNAuthorizationStatus {
    NotDetermined,
    Restricted,
    Denied,
    Authorized,
    Limited,
    Unknown(i32),
}

impl CNAuthorizationStatus {
    pub(crate) const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::NotDetermined,
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::Authorized,
            4 => Self::Limited,
            other => Self::Unknown(other),
        }
    }

    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::Authorized | Self::Limited)
    }
}

/// Typed wrapper for `CNErrorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CNErrorCode {
    CommunicationError,
    DataAccessError,
    AuthorizationDenied,
    NoAccessableWritableContainers,
    UnauthorizedKeys,
    FeatureDisabledByUser,
    FeatureNotAvailable,
    RecordDoesNotExist,
    InsertedRecordAlreadyExists,
    ContainmentCycle,
    ContainmentScope,
    ParentRecordDoesNotExist,
    RecordIdentifierInvalid,
    RecordNotWritable,
    ParentContainerNotWritable,
    ValidationMultipleErrors,
    ValidationTypeMismatch,
    ValidationConfigurationError,
    PredicateInvalid,
    PolicyViolation,
    ClientIdentifierInvalid,
    ClientIdentifierDoesNotExist,
    ClientIdentifierCollision,
    ChangeHistoryExpired,
    ChangeHistoryInvalidAnchor,
    ChangeHistoryInvalidFetchRequest,
    VCardMalformed,
    VCardSummarizationError,
    Unknown(i64),
}

impl CNErrorCode {
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            1 => Self::CommunicationError,
            2 => Self::DataAccessError,
            100 => Self::AuthorizationDenied,
            101 => Self::NoAccessableWritableContainers,
            102 => Self::UnauthorizedKeys,
            103 => Self::FeatureDisabledByUser,
            104 => Self::FeatureNotAvailable,
            200 => Self::RecordDoesNotExist,
            201 => Self::InsertedRecordAlreadyExists,
            202 => Self::ContainmentCycle,
            203 => Self::ContainmentScope,
            204 => Self::ParentRecordDoesNotExist,
            205 => Self::RecordIdentifierInvalid,
            206 => Self::RecordNotWritable,
            207 => Self::ParentContainerNotWritable,
            300 => Self::ValidationMultipleErrors,
            301 => Self::ValidationTypeMismatch,
            302 => Self::ValidationConfigurationError,
            400 => Self::PredicateInvalid,
            500 => Self::PolicyViolation,
            600 => Self::ClientIdentifierInvalid,
            601 => Self::ClientIdentifierDoesNotExist,
            602 => Self::ClientIdentifierCollision,
            603 => Self::ChangeHistoryExpired,
            604 => Self::ChangeHistoryInvalidAnchor,
            605 => Self::ChangeHistoryInvalidFetchRequest,
            700 => Self::VCardMalformed,
            701 => Self::VCardSummarizationError,
            other => Self::Unknown(other),
        }
    }

    pub const fn raw_value(self) -> i64 {
        match self {
            Self::CommunicationError => 1,
            Self::DataAccessError => 2,
            Self::AuthorizationDenied => 100,
            Self::NoAccessableWritableContainers => 101,
            Self::UnauthorizedKeys => 102,
            Self::FeatureDisabledByUser => 103,
            Self::FeatureNotAvailable => 104,
            Self::RecordDoesNotExist => 200,
            Self::InsertedRecordAlreadyExists => 201,
            Self::ContainmentCycle => 202,
            Self::ContainmentScope => 203,
            Self::ParentRecordDoesNotExist => 204,
            Self::RecordIdentifierInvalid => 205,
            Self::RecordNotWritable => 206,
            Self::ParentContainerNotWritable => 207,
            Self::ValidationMultipleErrors => 300,
            Self::ValidationTypeMismatch => 301,
            Self::ValidationConfigurationError => 302,
            Self::PredicateInvalid => 400,
            Self::PolicyViolation => 500,
            Self::ClientIdentifierInvalid => 600,
            Self::ClientIdentifierDoesNotExist => 601,
            Self::ClientIdentifierCollision => 602,
            Self::ChangeHistoryExpired => 603,
            Self::ChangeHistoryInvalidAnchor => 604,
            Self::ChangeHistoryInvalidFetchRequest => 605,
            Self::VCardMalformed => 700,
            Self::VCardSummarizationError => 701,
            Self::Unknown(raw) => raw,
        }
    }
}

/// Structured `NSError` details encoded by the Swift bridge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NSErrorInfo {
    pub domain: String,
    pub code: i64,
    pub message: String,
}

impl fmt::Display for NSErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) [{}]", self.message, self.code, self.domain)
    }
}

/// Errors returned by the Contacts bridge.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContactsError {
    InvalidArgument(String),
    Framework(NSErrorInfo),
    OperationFailed(String),
}

impl fmt::Display for ContactsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Framework(error) => write!(f, "Contacts.framework error: {error}"),
            Self::OperationFailed(message) => write!(f, "contacts operation failed: {message}"),
        }
    }
}

impl std::error::Error for ContactsError {}

impl ContactsError {
    pub(crate) unsafe fn from_error_ptr(error_ptr: *mut core::ffi::c_char, fallback: &str) -> Self {
        if error_ptr.is_null() {
            return Self::OperationFailed(fallback.to_owned());
        }

        let message = CStr::from_ptr(error_ptr).to_string_lossy().into_owned();
        ffi::core::cn_string_free(error_ptr);

        if let Ok(payload) = serde_json::from_str::<NSErrorInfo>(&message) {
            Self::Framework(payload)
        } else {
            Self::OperationFailed(message)
        }
    }
}
