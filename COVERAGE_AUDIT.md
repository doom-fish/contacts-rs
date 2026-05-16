# contacts-rs coverage audit (vs MacOSX26.2.sdk)

Methodology: full audit of top-level Contacts.framework public declarations (interfaces, categories, protocols, enums, exported constants).
Strictness: a symbol only counts as verified when the crate exposes a dedicated public Rust type/enum/helper for it; raw constant families therefore dominate the gap count, especially the contact-relation labels.

SDK_PUBLIC_SYMBOLS: 360
VERIFIED: 95
GAPS: 265
EXEMPT: 0
COVERAGE_PCT: 26.39%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CNChangeHistoryAddContactEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::AddContact |
| CNChangeHistoryAddGroupEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::AddGroup |
| CNChangeHistoryAddMemberToGroupEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::AddMemberToGroup |
| CNChangeHistoryAddSubgroupToGroupEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::AddSubgroupToGroup |
| CNChangeHistoryDeleteContactEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::DeleteContact |
| CNChangeHistoryDeleteGroupEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::DeleteGroup |
| CNChangeHistoryDropEverythingEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::DropEverything |
| CNChangeHistoryEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent |
| CNChangeHistoryFetchRequest | interface | CNChangeHistoryFetchRequest.h | contacts::CNChangeHistoryFetchRequest |
| CNChangeHistoryRemoveMemberFromGroupEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::RemoveMemberFromGroup |
| CNChangeHistoryRemoveSubgroupFromGroupEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::RemoveSubgroupFromGroup |
| CNChangeHistoryUpdateContactEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::UpdateContact |
| CNChangeHistoryUpdateGroupEvent | interface | CNChangeHistoryEvent.h | contacts::CNChangeHistoryEvent::UpdateGroup |
| CNContact | interface | CNContact.h | contacts::CNContact |
| CNContactFetchRequest | interface | CNContactFetchRequest.h | contacts::CNContactFetchRequest |
| CNContactFormatter | interface | CNContactFormatter.h | contacts::CNContactFormatter |
| CNContactProperty | interface | CNContactProperty.h | contacts::CNContactProperty |
| CNContactRelation | interface | CNContactRelation.h | contacts::CNContactRelation |
| CNContactStore | interface | CNContactStore.h | contacts::CNContactStore |
| CNContactsUserDefaults | interface | CNContactsUserDefaults.h | contacts::CNContactsUserDefaults |
| CNContactVCardSerialization | interface | CNContactVCardSerialization.h | contacts::CNContactVCardSerialization |
| CNContainer | interface | CNContainer.h | contacts::CNContainer |
| CNFetchRequest | interface | CNFetchRequest.h | contacts::CNFetchRequest |
| CNFetchResult | interface | CNFetchResult.h | contacts::CNFetchResult<T> |
| CNGroup | interface | CNGroup.h | contacts::CNGroup |
| CNInstantMessageAddress | interface | CNInstantMessageAddress.h | contacts::CNInstantMessageAddress |
| CNLabeledValue | interface | CNLabeledValue.h | contacts::CNLabeledValue<T> |
| CNMutableContact | interface | CNMutableContact.h | contacts::CNMutableContact |
| CNMutableGroup | interface | CNMutableGroup.h | contacts::CNMutableGroup |
| CNPhoneNumber | interface | CNPhoneNumber.h | contacts::CNPhoneNumber |
| CNPostalAddress | interface | CNPostalAddress.h | contacts::CNPostalAddress |
| CNPostalAddressFormatter | interface | CNPostalAddressFormatter.h | contacts::CNPostalAddressFormatter |
| CNSaveRequest | interface | CNSaveRequest.h | contacts::CNSaveRequest |
| CNSocialProfile | interface | CNSocialProfile.h | contacts::CNSocialProfile |
| CNContact (Predicates) | category | CNContact+Predicates.h | contacts::CNContactPredicate |
| CNContact (NSItemProvider) | category | CNContact+NSItemProvider.h | contacts::CNContact::{readable_type_identifiers_for_item_provider, writable_type_identifiers_for_item_provider, item_provider_data, from_item_provider_data} |
| CNContainer (Predicates) | category | CNContainer+Predicates.h | contacts::CNContainerPredicate |
| CNGroup (Predicates) | category | CNGroup+Predicates.h | contacts::CNGroupPredicate |
| CNKeyDescriptor | protocol | CNContact.h | contacts::CNKeyDescriptor |
| CNAuthorizationStatus | enum | CNContactStore.h | contacts::CNAuthorizationStatus |
| CNContactDisplayNameOrder | enum | CNContactFormatter.h | contacts::CNContactDisplayNameOrder |
| CNContactFormatterStyle | enum | CNContactFormatter.h | contacts::CNContactFormatterStyle |
| CNContactSortOrder | enum | CNContact.h | contacts::CNContactSortOrder |
| CNContactType | enum | CNContact.h | contacts::CNContactType |
| CNContainerType | enum | CNContainer.h | contacts::CNContainerType |
| CNErrorCode | enum | CNError.h | contacts::CNErrorCode |
| CNPostalAddressFormatterStyle | enum | CNPostalAddressFormatter.h | contacts::CNPostalAddressFormatterStyle |
| CNContactBirthdayKey | const | CNContact.h | contacts::CNContactKey::Birthday |
| CNContactDatesKey | const | CNContact.h | contacts::CNContactKey::Dates |
| CNContactDepartmentNameKey | const | CNContact.h | contacts::CNContactKey::DepartmentName |
| CNContactEmailAddressesKey | const | CNContact.h | contacts::CNContactKey::EmailAddresses |
| CNContactFamilyNameKey | const | CNContact.h | contacts::CNContactKey::FamilyName |
| CNContactGivenNameKey | const | CNContact.h | contacts::CNContactKey::GivenName |
| CNContactIdentifierKey | const | CNContact.h | contacts::CNContactKey::Identifier |
| CNContactImageDataAvailableKey | const | CNContact.h | contacts::CNContactKey::ImageDataAvailable |
| CNContactImageDataKey | const | CNContact.h | contacts::CNContactKey::ImageData |
| CNContactInstantMessageAddressesKey | const | CNContact.h | contacts::CNContactKey::InstantMessageAddresses |
| CNContactJobTitleKey | const | CNContact.h | contacts::CNContactKey::JobTitle |
| CNContactMiddleNameKey | const | CNContact.h | contacts::CNContactKey::MiddleName |
| CNContactNamePrefixKey | const | CNContact.h | contacts::CNContactKey::NamePrefix |
| CNContactNameSuffixKey | const | CNContact.h | contacts::CNContactKey::NameSuffix |
| CNContactNicknameKey | const | CNContact.h | contacts::CNContactKey::Nickname |
| CNContactNonGregorianBirthdayKey | const | CNContact.h | contacts::CNContactKey::NonGregorianBirthday |
| CNContactNoteKey | const | CNContact.h | contacts::CNContactKey::Note |
| CNContactOrganizationNameKey | const | CNContact.h | contacts::CNContactKey::OrganizationName |
| CNContactPhoneNumbersKey | const | CNContact.h | contacts::CNContactKey::PhoneNumbers |
| CNContactPhoneticFamilyNameKey | const | CNContact.h | contacts::CNContactKey::PhoneticFamilyName |
| CNContactPhoneticGivenNameKey | const | CNContact.h | contacts::CNContactKey::PhoneticGivenName |
| CNContactPhoneticMiddleNameKey | const | CNContact.h | contacts::CNContactKey::PhoneticMiddleName |
| CNContactPhoneticOrganizationNameKey | const | CNContact.h | contacts::CNContactKey::PhoneticOrganizationName |
| CNContactPostalAddressesKey | const | CNContact.h | contacts::CNContactKey::PostalAddresses |
| CNContactPreviousFamilyNameKey | const | CNContact.h | contacts::CNContactKey::PreviousFamilyName |
| CNContactPropertyAttribute | const | CNContactFormatter.h | contacts::CNAttributedStringRun::property |
| CNContactRelationsKey | const | CNContact.h | contacts::CNContactKey::ContactRelations |
| CNContactSocialProfilesKey | const | CNContact.h | contacts::CNContactKey::SocialProfiles |
| CNContactStoreDidChangeNotification | const | CNContactStore.h | contacts::contact_store_did_change_notification_name() |
| CNContactThumbnailImageDataKey | const | CNContact.h | contacts::CNContactKey::ThumbnailImageData |
| CNContactTypeKey | const | CNContact.h | contacts::CNContactKey::ContactType |
| CNContactUrlAddressesKey | const | CNContact.h | contacts::CNContactKey::UrlAddresses |
| CNInstantMessageAddressServiceKey | const | CNInstantMessageAddress.h | contacts::CNInstantMessageAddressKey::Service |
| CNInstantMessageAddressUsernameKey | const | CNInstantMessageAddress.h | contacts::CNInstantMessageAddressKey::Username |
| CNPostalAddressCityKey | const | CNPostalAddress.h | contacts::CNPostalAddressKey::City |
| CNPostalAddressCountryKey | const | CNPostalAddress.h | contacts::CNPostalAddressKey::Country |
| CNPostalAddressISOCountryCodeKey | const | CNPostalAddress.h | contacts::CNPostalAddressKey::IsoCountryCode |
| CNPostalAddressLocalizedPropertyNameAttribute | const | CNPostalAddressFormatter.h | contacts::CNAttributedStringRun::localized_property_name |
| CNPostalAddressPostalCodeKey | const | CNPostalAddress.h | contacts::CNPostalAddressKey::PostalCode |
| CNPostalAddressPropertyAttribute | const | CNPostalAddressFormatter.h | contacts::CNAttributedStringRun::property |
| CNPostalAddressStateKey | const | CNPostalAddress.h | contacts::CNPostalAddressKey::State |
| CNPostalAddressStreetKey | const | CNPostalAddress.h | contacts::CNPostalAddressKey::Street |
| CNPostalAddressSubAdministrativeAreaKey | const | CNPostalAddress.h | contacts::CNPostalAddressKey::SubAdministrativeArea |
| CNPostalAddressSubLocalityKey | const | CNPostalAddress.h | contacts::CNPostalAddressKey::SubLocality |
| CNSocialProfileServiceKey | const | CNSocialProfile.h | contacts::CNSocialProfileKey::Service |
| CNSocialProfileURLStringKey | const | CNSocialProfile.h | contacts::CNSocialProfileKey::UrlString |
| CNSocialProfileUserIdentifierKey | const | CNSocialProfile.h | contacts::CNSocialProfileKey::UserIdentifier |
| CNSocialProfileUsernameKey | const | CNSocialProfile.h | contacts::CNSocialProfileKey::Username |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| CNMutablePostalAddress | interface | CNMutablePostalAddress.h | The crate exposes value-style CNPostalAddress, not CNMutablePostalAddress. |
| NSString (Contacts) | category | CNContact.h | Descriptor semantics are modeled with CNContactKey/CNKeyDescriptor instead of the NSString category. |
| CNChangeHistoryEventVisitor | protocol | CNChangeHistoryEvent.h | Change history is flattened into a Rust enum; the visitor protocol is not exposed. |
| CNEntityType | enum | CNContactStore.h | Store helpers are hard-coded to contacts; CNEntityType is not public. |
| CNContactPropertyNotFetchedExceptionName | const | CNContact.h | The Rust API avoids this exception by tracking fetched keys explicitly. |
| CNContainerIdentifierKey | const | CNContainer.h | Container property key constants are not wrapped as dedicated Rust constants or enums. |
| CNContainerNameKey | const | CNContainer.h | Container property key constants are not wrapped as dedicated Rust constants or enums. |
| CNContainerTypeKey | const | CNContainer.h | Container property key constants are not wrapped as dedicated Rust constants or enums. |
| CNErrorDomain | const | CNError.h | NSErrorInfo preserves generic error details, but this Contacts error constant is not wrapped. |
| CNErrorUserInfoAffectedRecordIdentifiersKey | const | CNError.h | NSErrorInfo preserves generic error details, but this Contacts error constant is not wrapped. |
| CNErrorUserInfoAffectedRecordsKey | const | CNError.h | NSErrorInfo preserves generic error details, but this Contacts error constant is not wrapped. |
| CNErrorUserInfoKeyPathsKey | const | CNError.h | NSErrorInfo preserves generic error details, but this Contacts error constant is not wrapped. |
| CNErrorUserInfoValidationErrorsKey | const | CNError.h | NSErrorInfo preserves generic error details, but this Contacts error constant is not wrapped. |
| CNGroupIdentifierKey | const | CNGroup.h | Group property key constants are not wrapped as dedicated Rust constants or enums. |
| CNGroupNameKey | const | CNGroup.h | Group property key constants are not wrapped as dedicated Rust constants or enums. |
| CNInstantMessageServiceAIM | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceFacebook | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceGaduGadu | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceGoogleTalk | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceICQ | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceJabber | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceMSN | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceQQ | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceSkype | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNInstantMessageServiceYahoo | const | CNInstantMessageAddress.h | Instant-message service constants are not exposed; only generic service strings/localization helpers are public. |
| CNLabelContactRelationAssistant | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAunt | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntFathersBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntFathersElderBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntFathersElderSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntFathersSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntFathersYoungerBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntFathersYoungerSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntMothersBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntMothersElderSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntMothersSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntMothersYoungerSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntParentsElderSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntParentsSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationAuntParentsYoungerSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBoyfriend | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLawElderSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLawHusbandsBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLawHusbandsSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLawSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLawSpousesBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLawWifesBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLawWifesSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationBrotherInLawYoungerSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationChild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationChildInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCoBrotherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCoFatherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCoMotherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCoParentInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCoSiblingInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCoSisterInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationColleague | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousin | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinFathersBrothersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinFathersBrothersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinFathersSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinFathersSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinGrandparentsSiblingsChild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinGrandparentsSiblingsDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinGrandparentsSiblingsSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinMothersBrothersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinMothersBrothersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinMothersSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinMothersSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinOrSiblingsChild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinParentsSiblingsChild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinParentsSiblingsDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationCousinParentsSiblingsSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationDaughterInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationDaughterInLawOrSisterInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationDaughterInLawOrStepdaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderBrotherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousin | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinFathersBrothersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinFathersBrothersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinFathersSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinFathersSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinMothersBrothersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinMothersBrothersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinMothersSiblingsDaughterOrFathersSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinMothersSiblingsSonOrFathersSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinMothersSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinMothersSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinParentsSiblingsDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderCousinParentsSiblingsSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderSiblingInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationElderSisterInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationEldestBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationEldestSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFatherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFatherInLawHusbandsFather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFatherInLawOrStepfather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFatherInLawWifesFather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFemaleCousin | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFemaleFriend | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFemalePartner | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationFriend | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGirlfriend | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGirlfriendOrBoyfriend | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandaunt | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandchild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandchildOrSiblingsChild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGranddaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGranddaughterDaughtersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGranddaughterOrNiece | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGranddaughterSonsDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandfather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandfatherFathersFather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandfatherMothersFather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandmother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandmotherFathersMother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandmotherMothersMother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandnephew | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandnephewBrothersGrandson | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandnephewSistersGrandson | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandniece | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandnieceBrothersGranddaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandnieceSistersGranddaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandparent | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandson | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandsonDaughtersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandsonOrNephew | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGrandsonSonsSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGranduncle | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGreatGrandchild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGreatGrandchildOrSiblingsGrandchild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGreatGranddaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGreatGrandfather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGreatGrandmother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGreatGrandparent | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationGreatGrandson | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationMaleCousin | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationMaleFriend | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationMalePartner | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationManager | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationMother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationMotherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationMotherInLawHusbandsMother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationMotherInLawOrStepmother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationMotherInLawWifesMother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNephew | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNephewBrothersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNephewBrothersSonOrHusbandsSiblingsSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNephewOrCousin | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNephewSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNephewSistersSonOrWifesSiblingsSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNiece | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNieceBrothersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNieceBrothersDaughterOrHusbandsSiblingsDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNieceOrCousin | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNieceSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationNieceSistersDaughterOrWifesSiblingsDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParent | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsElderSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsSiblingFathersElderSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsSiblingFathersSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsSiblingFathersYoungerSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsSiblingMothersElderSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsSiblingMothersSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsSiblingMothersYoungerSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationParentsYoungerSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationPartner | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSiblingInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSiblingsChild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLawBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLawElderBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLawHusbandsBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLawHusbandsSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLawSpousesSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLawWifesBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLawWifesSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSisterInLawYoungerBrothersWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSonInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSonInLawOrBrotherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSonInLawOrStepson | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationSpouse | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationStepbrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationStepchild | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationStepdaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationStepfather | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationStepmother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationStepparent | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationStepsister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationStepson | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationTeacher | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncle | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleFathersBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleFathersElderBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleFathersElderSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleFathersSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleFathersYoungerBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleFathersYoungerSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleMothersBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleMothersElderBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleMothersSistersHusband | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleMothersYoungerBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleParentsBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleParentsElderBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationUncleParentsYoungerBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationWife | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerBrotherInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousin | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinFathersBrothersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinFathersBrothersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinFathersSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinFathersSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinMothersBrothersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinMothersBrothersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinMothersSiblingsSonOrFathersSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinMothersSistersDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinMothersSistersSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinParentsSiblingsDaughter | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerCousinParentsSiblingsSon | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerSibling | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerSiblingInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungerSisterInLaw | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungestBrother | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelContactRelationYoungestSister | const | CNContactRelation.h | Contact-relation label constants are not exposed; the crate only wraps CNContactRelation values. |
| CNLabelDateAnniversary | const | CNLabeledValue.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelEmailiCloud | const | CNLabeledValue.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelHome | const | CNLabeledValue.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelOther | const | CNLabeledValue.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelPhoneNumberAppleWatch | const | CNPhoneNumber.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelPhoneNumberHomeFax | const | CNPhoneNumber.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelPhoneNumberMain | const | CNPhoneNumber.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelPhoneNumberMobile | const | CNPhoneNumber.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelPhoneNumberOtherFax | const | CNPhoneNumber.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelPhoneNumberPager | const | CNPhoneNumber.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelPhoneNumberWorkFax | const | CNPhoneNumber.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelPhoneNumberiPhone | const | CNPhoneNumber.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelSchool | const | CNLabeledValue.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelURLAddressHomePage | const | CNLabeledValue.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNLabelWork | const | CNLabeledValue.h | Raw label constants are not surfaced as dedicated Rust constants or enums. |
| CNSocialProfileServiceFacebook | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |
| CNSocialProfileServiceFlickr | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |
| CNSocialProfileServiceGameCenter | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |
| CNSocialProfileServiceLinkedIn | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |
| CNSocialProfileServiceMySpace | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |
| CNSocialProfileServiceSinaWeibo | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |
| CNSocialProfileServiceTencentWeibo | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |
| CNSocialProfileServiceTwitter | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |
| CNSocialProfileServiceYelp | const | CNSocialProfile.h | Social-profile service constants are not exposed; only generic service strings/localization helpers are public. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| — | — | — | No deprecated macOS symbols were classified as exempt in Contacts.framework. | — |

