use serde::{Deserialize, Serialize};

use crate::contact::{CNContact, CNContactKey};
use crate::error::ContactsError;
use crate::ffi;
use crate::private::{cstring_from_str, take_required_string};
use crate::properties::{
    CNInstantMessageAddress, CNInstantMessageAddressKey, CNLabeledValue, CNPostalAddress,
    CNPostalAddressKey, CNSocialProfile, CNSocialProfileKey,
};

fn copy_contacts_constant(symbol_name: &str, context: &str) -> Result<String, ContactsError> {
    let symbol_name = cstring_from_str(symbol_name, context)?;
    let mut error = core::ptr::null_mut();
    let value =
        unsafe { ffi::constants::cn_copy_contacts_constant(symbol_name.as_ptr(), &mut error) };
    if value.is_null() {
        Err(unsafe { ContactsError::from_error_ptr(error, context) })
    } else {
        unsafe { take_required_string(value, context) }
    }
}

impl CNContactKey {
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Identifier => "CNContactIdentifierKey",
            Self::ContactType => "CNContactTypeKey",
            Self::NamePrefix => "CNContactNamePrefixKey",
            Self::GivenName => "CNContactGivenNameKey",
            Self::MiddleName => "CNContactMiddleNameKey",
            Self::FamilyName => "CNContactFamilyNameKey",
            Self::PreviousFamilyName => "CNContactPreviousFamilyNameKey",
            Self::NameSuffix => "CNContactNameSuffixKey",
            Self::Nickname => "CNContactNicknameKey",
            Self::OrganizationName => "CNContactOrganizationNameKey",
            Self::DepartmentName => "CNContactDepartmentNameKey",
            Self::JobTitle => "CNContactJobTitleKey",
            Self::PhoneticGivenName => "CNContactPhoneticGivenNameKey",
            Self::PhoneticMiddleName => "CNContactPhoneticMiddleNameKey",
            Self::PhoneticFamilyName => "CNContactPhoneticFamilyNameKey",
            Self::PhoneticOrganizationName => "CNContactPhoneticOrganizationNameKey",
            Self::Note => "CNContactNoteKey",
            Self::ImageData => "CNContactImageDataKey",
            Self::ThumbnailImageData => "CNContactThumbnailImageDataKey",
            Self::ImageDataAvailable => "CNContactImageDataAvailableKey",
            Self::PhoneNumbers => "CNContactPhoneNumbersKey",
            Self::EmailAddresses => "CNContactEmailAddressesKey",
            Self::PostalAddresses => "CNContactPostalAddressesKey",
            Self::Dates => "CNContactDatesKey",
            Self::UrlAddresses => "CNContactUrlAddressesKey",
            Self::ContactRelations => "CNContactRelationsKey",
            Self::SocialProfiles => "CNContactSocialProfilesKey",
            Self::InstantMessageAddresses => "CNContactInstantMessageAddressesKey",
            Self::Birthday => "CNContactBirthdayKey",
            Self::NonGregorianBirthday => "CNContactNonGregorianBirthdayKey",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }

    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNContact::localized_string_for_key(self)
    }
}

impl CNPostalAddressKey {
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::Street,
            Self::SubLocality,
            Self::City,
            Self::SubAdministrativeArea,
            Self::State,
            Self::PostalCode,
            Self::Country,
            Self::IsoCountryCode,
        ]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Street => "CNPostalAddressStreetKey",
            Self::SubLocality => "CNPostalAddressSubLocalityKey",
            Self::City => "CNPostalAddressCityKey",
            Self::SubAdministrativeArea => "CNPostalAddressSubAdministrativeAreaKey",
            Self::State => "CNPostalAddressStateKey",
            Self::PostalCode => "CNPostalAddressPostalCodeKey",
            Self::Country => "CNPostalAddressCountryKey",
            Self::IsoCountryCode => "CNPostalAddressISOCountryCodeKey",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }

    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNPostalAddress::localized_string_for_key(self)
    }
}

impl CNInstantMessageAddressKey {
    pub fn all_supported() -> &'static [Self] {
        &[Self::Username, Self::Service]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Username => "CNInstantMessageAddressUsernameKey",
            Self::Service => "CNInstantMessageAddressServiceKey",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }

    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNInstantMessageAddress::localized_string_for_key(self)
    }
}

impl CNSocialProfileKey {
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::UrlString,
            Self::Username,
            Self::UserIdentifier,
            Self::Service,
        ]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::UrlString => "CNSocialProfileURLStringKey",
            Self::Username => "CNSocialProfileUsernameKey",
            Self::UserIdentifier => "CNSocialProfileUserIdentifierKey",
            Self::Service => "CNSocialProfileServiceKey",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }

    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNSocialProfile::localized_string_for_key(self)
    }
}

pub fn contact_property_not_fetched_exception_name() -> Result<String, ContactsError> {
    copy_contacts_constant(
        "CNContactPropertyNotFetchedExceptionName",
        "CNContactPropertyNotFetchedExceptionName",
    )
}

pub fn contacts_error_domain() -> Result<String, ContactsError> {
    copy_contacts_constant("CNErrorDomain", "CNErrorDomain")
}

/// Typed wrappers for `CNContainer` property keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContainerKey {
    Identifier,
    Name,
    Type,
}

impl CNContainerKey {
    pub fn all_supported() -> &'static [Self] {
        &[Self::Identifier, Self::Name, Self::Type]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Identifier => "CNContainerIdentifierKey",
            Self::Name => "CNContainerNameKey",
            Self::Type => "CNContainerTypeKey",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for `CNGroup` property keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNGroupKey {
    Identifier,
    Name,
}

impl CNGroupKey {
    pub fn all_supported() -> &'static [Self] {
        &[Self::Identifier, Self::Name]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Identifier => "CNGroupIdentifierKey",
            Self::Name => "CNGroupNameKey",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for generic `CNLabeledValue` labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNLabeledValueLabel {
    Home,
    Work,
    School,
    Other,
}

impl CNLabeledValueLabel {
    pub fn all_supported() -> &'static [Self] {
        &[Self::Home, Self::Work, Self::School, Self::Other]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Home => "CNLabelHome",
            Self::Work => "CNLabelWork",
            Self::School => "CNLabelSchool",
            Self::Other => "CNLabelOther",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts email-address labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNEmailAddressLabel {
    ICloud,
}

impl CNEmailAddressLabel {
    pub fn all_supported() -> &'static [Self] {
        &[Self::ICloud]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::ICloud => "CNLabelEmailiCloud",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts URL-address labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNUrlAddressLabel {
    HomePage,
}

impl CNUrlAddressLabel {
    pub fn all_supported() -> &'static [Self] {
        &[Self::HomePage]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::HomePage => "CNLabelURLAddressHomePage",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts date labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNDateLabel {
    Anniversary,
}

impl CNDateLabel {
    pub fn all_supported() -> &'static [Self] {
        &[Self::Anniversary]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Anniversary => "CNLabelDateAnniversary",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts phone-number labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNPhoneNumberLabel {
    IPhone,
    AppleWatch,
    Mobile,
    Main,
    HomeFax,
    WorkFax,
    OtherFax,
    Pager,
}

impl CNPhoneNumberLabel {
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::IPhone,
            Self::AppleWatch,
            Self::Mobile,
            Self::Main,
            Self::HomeFax,
            Self::WorkFax,
            Self::OtherFax,
            Self::Pager,
        ]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::IPhone => "CNLabelPhoneNumberiPhone",
            Self::AppleWatch => "CNLabelPhoneNumberAppleWatch",
            Self::Mobile => "CNLabelPhoneNumberMobile",
            Self::Main => "CNLabelPhoneNumberMain",
            Self::HomeFax => "CNLabelPhoneNumberHomeFax",
            Self::WorkFax => "CNLabelPhoneNumberWorkFax",
            Self::OtherFax => "CNLabelPhoneNumberOtherFax",
            Self::Pager => "CNLabelPhoneNumberPager",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for `CNInstantMessageAddress` service constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNInstantMessageService {
    Aim,
    Facebook,
    GaduGadu,
    GoogleTalk,
    Icq,
    Jabber,
    Msn,
    Qq,
    Skype,
    Yahoo,
}

impl CNInstantMessageService {
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::Aim,
            Self::Facebook,
            Self::GaduGadu,
            Self::GoogleTalk,
            Self::Icq,
            Self::Jabber,
            Self::Msn,
            Self::Qq,
            Self::Skype,
            Self::Yahoo,
        ]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Aim => "CNInstantMessageServiceAIM",
            Self::Facebook => "CNInstantMessageServiceFacebook",
            Self::GaduGadu => "CNInstantMessageServiceGaduGadu",
            Self::GoogleTalk => "CNInstantMessageServiceGoogleTalk",
            Self::Icq => "CNInstantMessageServiceICQ",
            Self::Jabber => "CNInstantMessageServiceJabber",
            Self::Msn => "CNInstantMessageServiceMSN",
            Self::Qq => "CNInstantMessageServiceQQ",
            Self::Skype => "CNInstantMessageServiceSkype",
            Self::Yahoo => "CNInstantMessageServiceYahoo",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for `CNSocialProfile` service constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNSocialProfileService {
    Facebook,
    Flickr,
    LinkedIn,
    MySpace,
    SinaWeibo,
    TencentWeibo,
    Twitter,
    Yelp,
    GameCenter,
}

impl CNSocialProfileService {
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::Facebook,
            Self::Flickr,
            Self::LinkedIn,
            Self::MySpace,
            Self::SinaWeibo,
            Self::TencentWeibo,
            Self::Twitter,
            Self::Yelp,
            Self::GameCenter,
        ]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Facebook => "CNSocialProfileServiceFacebook",
            Self::Flickr => "CNSocialProfileServiceFlickr",
            Self::LinkedIn => "CNSocialProfileServiceLinkedIn",
            Self::MySpace => "CNSocialProfileServiceMySpace",
            Self::SinaWeibo => "CNSocialProfileServiceSinaWeibo",
            Self::TencentWeibo => "CNSocialProfileServiceTencentWeibo",
            Self::Twitter => "CNSocialProfileServiceTwitter",
            Self::Yelp => "CNSocialProfileServiceYelp",
            Self::GameCenter => "CNSocialProfileServiceGameCenter",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts `NSError.userInfo` keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNErrorUserInfoKey {
    AffectedRecords,
    AffectedRecordIdentifiers,
    ValidationErrors,
    KeyPaths,
}

impl CNErrorUserInfoKey {
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::AffectedRecords,
            Self::AffectedRecordIdentifiers,
            Self::ValidationErrors,
            Self::KeyPaths,
        ]
    }

    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::AffectedRecords => "CNErrorUserInfoAffectedRecordsKey",
            Self::AffectedRecordIdentifiers => "CNErrorUserInfoAffectedRecordIdentifiersKey",
            Self::ValidationErrors => "CNErrorUserInfoValidationErrorsKey",
            Self::KeyPaths => "CNErrorUserInfoKeyPathsKey",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts contact-relation labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactRelationLabel {
    Assistant,
    Manager,
    Colleague,
    Teacher,
    Sibling,
    YoungerSibling,
    ElderSibling,
    Sister,
    YoungerSister,
    YoungestSister,
    ElderSister,
    EldestSister,
    Brother,
    YoungerBrother,
    YoungestBrother,
    ElderBrother,
    EldestBrother,
    Friend,
    MaleFriend,
    FemaleFriend,
    Spouse,
    Wife,
    Husband,
    Partner,
    MalePartner,
    FemalePartner,
    GirlfriendOrBoyfriend,
    Girlfriend,
    Boyfriend,
    Parent,
    Mother,
    Father,
    Child,
    Daughter,
    Son,
    Grandparent,
    Grandmother,
    GrandmotherMothersMother,
    GrandmotherFathersMother,
    Grandfather,
    GrandfatherMothersFather,
    GrandfatherFathersFather,
    GreatGrandparent,
    GreatGrandmother,
    GreatGrandfather,
    Grandchild,
    Granddaughter,
    GranddaughterDaughtersDaughter,
    GranddaughterSonsDaughter,
    Grandson,
    GrandsonDaughtersSon,
    GrandsonSonsSon,
    GreatGrandchild,
    GreatGranddaughter,
    GreatGrandson,
    ParentInLaw,
    MotherInLaw,
    MotherInLawWifesMother,
    MotherInLawHusbandsMother,
    FatherInLaw,
    FatherInLawWifesFather,
    FatherInLawHusbandsFather,
    CoParentInLaw,
    CoMotherInLaw,
    CoFatherInLaw,
    SiblingInLaw,
    YoungerSiblingInLaw,
    ElderSiblingInLaw,
    SisterInLaw,
    YoungerSisterInLaw,
    ElderSisterInLaw,
    SisterInLawSpousesSister,
    SisterInLawWifesSister,
    SisterInLawHusbandsSister,
    SisterInLawBrothersWife,
    SisterInLawYoungerBrothersWife,
    SisterInLawElderBrothersWife,
    BrotherInLaw,
    YoungerBrotherInLaw,
    ElderBrotherInLaw,
    BrotherInLawSpousesBrother,
    BrotherInLawHusbandsBrother,
    BrotherInLawWifesBrother,
    BrotherInLawSistersHusband,
    BrotherInLawYoungerSistersHusband,
    BrotherInLawElderSistersHusband,
    SisterInLawWifesBrothersWife,
    SisterInLawHusbandsBrothersWife,
    BrotherInLawWifesSistersHusband,
    BrotherInLawHusbandsSistersHusband,
    CoSiblingInLaw,
    CoSisterInLaw,
    CoBrotherInLaw,
    ChildInLaw,
    DaughterInLaw,
    SonInLaw,
    Cousin,
    YoungerCousin,
    ElderCousin,
    MaleCousin,
    FemaleCousin,
    CousinParentsSiblingsChild,
    CousinParentsSiblingsSon,
    YoungerCousinParentsSiblingsSon,
    ElderCousinParentsSiblingsSon,
    CousinParentsSiblingsDaughter,
    YoungerCousinParentsSiblingsDaughter,
    ElderCousinParentsSiblingsDaughter,
    CousinMothersSistersDaughter,
    YoungerCousinMothersSistersDaughter,
    ElderCousinMothersSistersDaughter,
    CousinMothersSistersSon,
    YoungerCousinMothersSistersSon,
    ElderCousinMothersSistersSon,
    CousinMothersBrothersDaughter,
    YoungerCousinMothersBrothersDaughter,
    ElderCousinMothersBrothersDaughter,
    CousinMothersBrothersSon,
    YoungerCousinMothersBrothersSon,
    ElderCousinMothersBrothersSon,
    CousinFathersSistersDaughter,
    YoungerCousinFathersSistersDaughter,
    ElderCousinFathersSistersDaughter,
    CousinFathersSistersSon,
    YoungerCousinFathersSistersSon,
    ElderCousinFathersSistersSon,
    CousinFathersBrothersDaughter,
    YoungerCousinFathersBrothersDaughter,
    ElderCousinFathersBrothersDaughter,
    CousinFathersBrothersSon,
    YoungerCousinFathersBrothersSon,
    ElderCousinFathersBrothersSon,
    CousinGrandparentsSiblingsChild,
    CousinGrandparentsSiblingsDaughter,
    CousinGrandparentsSiblingsSon,
    YoungerCousinMothersSiblingsSonOrFathersSistersSon,
    ElderCousinMothersSiblingsSonOrFathersSistersSon,
    YoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter,
    ElderCousinMothersSiblingsDaughterOrFathersSistersDaughter,
    ParentsSibling,
    ParentsYoungerSibling,
    ParentsElderSibling,
    ParentsSiblingMothersSibling,
    ParentsSiblingMothersYoungerSibling,
    ParentsSiblingMothersElderSibling,
    ParentsSiblingFathersSibling,
    ParentsSiblingFathersYoungerSibling,
    ParentsSiblingFathersElderSibling,
    Aunt,
    AuntParentsSister,
    AuntParentsYoungerSister,
    AuntParentsElderSister,
    AuntFathersSister,
    AuntFathersYoungerSister,
    AuntFathersElderSister,
    AuntFathersBrothersWife,
    AuntFathersYoungerBrothersWife,
    AuntFathersElderBrothersWife,
    AuntMothersSister,
    AuntMothersYoungerSister,
    AuntMothersElderSister,
    AuntMothersBrothersWife,
    Grandaunt,
    Uncle,
    UncleParentsBrother,
    UncleParentsYoungerBrother,
    UncleParentsElderBrother,
    UncleMothersBrother,
    UncleMothersYoungerBrother,
    UncleMothersElderBrother,
    UncleMothersSistersHusband,
    UncleFathersBrother,
    UncleFathersYoungerBrother,
    UncleFathersElderBrother,
    UncleFathersSistersHusband,
    UncleFathersYoungerSistersHusband,
    UncleFathersElderSistersHusband,
    Granduncle,
    SiblingsChild,
    Niece,
    NieceSistersDaughter,
    NieceBrothersDaughter,
    NieceSistersDaughterOrWifesSiblingsDaughter,
    NieceBrothersDaughterOrHusbandsSiblingsDaughter,
    Nephew,
    NephewSistersSon,
    NephewBrothersSon,
    NephewBrothersSonOrHusbandsSiblingsSon,
    NephewSistersSonOrWifesSiblingsSon,
    Grandniece,
    GrandnieceSistersGranddaughter,
    GrandnieceBrothersGranddaughter,
    Grandnephew,
    GrandnephewSistersGrandson,
    GrandnephewBrothersGrandson,
    Stepparent,
    Stepmother,
    Stepfather,
    Stepchild,
    Stepdaughter,
    Stepson,
    Stepsister,
    Stepbrother,
    MotherInLawOrStepmother,
    FatherInLawOrStepfather,
    DaughterInLawOrStepdaughter,
    SonInLawOrStepson,
    CousinOrSiblingsChild,
    NieceOrCousin,
    NephewOrCousin,
    GrandchildOrSiblingsChild,
    GranddaughterOrNiece,
    GrandsonOrNephew,
    GreatGrandchildOrSiblingsGrandchild,
    DaughterInLawOrSisterInLaw,
    SonInLawOrBrotherInLaw,
}

impl CNContactRelationLabel {
    #[allow(clippy::too_many_lines)]
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::Assistant,
            Self::Manager,
            Self::Colleague,
            Self::Teacher,
            Self::Sibling,
            Self::YoungerSibling,
            Self::ElderSibling,
            Self::Sister,
            Self::YoungerSister,
            Self::YoungestSister,
            Self::ElderSister,
            Self::EldestSister,
            Self::Brother,
            Self::YoungerBrother,
            Self::YoungestBrother,
            Self::ElderBrother,
            Self::EldestBrother,
            Self::Friend,
            Self::MaleFriend,
            Self::FemaleFriend,
            Self::Spouse,
            Self::Wife,
            Self::Husband,
            Self::Partner,
            Self::MalePartner,
            Self::FemalePartner,
            Self::GirlfriendOrBoyfriend,
            Self::Girlfriend,
            Self::Boyfriend,
            Self::Parent,
            Self::Mother,
            Self::Father,
            Self::Child,
            Self::Daughter,
            Self::Son,
            Self::Grandparent,
            Self::Grandmother,
            Self::GrandmotherMothersMother,
            Self::GrandmotherFathersMother,
            Self::Grandfather,
            Self::GrandfatherMothersFather,
            Self::GrandfatherFathersFather,
            Self::GreatGrandparent,
            Self::GreatGrandmother,
            Self::GreatGrandfather,
            Self::Grandchild,
            Self::Granddaughter,
            Self::GranddaughterDaughtersDaughter,
            Self::GranddaughterSonsDaughter,
            Self::Grandson,
            Self::GrandsonDaughtersSon,
            Self::GrandsonSonsSon,
            Self::GreatGrandchild,
            Self::GreatGranddaughter,
            Self::GreatGrandson,
            Self::ParentInLaw,
            Self::MotherInLaw,
            Self::MotherInLawWifesMother,
            Self::MotherInLawHusbandsMother,
            Self::FatherInLaw,
            Self::FatherInLawWifesFather,
            Self::FatherInLawHusbandsFather,
            Self::CoParentInLaw,
            Self::CoMotherInLaw,
            Self::CoFatherInLaw,
            Self::SiblingInLaw,
            Self::YoungerSiblingInLaw,
            Self::ElderSiblingInLaw,
            Self::SisterInLaw,
            Self::YoungerSisterInLaw,
            Self::ElderSisterInLaw,
            Self::SisterInLawSpousesSister,
            Self::SisterInLawWifesSister,
            Self::SisterInLawHusbandsSister,
            Self::SisterInLawBrothersWife,
            Self::SisterInLawYoungerBrothersWife,
            Self::SisterInLawElderBrothersWife,
            Self::BrotherInLaw,
            Self::YoungerBrotherInLaw,
            Self::ElderBrotherInLaw,
            Self::BrotherInLawSpousesBrother,
            Self::BrotherInLawHusbandsBrother,
            Self::BrotherInLawWifesBrother,
            Self::BrotherInLawSistersHusband,
            Self::BrotherInLawYoungerSistersHusband,
            Self::BrotherInLawElderSistersHusband,
            Self::SisterInLawWifesBrothersWife,
            Self::SisterInLawHusbandsBrothersWife,
            Self::BrotherInLawWifesSistersHusband,
            Self::BrotherInLawHusbandsSistersHusband,
            Self::CoSiblingInLaw,
            Self::CoSisterInLaw,
            Self::CoBrotherInLaw,
            Self::ChildInLaw,
            Self::DaughterInLaw,
            Self::SonInLaw,
            Self::Cousin,
            Self::YoungerCousin,
            Self::ElderCousin,
            Self::MaleCousin,
            Self::FemaleCousin,
            Self::CousinParentsSiblingsChild,
            Self::CousinParentsSiblingsSon,
            Self::YoungerCousinParentsSiblingsSon,
            Self::ElderCousinParentsSiblingsSon,
            Self::CousinParentsSiblingsDaughter,
            Self::YoungerCousinParentsSiblingsDaughter,
            Self::ElderCousinParentsSiblingsDaughter,
            Self::CousinMothersSistersDaughter,
            Self::YoungerCousinMothersSistersDaughter,
            Self::ElderCousinMothersSistersDaughter,
            Self::CousinMothersSistersSon,
            Self::YoungerCousinMothersSistersSon,
            Self::ElderCousinMothersSistersSon,
            Self::CousinMothersBrothersDaughter,
            Self::YoungerCousinMothersBrothersDaughter,
            Self::ElderCousinMothersBrothersDaughter,
            Self::CousinMothersBrothersSon,
            Self::YoungerCousinMothersBrothersSon,
            Self::ElderCousinMothersBrothersSon,
            Self::CousinFathersSistersDaughter,
            Self::YoungerCousinFathersSistersDaughter,
            Self::ElderCousinFathersSistersDaughter,
            Self::CousinFathersSistersSon,
            Self::YoungerCousinFathersSistersSon,
            Self::ElderCousinFathersSistersSon,
            Self::CousinFathersBrothersDaughter,
            Self::YoungerCousinFathersBrothersDaughter,
            Self::ElderCousinFathersBrothersDaughter,
            Self::CousinFathersBrothersSon,
            Self::YoungerCousinFathersBrothersSon,
            Self::ElderCousinFathersBrothersSon,
            Self::CousinGrandparentsSiblingsChild,
            Self::CousinGrandparentsSiblingsDaughter,
            Self::CousinGrandparentsSiblingsSon,
            Self::YoungerCousinMothersSiblingsSonOrFathersSistersSon,
            Self::ElderCousinMothersSiblingsSonOrFathersSistersSon,
            Self::YoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter,
            Self::ElderCousinMothersSiblingsDaughterOrFathersSistersDaughter,
            Self::ParentsSibling,
            Self::ParentsYoungerSibling,
            Self::ParentsElderSibling,
            Self::ParentsSiblingMothersSibling,
            Self::ParentsSiblingMothersYoungerSibling,
            Self::ParentsSiblingMothersElderSibling,
            Self::ParentsSiblingFathersSibling,
            Self::ParentsSiblingFathersYoungerSibling,
            Self::ParentsSiblingFathersElderSibling,
            Self::Aunt,
            Self::AuntParentsSister,
            Self::AuntParentsYoungerSister,
            Self::AuntParentsElderSister,
            Self::AuntFathersSister,
            Self::AuntFathersYoungerSister,
            Self::AuntFathersElderSister,
            Self::AuntFathersBrothersWife,
            Self::AuntFathersYoungerBrothersWife,
            Self::AuntFathersElderBrothersWife,
            Self::AuntMothersSister,
            Self::AuntMothersYoungerSister,
            Self::AuntMothersElderSister,
            Self::AuntMothersBrothersWife,
            Self::Grandaunt,
            Self::Uncle,
            Self::UncleParentsBrother,
            Self::UncleParentsYoungerBrother,
            Self::UncleParentsElderBrother,
            Self::UncleMothersBrother,
            Self::UncleMothersYoungerBrother,
            Self::UncleMothersElderBrother,
            Self::UncleMothersSistersHusband,
            Self::UncleFathersBrother,
            Self::UncleFathersYoungerBrother,
            Self::UncleFathersElderBrother,
            Self::UncleFathersSistersHusband,
            Self::UncleFathersYoungerSistersHusband,
            Self::UncleFathersElderSistersHusband,
            Self::Granduncle,
            Self::SiblingsChild,
            Self::Niece,
            Self::NieceSistersDaughter,
            Self::NieceBrothersDaughter,
            Self::NieceSistersDaughterOrWifesSiblingsDaughter,
            Self::NieceBrothersDaughterOrHusbandsSiblingsDaughter,
            Self::Nephew,
            Self::NephewSistersSon,
            Self::NephewBrothersSon,
            Self::NephewBrothersSonOrHusbandsSiblingsSon,
            Self::NephewSistersSonOrWifesSiblingsSon,
            Self::Grandniece,
            Self::GrandnieceSistersGranddaughter,
            Self::GrandnieceBrothersGranddaughter,
            Self::Grandnephew,
            Self::GrandnephewSistersGrandson,
            Self::GrandnephewBrothersGrandson,
            Self::Stepparent,
            Self::Stepmother,
            Self::Stepfather,
            Self::Stepchild,
            Self::Stepdaughter,
            Self::Stepson,
            Self::Stepsister,
            Self::Stepbrother,
            Self::MotherInLawOrStepmother,
            Self::FatherInLawOrStepfather,
            Self::DaughterInLawOrStepdaughter,
            Self::SonInLawOrStepson,
            Self::CousinOrSiblingsChild,
            Self::NieceOrCousin,
            Self::NephewOrCousin,
            Self::GrandchildOrSiblingsChild,
            Self::GranddaughterOrNiece,
            Self::GrandsonOrNephew,
            Self::GreatGrandchildOrSiblingsGrandchild,
            Self::DaughterInLawOrSisterInLaw,
            Self::SonInLawOrBrotherInLaw,
        ]
    }

    #[allow(clippy::too_many_lines)]
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Assistant => "CNLabelContactRelationAssistant",
            Self::Manager => "CNLabelContactRelationManager",
            Self::Colleague => "CNLabelContactRelationColleague",
            Self::Teacher => "CNLabelContactRelationTeacher",
            Self::Sibling => "CNLabelContactRelationSibling",
            Self::YoungerSibling => "CNLabelContactRelationYoungerSibling",
            Self::ElderSibling => "CNLabelContactRelationElderSibling",
            Self::Sister => "CNLabelContactRelationSister",
            Self::YoungerSister => "CNLabelContactRelationYoungerSister",
            Self::YoungestSister => "CNLabelContactRelationYoungestSister",
            Self::ElderSister => "CNLabelContactRelationElderSister",
            Self::EldestSister => "CNLabelContactRelationEldestSister",
            Self::Brother => "CNLabelContactRelationBrother",
            Self::YoungerBrother => "CNLabelContactRelationYoungerBrother",
            Self::YoungestBrother => "CNLabelContactRelationYoungestBrother",
            Self::ElderBrother => "CNLabelContactRelationElderBrother",
            Self::EldestBrother => "CNLabelContactRelationEldestBrother",
            Self::Friend => "CNLabelContactRelationFriend",
            Self::MaleFriend => "CNLabelContactRelationMaleFriend",
            Self::FemaleFriend => "CNLabelContactRelationFemaleFriend",
            Self::Spouse => "CNLabelContactRelationSpouse",
            Self::Wife => "CNLabelContactRelationWife",
            Self::Husband => "CNLabelContactRelationHusband",
            Self::Partner => "CNLabelContactRelationPartner",
            Self::MalePartner => "CNLabelContactRelationMalePartner",
            Self::FemalePartner => "CNLabelContactRelationFemalePartner",
            Self::GirlfriendOrBoyfriend => "CNLabelContactRelationGirlfriendOrBoyfriend",
            Self::Girlfriend => "CNLabelContactRelationGirlfriend",
            Self::Boyfriend => "CNLabelContactRelationBoyfriend",
            Self::Parent => "CNLabelContactRelationParent",
            Self::Mother => "CNLabelContactRelationMother",
            Self::Father => "CNLabelContactRelationFather",
            Self::Child => "CNLabelContactRelationChild",
            Self::Daughter => "CNLabelContactRelationDaughter",
            Self::Son => "CNLabelContactRelationSon",
            Self::Grandparent => "CNLabelContactRelationGrandparent",
            Self::Grandmother => "CNLabelContactRelationGrandmother",
            Self::GrandmotherMothersMother => "CNLabelContactRelationGrandmotherMothersMother",
            Self::GrandmotherFathersMother => "CNLabelContactRelationGrandmotherFathersMother",
            Self::Grandfather => "CNLabelContactRelationGrandfather",
            Self::GrandfatherMothersFather => "CNLabelContactRelationGrandfatherMothersFather",
            Self::GrandfatherFathersFather => "CNLabelContactRelationGrandfatherFathersFather",
            Self::GreatGrandparent => "CNLabelContactRelationGreatGrandparent",
            Self::GreatGrandmother => "CNLabelContactRelationGreatGrandmother",
            Self::GreatGrandfather => "CNLabelContactRelationGreatGrandfather",
            Self::Grandchild => "CNLabelContactRelationGrandchild",
            Self::Granddaughter => "CNLabelContactRelationGranddaughter",
            Self::GranddaughterDaughtersDaughter => {
                "CNLabelContactRelationGranddaughterDaughtersDaughter"
            }
            Self::GranddaughterSonsDaughter => "CNLabelContactRelationGranddaughterSonsDaughter",
            Self::Grandson => "CNLabelContactRelationGrandson",
            Self::GrandsonDaughtersSon => "CNLabelContactRelationGrandsonDaughtersSon",
            Self::GrandsonSonsSon => "CNLabelContactRelationGrandsonSonsSon",
            Self::GreatGrandchild => "CNLabelContactRelationGreatGrandchild",
            Self::GreatGranddaughter => "CNLabelContactRelationGreatGranddaughter",
            Self::GreatGrandson => "CNLabelContactRelationGreatGrandson",
            Self::ParentInLaw => "CNLabelContactRelationParentInLaw",
            Self::MotherInLaw => "CNLabelContactRelationMotherInLaw",
            Self::MotherInLawWifesMother => "CNLabelContactRelationMotherInLawWifesMother",
            Self::MotherInLawHusbandsMother => "CNLabelContactRelationMotherInLawHusbandsMother",
            Self::FatherInLaw => "CNLabelContactRelationFatherInLaw",
            Self::FatherInLawWifesFather => "CNLabelContactRelationFatherInLawWifesFather",
            Self::FatherInLawHusbandsFather => "CNLabelContactRelationFatherInLawHusbandsFather",
            Self::CoParentInLaw => "CNLabelContactRelationCoParentInLaw",
            Self::CoMotherInLaw => "CNLabelContactRelationCoMotherInLaw",
            Self::CoFatherInLaw => "CNLabelContactRelationCoFatherInLaw",
            Self::SiblingInLaw => "CNLabelContactRelationSiblingInLaw",
            Self::YoungerSiblingInLaw => "CNLabelContactRelationYoungerSiblingInLaw",
            Self::ElderSiblingInLaw => "CNLabelContactRelationElderSiblingInLaw",
            Self::SisterInLaw => "CNLabelContactRelationSisterInLaw",
            Self::YoungerSisterInLaw => "CNLabelContactRelationYoungerSisterInLaw",
            Self::ElderSisterInLaw => "CNLabelContactRelationElderSisterInLaw",
            Self::SisterInLawSpousesSister => "CNLabelContactRelationSisterInLawSpousesSister",
            Self::SisterInLawWifesSister => "CNLabelContactRelationSisterInLawWifesSister",
            Self::SisterInLawHusbandsSister => "CNLabelContactRelationSisterInLawHusbandsSister",
            Self::SisterInLawBrothersWife => "CNLabelContactRelationSisterInLawBrothersWife",
            Self::SisterInLawYoungerBrothersWife => {
                "CNLabelContactRelationSisterInLawYoungerBrothersWife"
            }
            Self::SisterInLawElderBrothersWife => {
                "CNLabelContactRelationSisterInLawElderBrothersWife"
            }
            Self::BrotherInLaw => "CNLabelContactRelationBrotherInLaw",
            Self::YoungerBrotherInLaw => "CNLabelContactRelationYoungerBrotherInLaw",
            Self::ElderBrotherInLaw => "CNLabelContactRelationElderBrotherInLaw",
            Self::BrotherInLawSpousesBrother => "CNLabelContactRelationBrotherInLawSpousesBrother",
            Self::BrotherInLawHusbandsBrother => {
                "CNLabelContactRelationBrotherInLawHusbandsBrother"
            }
            Self::BrotherInLawWifesBrother => "CNLabelContactRelationBrotherInLawWifesBrother",
            Self::BrotherInLawSistersHusband => "CNLabelContactRelationBrotherInLawSistersHusband",
            Self::BrotherInLawYoungerSistersHusband => {
                "CNLabelContactRelationBrotherInLawYoungerSistersHusband"
            }
            Self::BrotherInLawElderSistersHusband => {
                "CNLabelContactRelationBrotherInLawElderSistersHusband"
            }
            Self::SisterInLawWifesBrothersWife => {
                "CNLabelContactRelationSisterInLawWifesBrothersWife"
            }
            Self::SisterInLawHusbandsBrothersWife => {
                "CNLabelContactRelationSisterInLawHusbandsBrothersWife"
            }
            Self::BrotherInLawWifesSistersHusband => {
                "CNLabelContactRelationBrotherInLawWifesSistersHusband"
            }
            Self::BrotherInLawHusbandsSistersHusband => {
                "CNLabelContactRelationBrotherInLawHusbandsSistersHusband"
            }
            Self::CoSiblingInLaw => "CNLabelContactRelationCoSiblingInLaw",
            Self::CoSisterInLaw => "CNLabelContactRelationCoSisterInLaw",
            Self::CoBrotherInLaw => "CNLabelContactRelationCoBrotherInLaw",
            Self::ChildInLaw => "CNLabelContactRelationChildInLaw",
            Self::DaughterInLaw => "CNLabelContactRelationDaughterInLaw",
            Self::SonInLaw => "CNLabelContactRelationSonInLaw",
            Self::Cousin => "CNLabelContactRelationCousin",
            Self::YoungerCousin => "CNLabelContactRelationYoungerCousin",
            Self::ElderCousin => "CNLabelContactRelationElderCousin",
            Self::MaleCousin => "CNLabelContactRelationMaleCousin",
            Self::FemaleCousin => "CNLabelContactRelationFemaleCousin",
            Self::CousinParentsSiblingsChild => "CNLabelContactRelationCousinParentsSiblingsChild",
            Self::CousinParentsSiblingsSon => "CNLabelContactRelationCousinParentsSiblingsSon",
            Self::YoungerCousinParentsSiblingsSon => {
                "CNLabelContactRelationYoungerCousinParentsSiblingsSon"
            }
            Self::ElderCousinParentsSiblingsSon => {
                "CNLabelContactRelationElderCousinParentsSiblingsSon"
            }
            Self::CousinParentsSiblingsDaughter => {
                "CNLabelContactRelationCousinParentsSiblingsDaughter"
            }
            Self::YoungerCousinParentsSiblingsDaughter => {
                "CNLabelContactRelationYoungerCousinParentsSiblingsDaughter"
            }
            Self::ElderCousinParentsSiblingsDaughter => {
                "CNLabelContactRelationElderCousinParentsSiblingsDaughter"
            }
            Self::CousinMothersSistersDaughter => {
                "CNLabelContactRelationCousinMothersSistersDaughter"
            }
            Self::YoungerCousinMothersSistersDaughter => {
                "CNLabelContactRelationYoungerCousinMothersSistersDaughter"
            }
            Self::ElderCousinMothersSistersDaughter => {
                "CNLabelContactRelationElderCousinMothersSistersDaughter"
            }
            Self::CousinMothersSistersSon => "CNLabelContactRelationCousinMothersSistersSon",
            Self::YoungerCousinMothersSistersSon => {
                "CNLabelContactRelationYoungerCousinMothersSistersSon"
            }
            Self::ElderCousinMothersSistersSon => {
                "CNLabelContactRelationElderCousinMothersSistersSon"
            }
            Self::CousinMothersBrothersDaughter => {
                "CNLabelContactRelationCousinMothersBrothersDaughter"
            }
            Self::YoungerCousinMothersBrothersDaughter => {
                "CNLabelContactRelationYoungerCousinMothersBrothersDaughter"
            }
            Self::ElderCousinMothersBrothersDaughter => {
                "CNLabelContactRelationElderCousinMothersBrothersDaughter"
            }
            Self::CousinMothersBrothersSon => "CNLabelContactRelationCousinMothersBrothersSon",
            Self::YoungerCousinMothersBrothersSon => {
                "CNLabelContactRelationYoungerCousinMothersBrothersSon"
            }
            Self::ElderCousinMothersBrothersSon => {
                "CNLabelContactRelationElderCousinMothersBrothersSon"
            }
            Self::CousinFathersSistersDaughter => {
                "CNLabelContactRelationCousinFathersSistersDaughter"
            }
            Self::YoungerCousinFathersSistersDaughter => {
                "CNLabelContactRelationYoungerCousinFathersSistersDaughter"
            }
            Self::ElderCousinFathersSistersDaughter => {
                "CNLabelContactRelationElderCousinFathersSistersDaughter"
            }
            Self::CousinFathersSistersSon => "CNLabelContactRelationCousinFathersSistersSon",
            Self::YoungerCousinFathersSistersSon => {
                "CNLabelContactRelationYoungerCousinFathersSistersSon"
            }
            Self::ElderCousinFathersSistersSon => {
                "CNLabelContactRelationElderCousinFathersSistersSon"
            }
            Self::CousinFathersBrothersDaughter => {
                "CNLabelContactRelationCousinFathersBrothersDaughter"
            }
            Self::YoungerCousinFathersBrothersDaughter => {
                "CNLabelContactRelationYoungerCousinFathersBrothersDaughter"
            }
            Self::ElderCousinFathersBrothersDaughter => {
                "CNLabelContactRelationElderCousinFathersBrothersDaughter"
            }
            Self::CousinFathersBrothersSon => "CNLabelContactRelationCousinFathersBrothersSon",
            Self::YoungerCousinFathersBrothersSon => {
                "CNLabelContactRelationYoungerCousinFathersBrothersSon"
            }
            Self::ElderCousinFathersBrothersSon => {
                "CNLabelContactRelationElderCousinFathersBrothersSon"
            }
            Self::CousinGrandparentsSiblingsChild => {
                "CNLabelContactRelationCousinGrandparentsSiblingsChild"
            }
            Self::CousinGrandparentsSiblingsDaughter => {
                "CNLabelContactRelationCousinGrandparentsSiblingsDaughter"
            }
            Self::CousinGrandparentsSiblingsSon => {
                "CNLabelContactRelationCousinGrandparentsSiblingsSon"
            }
            Self::YoungerCousinMothersSiblingsSonOrFathersSistersSon => {
                "CNLabelContactRelationYoungerCousinMothersSiblingsSonOrFathersSistersSon"
            }
            Self::ElderCousinMothersSiblingsSonOrFathersSistersSon => {
                "CNLabelContactRelationElderCousinMothersSiblingsSonOrFathersSistersSon"
            }
            Self::YoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter => {
                "CNLabelContactRelationYoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter"
            }
            Self::ElderCousinMothersSiblingsDaughterOrFathersSistersDaughter => {
                "CNLabelContactRelationElderCousinMothersSiblingsDaughterOrFathersSistersDaughter"
            }
            Self::ParentsSibling => "CNLabelContactRelationParentsSibling",
            Self::ParentsYoungerSibling => "CNLabelContactRelationParentsYoungerSibling",
            Self::ParentsElderSibling => "CNLabelContactRelationParentsElderSibling",
            Self::ParentsSiblingMothersSibling => {
                "CNLabelContactRelationParentsSiblingMothersSibling"
            }
            Self::ParentsSiblingMothersYoungerSibling => {
                "CNLabelContactRelationParentsSiblingMothersYoungerSibling"
            }
            Self::ParentsSiblingMothersElderSibling => {
                "CNLabelContactRelationParentsSiblingMothersElderSibling"
            }
            Self::ParentsSiblingFathersSibling => {
                "CNLabelContactRelationParentsSiblingFathersSibling"
            }
            Self::ParentsSiblingFathersYoungerSibling => {
                "CNLabelContactRelationParentsSiblingFathersYoungerSibling"
            }
            Self::ParentsSiblingFathersElderSibling => {
                "CNLabelContactRelationParentsSiblingFathersElderSibling"
            }
            Self::Aunt => "CNLabelContactRelationAunt",
            Self::AuntParentsSister => "CNLabelContactRelationAuntParentsSister",
            Self::AuntParentsYoungerSister => "CNLabelContactRelationAuntParentsYoungerSister",
            Self::AuntParentsElderSister => "CNLabelContactRelationAuntParentsElderSister",
            Self::AuntFathersSister => "CNLabelContactRelationAuntFathersSister",
            Self::AuntFathersYoungerSister => "CNLabelContactRelationAuntFathersYoungerSister",
            Self::AuntFathersElderSister => "CNLabelContactRelationAuntFathersElderSister",
            Self::AuntFathersBrothersWife => "CNLabelContactRelationAuntFathersBrothersWife",
            Self::AuntFathersYoungerBrothersWife => {
                "CNLabelContactRelationAuntFathersYoungerBrothersWife"
            }
            Self::AuntFathersElderBrothersWife => {
                "CNLabelContactRelationAuntFathersElderBrothersWife"
            }
            Self::AuntMothersSister => "CNLabelContactRelationAuntMothersSister",
            Self::AuntMothersYoungerSister => "CNLabelContactRelationAuntMothersYoungerSister",
            Self::AuntMothersElderSister => "CNLabelContactRelationAuntMothersElderSister",
            Self::AuntMothersBrothersWife => "CNLabelContactRelationAuntMothersBrothersWife",
            Self::Grandaunt => "CNLabelContactRelationGrandaunt",
            Self::Uncle => "CNLabelContactRelationUncle",
            Self::UncleParentsBrother => "CNLabelContactRelationUncleParentsBrother",
            Self::UncleParentsYoungerBrother => "CNLabelContactRelationUncleParentsYoungerBrother",
            Self::UncleParentsElderBrother => "CNLabelContactRelationUncleParentsElderBrother",
            Self::UncleMothersBrother => "CNLabelContactRelationUncleMothersBrother",
            Self::UncleMothersYoungerBrother => "CNLabelContactRelationUncleMothersYoungerBrother",
            Self::UncleMothersElderBrother => "CNLabelContactRelationUncleMothersElderBrother",
            Self::UncleMothersSistersHusband => "CNLabelContactRelationUncleMothersSistersHusband",
            Self::UncleFathersBrother => "CNLabelContactRelationUncleFathersBrother",
            Self::UncleFathersYoungerBrother => "CNLabelContactRelationUncleFathersYoungerBrother",
            Self::UncleFathersElderBrother => "CNLabelContactRelationUncleFathersElderBrother",
            Self::UncleFathersSistersHusband => "CNLabelContactRelationUncleFathersSistersHusband",
            Self::UncleFathersYoungerSistersHusband => {
                "CNLabelContactRelationUncleFathersYoungerSistersHusband"
            }
            Self::UncleFathersElderSistersHusband => {
                "CNLabelContactRelationUncleFathersElderSistersHusband"
            }
            Self::Granduncle => "CNLabelContactRelationGranduncle",
            Self::SiblingsChild => "CNLabelContactRelationSiblingsChild",
            Self::Niece => "CNLabelContactRelationNiece",
            Self::NieceSistersDaughter => "CNLabelContactRelationNieceSistersDaughter",
            Self::NieceBrothersDaughter => "CNLabelContactRelationNieceBrothersDaughter",
            Self::NieceSistersDaughterOrWifesSiblingsDaughter => {
                "CNLabelContactRelationNieceSistersDaughterOrWifesSiblingsDaughter"
            }
            Self::NieceBrothersDaughterOrHusbandsSiblingsDaughter => {
                "CNLabelContactRelationNieceBrothersDaughterOrHusbandsSiblingsDaughter"
            }
            Self::Nephew => "CNLabelContactRelationNephew",
            Self::NephewSistersSon => "CNLabelContactRelationNephewSistersSon",
            Self::NephewBrothersSon => "CNLabelContactRelationNephewBrothersSon",
            Self::NephewBrothersSonOrHusbandsSiblingsSon => {
                "CNLabelContactRelationNephewBrothersSonOrHusbandsSiblingsSon"
            }
            Self::NephewSistersSonOrWifesSiblingsSon => {
                "CNLabelContactRelationNephewSistersSonOrWifesSiblingsSon"
            }
            Self::Grandniece => "CNLabelContactRelationGrandniece",
            Self::GrandnieceSistersGranddaughter => {
                "CNLabelContactRelationGrandnieceSistersGranddaughter"
            }
            Self::GrandnieceBrothersGranddaughter => {
                "CNLabelContactRelationGrandnieceBrothersGranddaughter"
            }
            Self::Grandnephew => "CNLabelContactRelationGrandnephew",
            Self::GrandnephewSistersGrandson => "CNLabelContactRelationGrandnephewSistersGrandson",
            Self::GrandnephewBrothersGrandson => {
                "CNLabelContactRelationGrandnephewBrothersGrandson"
            }
            Self::Stepparent => "CNLabelContactRelationStepparent",
            Self::Stepmother => "CNLabelContactRelationStepmother",
            Self::Stepfather => "CNLabelContactRelationStepfather",
            Self::Stepchild => "CNLabelContactRelationStepchild",
            Self::Stepdaughter => "CNLabelContactRelationStepdaughter",
            Self::Stepson => "CNLabelContactRelationStepson",
            Self::Stepsister => "CNLabelContactRelationStepsister",
            Self::Stepbrother => "CNLabelContactRelationStepbrother",
            Self::MotherInLawOrStepmother => "CNLabelContactRelationMotherInLawOrStepmother",
            Self::FatherInLawOrStepfather => "CNLabelContactRelationFatherInLawOrStepfather",
            Self::DaughterInLawOrStepdaughter => {
                "CNLabelContactRelationDaughterInLawOrStepdaughter"
            }
            Self::SonInLawOrStepson => "CNLabelContactRelationSonInLawOrStepson",
            Self::CousinOrSiblingsChild => "CNLabelContactRelationCousinOrSiblingsChild",
            Self::NieceOrCousin => "CNLabelContactRelationNieceOrCousin",
            Self::NephewOrCousin => "CNLabelContactRelationNephewOrCousin",
            Self::GrandchildOrSiblingsChild => "CNLabelContactRelationGrandchildOrSiblingsChild",
            Self::GranddaughterOrNiece => "CNLabelContactRelationGranddaughterOrNiece",
            Self::GrandsonOrNephew => "CNLabelContactRelationGrandsonOrNephew",
            Self::GreatGrandchildOrSiblingsGrandchild => {
                "CNLabelContactRelationGreatGrandchildOrSiblingsGrandchild"
            }
            Self::DaughterInLawOrSisterInLaw => "CNLabelContactRelationDaughterInLawOrSisterInLaw",
            Self::SonInLawOrBrotherInLaw => "CNLabelContactRelationSonInLawOrBrotherInLaw",
        }
    }

    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

impl CNLabeledValueLabel {
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNEmailAddressLabel {
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNUrlAddressLabel {
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNDateLabel {
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNPhoneNumberLabel {
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNInstantMessageService {
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNInstantMessageAddress::localized_string_for_service(&self.value()?)
    }
}

impl CNSocialProfileService {
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNSocialProfile::localized_string_for_service(&self.value()?)
    }
}

impl CNContactRelationLabel {
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}
