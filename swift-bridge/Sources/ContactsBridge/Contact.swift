import Contacts
import Foundation

enum CNRContactKey: String, Codable, Hashable {
  case identifier
  case contactType
  case namePrefix
  case givenName
  case middleName
  case familyName
  case previousFamilyName
  case nameSuffix
  case nickname
  case organizationName
  case departmentName
  case jobTitle
  case phoneticGivenName
  case phoneticMiddleName
  case phoneticFamilyName
  case phoneticOrganizationName
  case note
  case imageData
  case thumbnailImageData
  case imageDataAvailable
  case phoneNumbers
  case emailAddresses
  case postalAddresses
  case dates
  case urlAddresses
  case contactRelations
  case socialProfiles
  case instantMessageAddresses
  case birthday
  case nonGregorianBirthday
}

enum CNRContactSortOrder: String, Codable {
  case none
  case userDefault
  case givenName
  case familyName
}

enum CNRContactType: String, Codable {
  case person
  case organization
}

struct CNRContactPayload: Codable {
  var identifier: String
  var fetchedKeys: [CNRContactKey]
  var contactType: CNRContactType?
  var namePrefix: String?
  var givenName: String?
  var middleName: String?
  var familyName: String?
  var previousFamilyName: String?
  var nameSuffix: String?
  var nickname: String?
  var organizationName: String?
  var departmentName: String?
  var jobTitle: String?
  var phoneticGivenName: String?
  var phoneticMiddleName: String?
  var phoneticFamilyName: String?
  var phoneticOrganizationName: String?
  var note: String?
  var imageData: Data?
  var thumbnailImageData: Data?
  var imageDataAvailable: Bool?
  var phoneNumbers: [CNRLabeledValuePayload<CNRPhoneNumberPayload>]
  var emailAddresses: [CNRLabeledValuePayload<String>]
  var postalAddresses: [CNRLabeledValuePayload<CNRPostalAddressPayload>]
  var dates: [CNRLabeledValuePayload<CNRDateComponentsPayload>]
  var urlAddresses: [CNRLabeledValuePayload<String>]
  var contactRelations: [CNRLabeledValuePayload<CNRContactRelationPayload>]
  var socialProfiles: [CNRLabeledValuePayload<CNRSocialProfilePayload>]
  var instantMessageAddresses: [CNRLabeledValuePayload<CNRInstantMessageAddressPayload>]
  var birthday: CNRDateComponentsPayload?
  var nonGregorianBirthday: CNRDateComponentsPayload?
}

func cnrPayloadContactType(from contactType: CNContactType) -> CNRContactType {
  switch contactType {
  case .person:
    .person
  case .organization:
    .organization
  @unknown default:
    .person
  }
}

func cnrNativeContactType(_ contactType: CNRContactType) -> CNContactType {
  switch contactType {
  case .person:
    .person
  case .organization:
    .organization
  }
}

func cnrContactKeyConstant(_ key: CNRContactKey) -> NSString {
  switch key {
  case .identifier:
    CNContactIdentifierKey as NSString
  case .contactType:
    CNContactTypeKey as NSString
  case .namePrefix:
    CNContactNamePrefixKey as NSString
  case .givenName:
    CNContactGivenNameKey as NSString
  case .middleName:
    CNContactMiddleNameKey as NSString
  case .familyName:
    CNContactFamilyNameKey as NSString
  case .previousFamilyName:
    CNContactPreviousFamilyNameKey as NSString
  case .nameSuffix:
    CNContactNameSuffixKey as NSString
  case .nickname:
    CNContactNicknameKey as NSString
  case .organizationName:
    CNContactOrganizationNameKey as NSString
  case .departmentName:
    CNContactDepartmentNameKey as NSString
  case .jobTitle:
    CNContactJobTitleKey as NSString
  case .phoneticGivenName:
    CNContactPhoneticGivenNameKey as NSString
  case .phoneticMiddleName:
    CNContactPhoneticMiddleNameKey as NSString
  case .phoneticFamilyName:
    CNContactPhoneticFamilyNameKey as NSString
  case .phoneticOrganizationName:
    CNContactPhoneticOrganizationNameKey as NSString
  case .note:
    CNContactNoteKey as NSString
  case .imageData:
    CNContactImageDataKey as NSString
  case .thumbnailImageData:
    CNContactThumbnailImageDataKey as NSString
  case .imageDataAvailable:
    CNContactImageDataAvailableKey as NSString
  case .phoneNumbers:
    CNContactPhoneNumbersKey as NSString
  case .emailAddresses:
    CNContactEmailAddressesKey as NSString
  case .postalAddresses:
    CNContactPostalAddressesKey as NSString
  case .dates:
    CNContactDatesKey as NSString
  case .urlAddresses:
    CNContactUrlAddressesKey as NSString
  case .contactRelations:
    CNContactRelationsKey as NSString
  case .socialProfiles:
    CNContactSocialProfilesKey as NSString
  case .instantMessageAddresses:
    CNContactInstantMessageAddressesKey as NSString
  case .birthday:
    CNContactBirthdayKey as NSString
  case .nonGregorianBirthday:
    CNContactNonGregorianBirthdayKey as NSString
  }
}

func cnrEncodeContact(
  _ contact: CNContact,
  requestedKeys: Set<CNRContactKey>
) -> CNRContactPayload {
  let fetchedKeys = Array(requestedKeys.union([.identifier])).sorted { $0.rawValue < $1.rawValue }
  return CNRContactPayload(
    identifier: contact.identifier,
    fetchedKeys: fetchedKeys,
    contactType: requestedKeys.contains(.contactType)
      ? cnrPayloadContactType(from: contact.contactType) : nil,
    namePrefix: requestedKeys.contains(.namePrefix) ? contact.namePrefix : nil,
    givenName: requestedKeys.contains(.givenName) ? contact.givenName : nil,
    middleName: requestedKeys.contains(.middleName) ? contact.middleName : nil,
    familyName: requestedKeys.contains(.familyName) ? contact.familyName : nil,
    previousFamilyName: requestedKeys.contains(.previousFamilyName)
      ? contact.previousFamilyName : nil,
    nameSuffix: requestedKeys.contains(.nameSuffix) ? contact.nameSuffix : nil,
    nickname: requestedKeys.contains(.nickname) ? contact.nickname : nil,
    organizationName: requestedKeys.contains(.organizationName) ? contact.organizationName : nil,
    departmentName: requestedKeys.contains(.departmentName) ? contact.departmentName : nil,
    jobTitle: requestedKeys.contains(.jobTitle) ? contact.jobTitle : nil,
    phoneticGivenName: requestedKeys.contains(.phoneticGivenName) ? contact.phoneticGivenName : nil,
    phoneticMiddleName: requestedKeys.contains(.phoneticMiddleName)
      ? contact.phoneticMiddleName : nil,
    phoneticFamilyName: requestedKeys.contains(.phoneticFamilyName)
      ? contact.phoneticFamilyName : nil,
    phoneticOrganizationName: requestedKeys.contains(.phoneticOrganizationName)
      ? contact.phoneticOrganizationName : nil,
    note: requestedKeys.contains(.note) ? contact.note : nil,
    imageData: requestedKeys.contains(.imageData) ? contact.imageData : nil,
    thumbnailImageData: requestedKeys.contains(.thumbnailImageData)
      ? contact.thumbnailImageData : nil,
    imageDataAvailable: requestedKeys.contains(.imageDataAvailable)
      ? contact.imageDataAvailable : nil,
    phoneNumbers: requestedKeys.contains(.phoneNumbers)
      ? contact.phoneNumbers.map {
        CNRLabeledValuePayload(
          identifier: $0.identifier,
          label: $0.label,
          value: cnrEncodePhoneNumber($0.value)
        )
      } : [],
    emailAddresses: requestedKeys.contains(.emailAddresses)
      ? contact.emailAddresses.map {
        CNRLabeledValuePayload(
          identifier: $0.identifier, label: $0.label, value: $0.value as String)
      } : [],
    postalAddresses: requestedKeys.contains(.postalAddresses)
      ? contact.postalAddresses.map {
        CNRLabeledValuePayload(
          identifier: $0.identifier, label: $0.label, value: cnrEncodePostalAddress($0.value))
      } : [],
    dates: requestedKeys.contains(.dates)
      ? contact.dates.map {
        CNRLabeledValuePayload(
          identifier: $0.identifier, label: $0.label, value: cnrEncodeDateComponents($0.value)!)
      } : [],
    urlAddresses: requestedKeys.contains(.urlAddresses)
      ? contact.urlAddresses.map {
        CNRLabeledValuePayload(
          identifier: $0.identifier, label: $0.label, value: $0.value as String)
      } : [],
    contactRelations: requestedKeys.contains(.contactRelations)
      ? contact.contactRelations.map {
        CNRLabeledValuePayload(
          identifier: $0.identifier, label: $0.label, value: cnrEncodeContactRelation($0.value))
      } : [],
    socialProfiles: requestedKeys.contains(.socialProfiles)
      ? contact.socialProfiles.map {
        CNRLabeledValuePayload(
          identifier: $0.identifier, label: $0.label, value: cnrEncodeSocialProfile($0.value))
      } : [],
    instantMessageAddresses: requestedKeys.contains(.instantMessageAddresses)
      ? contact.instantMessageAddresses.map {
        CNRLabeledValuePayload(
          identifier: $0.identifier, label: $0.label,
          value: cnrEncodeInstantMessageAddress($0.value))
      } : [],
    birthday: requestedKeys.contains(.birthday) ? cnrEncodeDateComponents(contact.birthday) : nil,
    nonGregorianBirthday: requestedKeys.contains(.nonGregorianBirthday)
      ? cnrEncodeDateComponents(contact.nonGregorianBirthday) : nil
  )
}

@_cdecl("cn_contact_localized_string_for_key")
public func cn_contact_localized_string_for_key(
  _ keyJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let key = try cnrDecodeJSON(keyJSON, as: CNRContactKey.self)
    return cnrCString(CNContact.localizedString(forKey: cnrContactKeyConstant(key) as String))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}
