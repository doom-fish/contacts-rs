//! Typed wrappers around Contacts framework constants.

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
    /// Returns the framework symbol name.
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

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }

    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNContact::localized_string_for_key(self)
    }
}

impl CNPostalAddressKey {
    /// Returns all supported values.
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

    /// Returns the framework symbol name.
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

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }

    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNPostalAddress::localized_string_for_key(self)
    }
}

impl CNInstantMessageAddressKey {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[Self::Username, Self::Service]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Username => "CNInstantMessageAddressUsernameKey",
            Self::Service => "CNInstantMessageAddressServiceKey",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }

    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNInstantMessageAddress::localized_string_for_key(self)
    }
}

impl CNSocialProfileKey {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::UrlString,
            Self::Username,
            Self::UserIdentifier,
            Self::Service,
        ]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::UrlString => "CNSocialProfileURLStringKey",
            Self::Username => "CNSocialProfileUsernameKey",
            Self::UserIdentifier => "CNSocialProfileUserIdentifierKey",
            Self::Service => "CNSocialProfileServiceKey",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }

    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNSocialProfile::localized_string_for_key(self)
    }
}

/// Returns the property-not-fetched exception name.
pub fn contact_property_not_fetched_exception_name() -> Result<String, ContactsError> {
    copy_contacts_constant(
        "CNContactPropertyNotFetchedExceptionName",
        "CNContactPropertyNotFetchedExceptionName",
    )
}

/// Returns the Contacts error-domain string.
pub fn contacts_error_domain() -> Result<String, ContactsError> {
    copy_contacts_constant("CNErrorDomain", "CNErrorDomain")
}

/// Typed wrappers for `CNContainer` property keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContainerKey {
    /// The identifier key.
    Identifier,
    /// The name key.
    Name,
    /// The type key.
    Type,
}

impl CNContainerKey {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[Self::Identifier, Self::Name, Self::Type]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Identifier => "CNContainerIdentifierKey",
            Self::Name => "CNContainerNameKey",
            Self::Type => "CNContainerTypeKey",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for `CNGroup` property keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNGroupKey {
    /// The identifier key.
    Identifier,
    /// The name key.
    Name,
}

impl CNGroupKey {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[Self::Identifier, Self::Name]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Identifier => "CNGroupIdentifierKey",
            Self::Name => "CNGroupNameKey",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for generic `CNLabeledValue` labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNLabeledValueLabel {
    /// The home label.
    Home,
    /// The work label.
    Work,
    /// The school label.
    School,
    /// The other label.
    Other,
}

impl CNLabeledValueLabel {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[Self::Home, Self::Work, Self::School, Self::Other]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Home => "CNLabelHome",
            Self::Work => "CNLabelWork",
            Self::School => "CNLabelSchool",
            Self::Other => "CNLabelOther",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts email-address labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNEmailAddressLabel {
    /// The iCloud label.
    ICloud,
}

impl CNEmailAddressLabel {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[Self::ICloud]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::ICloud => "CNLabelEmailiCloud",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts URL-address labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNUrlAddressLabel {
    /// The home-page label.
    HomePage,
}

impl CNUrlAddressLabel {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[Self::HomePage]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::HomePage => "CNLabelURLAddressHomePage",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts date labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNDateLabel {
    /// The anniversary label.
    Anniversary,
}

impl CNDateLabel {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[Self::Anniversary]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::Anniversary => "CNLabelDateAnniversary",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts phone-number labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNPhoneNumberLabel {
    /// The i phone label.
    IPhone,
    /// The apple watch label.
    AppleWatch,
    /// The mobile label.
    Mobile,
    /// The main label.
    Main,
    /// The home-fax label.
    HomeFax,
    /// The work fax label.
    WorkFax,
    /// The other fax label.
    OtherFax,
    /// The pager label.
    Pager,
}

impl CNPhoneNumberLabel {
    /// Returns all supported values.
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

    /// Returns the framework symbol name.
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

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for `CNInstantMessageAddress` service constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNInstantMessageService {
    /// The aim service.
    Aim,
    /// The facebook service.
    Facebook,
    /// The gadu gadu service.
    GaduGadu,
    /// The google talk service.
    GoogleTalk,
    /// The icq service.
    Icq,
    /// The jabber service.
    Jabber,
    /// The msn service.
    Msn,
    /// The qq service.
    Qq,
    /// The skype service.
    Skype,
    /// The yahoo service.
    Yahoo,
}

impl CNInstantMessageService {
    /// Returns all supported values.
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

    /// Returns the framework symbol name.
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

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for `CNSocialProfile` service constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNSocialProfileService {
    /// The facebook service.
    Facebook,
    /// The flickr service.
    Flickr,
    /// The linked in service.
    LinkedIn,
    /// The my space service.
    MySpace,
    /// The sina weibo service.
    SinaWeibo,
    /// The tencent weibo service.
    TencentWeibo,
    /// The twitter service.
    Twitter,
    /// The yelp service.
    Yelp,
    /// The gaMe center service.
    GameCenter,
}

impl CNSocialProfileService {
    /// Returns all supported values.
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

    /// Returns the framework symbol name.
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

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts `NSError.userInfo` keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNErrorUserInfoKey {
    /// The affected records key.
    AffectedRecords,
    /// The affected record identifiers key.
    AffectedRecordIdentifiers,
    /// The validation errors key.
    ValidationErrors,
    /// The key paths key.
    KeyPaths,
}

impl CNErrorUserInfoKey {
    /// Returns all supported values.
    pub fn all_supported() -> &'static [Self] {
        &[
            Self::AffectedRecords,
            Self::AffectedRecordIdentifiers,
            Self::ValidationErrors,
            Self::KeyPaths,
        ]
    }

    /// Returns the framework symbol name.
    pub const fn symbol_name(self) -> &'static str {
        match self {
            Self::AffectedRecords => "CNErrorUserInfoAffectedRecordsKey",
            Self::AffectedRecordIdentifiers => "CNErrorUserInfoAffectedRecordIdentifiersKey",
            Self::ValidationErrors => "CNErrorUserInfoValidationErrorsKey",
            Self::KeyPaths => "CNErrorUserInfoKeyPathsKey",
        }
    }

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

/// Typed wrappers for Contacts contact-relation labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CNContactRelationLabel {
    /// The assistant label.
    Assistant,
    /// The manager label.
    Manager,
    /// The colleague label.
    Colleague,
    /// The teacher label.
    Teacher,
    /// The sibling label.
    Sibling,
    /// The younger sibling label.
    YoungerSibling,
    /// The elder sibling label.
    ElderSibling,
    /// The sister label.
    Sister,
    /// The younger sister label.
    YoungerSister,
    /// The youngest sister label.
    YoungestSister,
    /// The elder sister label.
    ElderSister,
    /// The eldest sister label.
    EldestSister,
    /// The brother label.
    Brother,
    /// The younger brother label.
    YoungerBrother,
    /// The youngest brother label.
    YoungestBrother,
    /// The elder brother label.
    ElderBrother,
    /// The eldest brother label.
    EldestBrother,
    /// The friend label.
    Friend,
    /// The male friend label.
    MaleFriend,
    /// The female friend label.
    FemaleFriend,
    /// The spouse label.
    Spouse,
    /// The wife label.
    Wife,
    /// The husband label.
    Husband,
    /// The partner label.
    Partner,
    /// The male partner label.
    MalePartner,
    /// The female partner label.
    FemalePartner,
    /// The girlfriend or boyfriend label.
    GirlfriendOrBoyfriend,
    /// The girlfriend label.
    Girlfriend,
    /// The boyfriend label.
    Boyfriend,
    /// The parent label.
    Parent,
    /// The mother label.
    Mother,
    /// The father label.
    Father,
    /// The child label.
    Child,
    /// The daughter label.
    Daughter,
    /// The son label.
    Son,
    /// The grandparent label.
    Grandparent,
    /// The grandmother label.
    Grandmother,
    /// The grandmother mothers mother label.
    GrandmotherMothersMother,
    /// The grandmother fathers mother label.
    GrandmotherFathersMother,
    /// The grandfather label.
    Grandfather,
    /// The grandfather mothers father label.
    GrandfatherMothersFather,
    /// The grandfather fathers father label.
    GrandfatherFathersFather,
    /// The great grandparent label.
    GreatGrandparent,
    /// The great grandmother label.
    GreatGrandmother,
    /// The great grandfather label.
    GreatGrandfather,
    /// The grandchild label.
    Grandchild,
    /// The granddaughter label.
    Granddaughter,
    /// The granddaughter daughters daughter label.
    GranddaughterDaughtersDaughter,
    /// The granddaughter sons daughter label.
    GranddaughterSonsDaughter,
    /// The grandson label.
    Grandson,
    /// The grandson daughters son label.
    GrandsonDaughtersSon,
    /// The grandson sons son label.
    GrandsonSonsSon,
    /// The great grandchild label.
    GreatGrandchild,
    /// The great granddaughter label.
    GreatGranddaughter,
    /// The great grandson label.
    GreatGrandson,
    /// The parent in law label.
    ParentInLaw,
    /// The mother in law label.
    MotherInLaw,
    /// The mother in law wifes mother label.
    MotherInLawWifesMother,
    /// The mother in law husbands mother label.
    MotherInLawHusbandsMother,
    /// The father in law label.
    FatherInLaw,
    /// The father in law wifes father label.
    FatherInLawWifesFather,
    /// The father in law husbands father label.
    FatherInLawHusbandsFather,
    /// The co parent in law label.
    CoParentInLaw,
    /// The co mother in law label.
    CoMotherInLaw,
    /// The co father in law label.
    CoFatherInLaw,
    /// The sibling in law label.
    SiblingInLaw,
    /// The younger sibling in law label.
    YoungerSiblingInLaw,
    /// The elder sibling in law label.
    ElderSiblingInLaw,
    /// The sister in law label.
    SisterInLaw,
    /// The younger sister in law label.
    YoungerSisterInLaw,
    /// The elder sister in law label.
    ElderSisterInLaw,
    /// The sister in law spouses sister label.
    SisterInLawSpousesSister,
    /// The sister in law wifes sister label.
    SisterInLawWifesSister,
    /// The sister in law husbands sister label.
    SisterInLawHusbandsSister,
    /// The sister in law brothers wife label.
    SisterInLawBrothersWife,
    /// The sister in law younger brothers wife label.
    SisterInLawYoungerBrothersWife,
    /// The sister in law elder brothers wife label.
    SisterInLawElderBrothersWife,
    /// The brother in law label.
    BrotherInLaw,
    /// The younger brother in law label.
    YoungerBrotherInLaw,
    /// The elder brother in law label.
    ElderBrotherInLaw,
    /// The brother in law spouses brother label.
    BrotherInLawSpousesBrother,
    /// The brother in law husbands brother label.
    BrotherInLawHusbandsBrother,
    /// The brother in law wifes brother label.
    BrotherInLawWifesBrother,
    /// The brother in law sisters husband label.
    BrotherInLawSistersHusband,
    /// The brother in law younger sisters husband label.
    BrotherInLawYoungerSistersHusband,
    /// The brother in law elder sisters husband label.
    BrotherInLawElderSistersHusband,
    /// The sister in law wifes brothers wife label.
    SisterInLawWifesBrothersWife,
    /// The sister in law husbands brothers wife label.
    SisterInLawHusbandsBrothersWife,
    /// The brother in law wifes sisters husband label.
    BrotherInLawWifesSistersHusband,
    /// The brother in law husbands sisters husband label.
    BrotherInLawHusbandsSistersHusband,
    /// The co sibling in law label.
    CoSiblingInLaw,
    /// The co sister in law label.
    CoSisterInLaw,
    /// The co brother in law label.
    CoBrotherInLaw,
    /// The child in law label.
    ChildInLaw,
    /// The daughter in law label.
    DaughterInLaw,
    /// The son in law label.
    SonInLaw,
    /// The cousin label.
    Cousin,
    /// The younger cousin label.
    YoungerCousin,
    /// The elder cousin label.
    ElderCousin,
    /// The male cousin label.
    MaleCousin,
    /// The female cousin label.
    FemaleCousin,
    /// The cousin parents siblings child label.
    CousinParentsSiblingsChild,
    /// The cousin parents siblings son label.
    CousinParentsSiblingsSon,
    /// The younger cousin parents siblings son label.
    YoungerCousinParentsSiblingsSon,
    /// The elder cousin parents siblings son label.
    ElderCousinParentsSiblingsSon,
    /// The cousin parents siblings daughter label.
    CousinParentsSiblingsDaughter,
    /// The younger cousin parents siblings daughter label.
    YoungerCousinParentsSiblingsDaughter,
    /// The elder cousin parents siblings daughter label.
    ElderCousinParentsSiblingsDaughter,
    /// The cousin mothers sisters daughter label.
    CousinMothersSistersDaughter,
    /// The younger cousin mothers sisters daughter label.
    YoungerCousinMothersSistersDaughter,
    /// The elder cousin mothers sisters daughter label.
    ElderCousinMothersSistersDaughter,
    /// The cousin mothers sisters son label.
    CousinMothersSistersSon,
    /// The younger cousin mothers sisters son label.
    YoungerCousinMothersSistersSon,
    /// The elder cousin mothers sisters son label.
    ElderCousinMothersSistersSon,
    /// The cousin mothers brothers daughter label.
    CousinMothersBrothersDaughter,
    /// The younger cousin mothers brothers daughter label.
    YoungerCousinMothersBrothersDaughter,
    /// The elder cousin mothers brothers daughter label.
    ElderCousinMothersBrothersDaughter,
    /// The cousin mothers brothers son label.
    CousinMothersBrothersSon,
    /// The younger cousin mothers brothers son label.
    YoungerCousinMothersBrothersSon,
    /// The elder cousin mothers brothers son label.
    ElderCousinMothersBrothersSon,
    /// The cousin fathers sisters daughter label.
    CousinFathersSistersDaughter,
    /// The younger cousin fathers sisters daughter label.
    YoungerCousinFathersSistersDaughter,
    /// The elder cousin fathers sisters daughter label.
    ElderCousinFathersSistersDaughter,
    /// The cousin fathers sisters son label.
    CousinFathersSistersSon,
    /// The younger cousin fathers sisters son label.
    YoungerCousinFathersSistersSon,
    /// The elder cousin fathers sisters son label.
    ElderCousinFathersSistersSon,
    /// The cousin fathers brothers daughter label.
    CousinFathersBrothersDaughter,
    /// The younger cousin fathers brothers daughter label.
    YoungerCousinFathersBrothersDaughter,
    /// The elder cousin fathers brothers daughter label.
    ElderCousinFathersBrothersDaughter,
    /// The cousin fathers brothers son label.
    CousinFathersBrothersSon,
    /// The younger cousin fathers brothers son label.
    YoungerCousinFathersBrothersSon,
    /// The elder cousin fathers brothers son label.
    ElderCousinFathersBrothersSon,
    /// The cousin grandparents siblings child label.
    CousinGrandparentsSiblingsChild,
    /// The cousin grandparents siblings daughter label.
    CousinGrandparentsSiblingsDaughter,
    /// The cousin grandparents siblings son label.
    CousinGrandparentsSiblingsSon,
    /// The younger cousin mothers siblings son or fathers sisters son label.
    YoungerCousinMothersSiblingsSonOrFathersSistersSon,
    /// The elder cousin mothers siblings son or fathers sisters son label.
    ElderCousinMothersSiblingsSonOrFathersSistersSon,
    /// The younger cousin mothers siblings daughter or fathers sisters daughter label.
    YoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter,
    /// The elder cousin mothers siblings daughter or fathers sisters daughter label.
    ElderCousinMothersSiblingsDaughterOrFathersSistersDaughter,
    /// The parents sibling label.
    ParentsSibling,
    /// The parents younger sibling label.
    ParentsYoungerSibling,
    /// The parents elder sibling label.
    ParentsElderSibling,
    /// The parents sibling mothers sibling label.
    ParentsSiblingMothersSibling,
    /// The parents sibling mothers younger sibling label.
    ParentsSiblingMothersYoungerSibling,
    /// The parents sibling mothers elder sibling label.
    ParentsSiblingMothersElderSibling,
    /// The parents sibling fathers sibling label.
    ParentsSiblingFathersSibling,
    /// The parents sibling fathers younger sibling label.
    ParentsSiblingFathersYoungerSibling,
    /// The parents sibling fathers elder sibling label.
    ParentsSiblingFathersElderSibling,
    /// The aunt label.
    Aunt,
    /// The aunt parents sister label.
    AuntParentsSister,
    /// The aunt parents younger sister label.
    AuntParentsYoungerSister,
    /// The aunt parents elder sister label.
    AuntParentsElderSister,
    /// The aunt fathers sister label.
    AuntFathersSister,
    /// The aunt fathers younger sister label.
    AuntFathersYoungerSister,
    /// The aunt fathers elder sister label.
    AuntFathersElderSister,
    /// The aunt fathers brothers wife label.
    AuntFathersBrothersWife,
    /// The aunt fathers younger brothers wife label.
    AuntFathersYoungerBrothersWife,
    /// The aunt fathers elder brothers wife label.
    AuntFathersElderBrothersWife,
    /// The aunt mothers sister label.
    AuntMothersSister,
    /// The aunt mothers younger sister label.
    AuntMothersYoungerSister,
    /// The aunt mothers elder sister label.
    AuntMothersElderSister,
    /// The aunt mothers brothers wife label.
    AuntMothersBrothersWife,
    /// The grandaunt label.
    Grandaunt,
    /// The uncle label.
    Uncle,
    /// The uncle parents brother label.
    UncleParentsBrother,
    /// The uncle parents younger brother label.
    UncleParentsYoungerBrother,
    /// The uncle parents elder brother label.
    UncleParentsElderBrother,
    /// The uncle mothers brother label.
    UncleMothersBrother,
    /// The uncle mothers younger brother label.
    UncleMothersYoungerBrother,
    /// The uncle mothers elder brother label.
    UncleMothersElderBrother,
    /// The uncle mothers sisters husband label.
    UncleMothersSistersHusband,
    /// The uncle fathers brother label.
    UncleFathersBrother,
    /// The uncle fathers younger brother label.
    UncleFathersYoungerBrother,
    /// The uncle fathers elder brother label.
    UncleFathersElderBrother,
    /// The uncle fathers sisters husband label.
    UncleFathersSistersHusband,
    /// The uncle fathers younger sisters husband label.
    UncleFathersYoungerSistersHusband,
    /// The uncle fathers elder sisters husband label.
    UncleFathersElderSistersHusband,
    /// The granduncle label.
    Granduncle,
    /// The siblings child label.
    SiblingsChild,
    /// The niece label.
    Niece,
    /// The niece sisters daughter label.
    NieceSistersDaughter,
    /// The niece brothers daughter label.
    NieceBrothersDaughter,
    /// The niece sisters daughter or wifes siblings daughter label.
    NieceSistersDaughterOrWifesSiblingsDaughter,
    /// The niece brothers daughter or husbands siblings daughter label.
    NieceBrothersDaughterOrHusbandsSiblingsDaughter,
    /// The nephew label.
    Nephew,
    /// The nephew sisters son label.
    NephewSistersSon,
    /// The nephew brothers son label.
    NephewBrothersSon,
    /// The nephew brothers son or husbands siblings son label.
    NephewBrothersSonOrHusbandsSiblingsSon,
    /// The nephew sisters son or wifes siblings son label.
    NephewSistersSonOrWifesSiblingsSon,
    /// The grandniece label.
    Grandniece,
    /// The grandniece sisters granddaughter label.
    GrandnieceSistersGranddaughter,
    /// The grandniece brothers granddaughter label.
    GrandnieceBrothersGranddaughter,
    /// The grandnephew label.
    Grandnephew,
    /// The grandnephew sisters grandson label.
    GrandnephewSistersGrandson,
    /// The grandnephew brothers grandson label.
    GrandnephewBrothersGrandson,
    /// The stepparent label.
    Stepparent,
    /// The stepmother label.
    Stepmother,
    /// The stepfather label.
    Stepfather,
    /// The stepchild label.
    Stepchild,
    /// The stepdaughter label.
    Stepdaughter,
    /// The stepson label.
    Stepson,
    /// The stepsister label.
    Stepsister,
    /// The stepbrother label.
    Stepbrother,
    /// The mother in law or stepmother label.
    MotherInLawOrStepmother,
    /// The father in law or stepfather label.
    FatherInLawOrStepfather,
    /// The daughter in law or stepdaughter label.
    DaughterInLawOrStepdaughter,
    /// The son in law or stepson label.
    SonInLawOrStepson,
    /// The cousin or siblings child label.
    CousinOrSiblingsChild,
    /// The niece or cousin label.
    NieceOrCousin,
    /// The nephew or cousin label.
    NephewOrCousin,
    /// The grandchild or siblings child label.
    GrandchildOrSiblingsChild,
    /// The granddaughter or niece label.
    GranddaughterOrNiece,
    /// The grandson or nephew label.
    GrandsonOrNephew,
    /// The great grandchild or siblings grandchild label.
    GreatGrandchildOrSiblingsGrandchild,
    /// The daughter in law or sister in law label.
    DaughterInLawOrSisterInLaw,
    /// The son in law or brother in law label.
    SonInLawOrBrotherInLaw,
}

impl CNContactRelationLabel {
    #[allow(clippy::too_many_lines)]
    /// Returns all supported values.
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
    /// Returns the framework symbol name.
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

    /// Returns the framework constant value.
    pub fn value(self) -> Result<String, ContactsError> {
        copy_contacts_constant(self.symbol_name(), self.symbol_name())
    }
}

impl CNLabeledValueLabel {
    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNEmailAddressLabel {
    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNUrlAddressLabel {
    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNDateLabel {
    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNPhoneNumberLabel {
    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}

impl CNInstantMessageService {
    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNInstantMessageAddress::localized_string_for_service(&self.value()?)
    }
}

impl CNSocialProfileService {
    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNSocialProfile::localized_string_for_service(&self.value()?)
    }
}

impl CNContactRelationLabel {
    /// Returns the localized string.
    pub fn localized_string(self) -> Result<String, ContactsError> {
        CNLabeledValue::<String>::localized_string_for_label(&self.value()?)
    }
}
