import Contacts
import Foundation

struct CNRMutableContactPayload: Codable {
  var identifier: String?
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
  var clearImageData: Bool
  var phoneNumbers: [CNRLabeledValuePayload<CNRPhoneNumberPayload>]?
  var emailAddresses: [CNRLabeledValuePayload<String>]?
  var postalAddresses: [CNRLabeledValuePayload<CNRPostalAddressPayload>]?
  var dates: [CNRLabeledValuePayload<CNRDateComponentsPayload>]?
  var urlAddresses: [CNRLabeledValuePayload<String>]?
  var contactRelations: [CNRLabeledValuePayload<CNRContactRelationPayload>]?
  var socialProfiles: [CNRLabeledValuePayload<CNRSocialProfilePayload>]?
  var instantMessageAddresses: [CNRLabeledValuePayload<CNRInstantMessageAddressPayload>]?
  var birthday: CNRDateComponentsPayload?
  var clearBirthday: Bool
  var nonGregorianBirthday: CNRDateComponentsPayload?
  var clearNonGregorianBirthday: Bool
}

func cnrApplyMutableContactPayload(
  _ payload: CNRMutableContactPayload, to contact: CNMutableContact
) {
  if let contactType = payload.contactType {
    contact.contactType = cnrNativeContactType(contactType)
  }
  if let namePrefix = payload.namePrefix {
    contact.namePrefix = namePrefix
  }
  if let givenName = payload.givenName {
    contact.givenName = givenName
  }
  if let middleName = payload.middleName {
    contact.middleName = middleName
  }
  if let familyName = payload.familyName {
    contact.familyName = familyName
  }
  if let previousFamilyName = payload.previousFamilyName {
    contact.previousFamilyName = previousFamilyName
  }
  if let nameSuffix = payload.nameSuffix {
    contact.nameSuffix = nameSuffix
  }
  if let nickname = payload.nickname {
    contact.nickname = nickname
  }
  if let organizationName = payload.organizationName {
    contact.organizationName = organizationName
  }
  if let departmentName = payload.departmentName {
    contact.departmentName = departmentName
  }
  if let jobTitle = payload.jobTitle {
    contact.jobTitle = jobTitle
  }
  if let phoneticGivenName = payload.phoneticGivenName {
    contact.phoneticGivenName = phoneticGivenName
  }
  if let phoneticMiddleName = payload.phoneticMiddleName {
    contact.phoneticMiddleName = phoneticMiddleName
  }
  if let phoneticFamilyName = payload.phoneticFamilyName {
    contact.phoneticFamilyName = phoneticFamilyName
  }
  if let phoneticOrganizationName = payload.phoneticOrganizationName {
    contact.phoneticOrganizationName = phoneticOrganizationName
  }
  if let note = payload.note {
    contact.note = note
  }
  if payload.clearImageData {
    contact.imageData = nil
  } else if let imageData = payload.imageData {
    contact.imageData = imageData
  }
  if let phoneNumbers = payload.phoneNumbers {
    contact.phoneNumbers = phoneNumbers.map {
      CNLabeledValue(label: $0.label, value: cnrDecodePhoneNumber($0.value))
    }
  }
  if let emailAddresses = payload.emailAddresses {
    contact.emailAddresses = emailAddresses.map {
      CNLabeledValue(label: $0.label, value: $0.value as NSString)
    }
  }
  if let postalAddresses = payload.postalAddresses {
    contact.postalAddresses = postalAddresses.map {
      CNLabeledValue(label: $0.label, value: cnrDecodePostalAddress($0.value))
    }
  }
  if let dates = payload.dates {
    contact.dates = dates.map {
      CNLabeledValue(label: $0.label, value: cnrDecodeNSDateComponents($0.value))
    }
  }
  if let urlAddresses = payload.urlAddresses {
    contact.urlAddresses = urlAddresses.map {
      CNLabeledValue(label: $0.label, value: $0.value as NSString)
    }
  }
  if let contactRelations = payload.contactRelations {
    contact.contactRelations = contactRelations.map {
      CNLabeledValue(label: $0.label, value: cnrDecodeContactRelation($0.value))
    }
  }
  if let socialProfiles = payload.socialProfiles {
    contact.socialProfiles = socialProfiles.map {
      CNLabeledValue(label: $0.label, value: cnrDecodeSocialProfile($0.value))
    }
  }
  if let instantMessageAddresses = payload.instantMessageAddresses {
    contact.instantMessageAddresses = instantMessageAddresses.map {
      CNLabeledValue(label: $0.label, value: cnrDecodeInstantMessageAddress($0.value))
    }
  }
  if payload.clearBirthday {
    contact.birthday = nil
  } else if let birthday = payload.birthday {
    contact.birthday = cnrDecodeDateComponents(birthday)
  }
  if payload.clearNonGregorianBirthday {
    contact.nonGregorianBirthday = nil
  } else if let nonGregorianBirthday = payload.nonGregorianBirthday {
    contact.nonGregorianBirthday = cnrDecodeDateComponents(nonGregorianBirthday)
  }
}
