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
    FullName,
    PhoneticFullName,
}

/// Corresponds to `CNContactDisplayNameOrder`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactDisplayNameOrder {
    UserDefault,
    GivenNameFirst,
    FamilyNameFirst,
}

/// Corresponds to `CNPostalAddressFormatterStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CNPostalAddressFormatterStyle {
    #[default]
    MailingAddress,
}

/// A lightweight attributed string representation returned by formatter helpers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNAttributedString {
    pub string: String,
    #[serde(default)]
    pub runs: Vec<CNAttributedStringRun>,
}

/// A contiguous attributed range from a Contacts formatter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CNAttributedStringRun {
    pub location: usize,
    pub length: usize,
    pub value: String,
    #[serde(default)]
    pub property: Option<String>,
    #[serde(default)]
    pub localized_property_name: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CNContactFormatter;

impl CNContactFormatter {
    pub fn descriptor_for_required_keys_for_style(
        style: CNContactFormatterStyle,
    ) -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::FormatterRequiredKeys { style }
    }

    pub fn descriptor_for_required_keys_for_name_order() -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::FormatterNameOrder
    }

    pub fn descriptor_for_required_keys_for_delimiter() -> CNAdditionalKeyDescriptor {
        CNAdditionalKeyDescriptor::FormatterDelimiter
    }

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
pub struct CNPostalAddressFormatter {
    pub style: CNPostalAddressFormatterStyle,
}

impl CNPostalAddressFormatter {
    pub fn new(style: CNPostalAddressFormatterStyle) -> Self {
        Self { style }
    }

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

    pub fn string_from(&self, postal_address: &CNPostalAddress) -> Result<String, ContactsError> {
        Self::string_from_postal_address(postal_address, self.style)
    }

    pub fn attributed_string_from(
        &self,
        postal_address: &CNPostalAddress,
    ) -> Result<CNAttributedString, ContactsError> {
        Self::attributed_string_from_postal_address(postal_address, self.style)
    }
}
