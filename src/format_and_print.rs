//! Formatting helpers for contacts and postal addresses.

use serde::{Deserialize, Serialize};

use crate::contact::CNContact;
use crate::error::ContactsError;
use crate::fetch_request::CNAdditionalKeyDescriptor;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr, take_string};
use crate::properties::CNPostalAddress;

/// Corresponds to `CNContactFormatterStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactFormatterStyle {
    /// The full name variant.
    FullName,
    /// The phonetic full name variant.
    PhoneticFullName,
}

/// Corresponds to `CNContactDisplayNameOrder`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactDisplayNameOrder {
    /// The user default order.
    UserDefault,
    /// The given naMe first order.
    GivenNameFirst,
    /// The family naMe first order.
    FamilyNameFirst,
}

/// Corresponds to `CNPostalAddressFormatterStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CNPostalAddressFormatterStyle {
    #[default]
    /// The mailing address variant.
    MailingAddress,
}

/// A lightweight attributed string representation returned by formatter helpers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNAttributedString {
    /// The plain string value.
    pub string: String,
    #[serde(default)]
    /// The attributed runs.
    pub runs: Vec<CNAttributedStringRun>,
}

/// A contiguous attributed range from a Contacts formatter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNAttributedStringRun {
    /// The location.
    pub location: usize,
    /// The length.
    pub length: usize,
    /// The value.
    pub value: String,
    #[serde(default)]
    /// The property.
    pub property: Option<String>,
    #[serde(default)]
    /// The localized property name.
    pub localized_property_name: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
/// The `CNContactFormatter` namespace wrapper.
pub struct CNContactFormatter;

impl CNContactFormatter {
    /// Returns the descriptor for the formatter style.
    pub fn descriptor_for_required_keys_for_style(
        style: CNContactFormatterStyle,
    ) -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::FormatterRequiredKeys { style }
    }

    /// Returns the descriptor for the formatter name-order keys.
    pub fn descriptor_for_required_keys_for_name_order() -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::FormatterNameOrder
    }

    /// Returns the descriptor for the formatter delimiter keys.
    pub fn descriptor_for_required_keys_for_delimiter() -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::FormatterDelimiter
    }

    /// Formats the contact as a string.
    pub fn string_from_contact(
        contact: &CNContact,
        style: CNContactFormatterStyle,
    ) -> Result<Option<String>, ContactsError> {
        let contact_json = json_cstring(contact, "CNContact")?;
        let style_json = json_cstring(&style, "CNContactFormatterStyle")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::format_and_print::cn_contact_formatter_string_from_contact_json(
                contact_json.as_ptr(),
                style_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            if error.is_null() {
                Ok(None)
            } else {
                Err(unsafe {
                    ContactsError::from_error_ptr(
                        error,
                        "CNContactFormatter.string(from:style:) failed",
                    )
                })
            }
        } else {
            Ok(unsafe { take_string(value) })
        }
    }

    /// Formats the contact as an attributed string.
    pub fn attributed_string_from_contact(
        contact: &CNContact,
        style: CNContactFormatterStyle,
    ) -> Result<Option<CNAttributedString>, ContactsError> {
        let contact_json = json_cstring(contact, "CNContact")?;
        let style_json = json_cstring(&style, "CNContactFormatterStyle")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::format_and_print::cn_contact_formatter_attributed_string_from_contact_json(
                contact_json.as_ptr(),
                style_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNContactFormatter.attributedString(from:style:defaultAttributes:) failed",
                )
            })
        } else {
            unsafe { parse_json_ptr(value, "optional CNAttributedString") }
        }
    }

    /// Returns the display-name order for the contact.
    pub fn name_order_for_contact(
        contact: &CNContact,
    ) -> Result<CNContactDisplayNameOrder, ContactsError> {
        let contact_json = json_cstring(contact, "CNContact")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::format_and_print::cn_contact_formatter_name_order_from_contact_json(
                contact_json.as_ptr(),
                &mut error,
            )
        };
        match value {
            0 => Ok(CNContactDisplayNameOrder::UserDefault),
            1 => Ok(CNContactDisplayNameOrder::GivenNameFirst),
            2 => Ok(CNContactDisplayNameOrder::FamilyNameFirst),
            _ => Err(unsafe {
                ContactsError::from_error_ptr(error, "CNContactFormatter.nameOrder(for:) failed")
            }),
        }
    }

    /// Returns the delimiter for the contact.
    pub fn delimiter_for_contact(contact: &CNContact) -> Result<String, ContactsError> {
        let contact_json = json_cstring(contact, "CNContact")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::format_and_print::cn_contact_formatter_delimiter_for_contact_json(
                contact_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(error, "CNContactFormatter.delimiter(for:) failed")
            })
        } else {
            unsafe {
                crate::private::take_required_string(value, "CNContactFormatter.delimiter(for:)")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
/// The `CNPostalAddressFormatter` wrapper.
pub struct CNPostalAddressFormatter {
    /// The formatter style.
    pub style: CNPostalAddressFormatterStyle,
}

impl CNPostalAddressFormatter {
    /// Creates a new `CNPostalAddressFormatter`.
    pub fn new(style: CNPostalAddressFormatterStyle) -> Self {
        Self { style }
    }

    /// Formats the postal address as a string.
    pub fn string_from_postal_address(
        postal_address: &CNPostalAddress,
        style: CNPostalAddressFormatterStyle,
    ) -> Result<String, ContactsError> {
        let address_json = json_cstring(postal_address, "CNPostalAddress")?;
        let style_json = json_cstring(&style, "CNPostalAddressFormatterStyle")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::format_and_print::cn_postal_address_formatter_string_from_postal_address_json(
                address_json.as_ptr(),
                style_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNPostalAddressFormatter.string(from:style:) failed",
                )
            })
        } else {
            unsafe {
                crate::private::take_required_string(
                    value,
                    "CNPostalAddressFormatter.string(from:style:)",
                )
            }
        }
    }

    /// Formats the postal address as an attributed string.
    pub fn attributed_string_from_postal_address(
        postal_address: &CNPostalAddress,
        style: CNPostalAddressFormatterStyle,
    ) -> Result<CNAttributedString, ContactsError> {
        let address_json = json_cstring(postal_address, "CNPostalAddress")?;
        let style_json = json_cstring(&style, "CNPostalAddressFormatterStyle")?;
        let mut error = core::ptr::null_mut();
        let value = unsafe {
            ffi::format_and_print::cn_postal_address_formatter_attributed_string_from_postal_address_json(
                address_json.as_ptr(),
                style_json.as_ptr(),
                &mut error,
            )
        };
        if value.is_null() {
            Err(unsafe {
                ContactsError::from_error_ptr(
                    error,
                    "CNPostalAddressFormatter.attributedString(from:style:withDefaultAttributes:) failed",
                )
            })
        } else {
            unsafe { parse_json_ptr(value, "CNAttributedString") }
        }
    }

    /// Formats the postal address using this formatter.
    pub fn string_from(&self, postal_address: &CNPostalAddress) -> Result<String, ContactsError> {
        Self::string_from_postal_address(postal_address, self.style)
    }

    /// Formats the postal address as an attributed string using this formatter.
    pub fn attributed_string_from(
        &self,
        postal_address: &CNPostalAddress,
    ) -> Result<CNAttributedString, ContactsError> {
        Self::attributed_string_from_postal_address(postal_address, self.style)
    }
}
