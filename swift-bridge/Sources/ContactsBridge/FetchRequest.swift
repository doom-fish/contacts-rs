import Contacts
import Foundation

enum CNRAdditionalKeyDescriptorKind: String, Codable {
  case comparatorKeys
  case formatterRequiredKeys
  case formatterNameOrder
  case formatterDelimiter
  case vcardRequiredKeys
}

struct CNRAdditionalKeyDescriptorPayload: Codable {
  var kind: CNRAdditionalKeyDescriptorKind
  var style: CNRContactFormatterStylePayload?
}

struct CNRContactFetchRequestPayload: Codable {
  var keysToFetch: [CNRContactKey]
  var extraDescriptors: [CNRAdditionalKeyDescriptorPayload]
  var predicate: CNRContactPredicatePayload?
  var mutableObjects: Bool
  var unifyResults: Bool
  var sortOrder: CNRContactSortOrder
}

func cnrAllContactKeys() -> [CNRContactKey] {
  [
    .identifier,
    .contactType,
    .namePrefix,
    .givenName,
    .middleName,
    .familyName,
    .previousFamilyName,
    .nameSuffix,
    .nickname,
    .organizationName,
    .departmentName,
    .jobTitle,
    .phoneticGivenName,
    .phoneticMiddleName,
    .phoneticFamilyName,
    .phoneticOrganizationName,
    .note,
    .imageData,
    .thumbnailImageData,
    .imageDataAvailable,
    .phoneNumbers,
    .emailAddresses,
    .postalAddresses,
    .dates,
    .urlAddresses,
    .contactRelations,
    .socialProfiles,
    .instantMessageAddresses,
    .birthday,
    .nonGregorianBirthday,
  ]
}

func cnrDefaultFetchKeys() -> [CNRContactKey] {
  [.givenName, .familyName, .organizationName]
}

func cnrMutableContactKeys() -> [CNRContactKey] {
  cnrAllContactKeys()
}

func cnrNativeSortOrder(_ sortOrder: CNRContactSortOrder) -> CNContactSortOrder {
  switch sortOrder {
  case .none:
    .none
  case .userDefault:
    .userDefault
  case .givenName:
    .givenName
  case .familyName:
    .familyName
  }
}

func cnrResolvedContactKeys(
  contactKeys: [CNRContactKey],
  extraDescriptors: [CNRAdditionalKeyDescriptorPayload]
) -> Set<CNRContactKey> {
  var keys = Set(contactKeys.isEmpty ? cnrDefaultFetchKeys() : contactKeys)
  keys.insert(.identifier)

  for descriptor in extraDescriptors {
    switch descriptor.kind {
    case .comparatorKeys:
      keys.formUnion([
        .namePrefix,
        .givenName,
        .middleName,
        .familyName,
        .organizationName,
        .phoneticGivenName,
        .phoneticMiddleName,
        .phoneticFamilyName,
        .phoneticOrganizationName,
      ])
    case .formatterRequiredKeys:
      switch descriptor.style ?? .fullName {
      case .fullName:
        keys.formUnion([
          .namePrefix,
          .givenName,
          .middleName,
          .familyName,
          .previousFamilyName,
          .nameSuffix,
          .nickname,
          .organizationName,
        ])
      case .phoneticFullName:
        keys.formUnion([
          .phoneticGivenName,
          .phoneticMiddleName,
          .phoneticFamilyName,
          .phoneticOrganizationName,
        ])
      }
    case .formatterNameOrder, .formatterDelimiter:
      keys.formUnion([
        .namePrefix,
        .givenName,
        .middleName,
        .familyName,
        .previousFamilyName,
        .nameSuffix,
        .nickname,
        .organizationName,
      ])
    case .vcardRequiredKeys:
      keys.formUnion(cnrAllContactKeys())
    }
  }

  return keys
}

func cnrKeyDescriptors(
  from contactKeys: [CNRContactKey],
  extraDescriptors: [CNRAdditionalKeyDescriptorPayload]
) -> [any CNKeyDescriptor] {
  let requestedKeys = contactKeys.isEmpty ? cnrDefaultFetchKeys() : contactKeys
  var descriptors: [any CNKeyDescriptor] = [CNContactIdentifierKey as NSString]
  var seen = Set<String>([CNContactIdentifierKey])

  for key in requestedKeys {
    let value = cnrContactKeyConstant(key) as String
    if seen.insert(value).inserted {
      descriptors.append(value as NSString)
    }
  }

  for descriptor in extraDescriptors {
    switch descriptor.kind {
    case .comparatorKeys:
      descriptors.append(CNContact.descriptorForAllComparatorKeys())
    case .formatterRequiredKeys:
      descriptors.append(
        CNContactFormatter.descriptorForRequiredKeys(
          for: cnrNativeContactFormatterStyle(descriptor.style ?? .fullName)))
    case .formatterNameOrder:
      descriptors.append(CNContactFormatter.descriptorForRequiredKeysForNameOrder)
    case .formatterDelimiter:
      descriptors.append(CNContactFormatter.descriptorForRequiredKeysForDelimiter)
    case .vcardRequiredKeys:
      descriptors.append(CNContactVCardSerialization.descriptorForRequiredKeys())
    }
  }

  return descriptors
}
