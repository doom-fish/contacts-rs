import Contacts
import Foundation

enum CNRPostalAddressKey: String, Codable {
  case street
  case subLocality
  case city
  case subAdministrativeArea
  case state
  case postalCode
  case country
  case isoCountryCode
}

enum CNRInstantMessageAddressKey: String, Codable {
  case username
  case service
}

enum CNRSocialProfileKey: String, Codable {
  case urlString
  case username
  case userIdentifier
  case service
}

struct CNRLabeledValuePayload<Value: Codable>: Codable {
  var identifier: String?
  var label: String?
  var value: Value
}

struct CNRPhoneNumberPayload: Codable {
  var stringValue: String
}

struct CNRPostalAddressPayload: Codable {
  var street: String
  var subLocality: String
  var city: String
  var subAdministrativeArea: String
  var state: String
  var postalCode: String
  var country: String
  var isoCountryCode: String
}

struct CNRInstantMessageAddressPayload: Codable {
  var username: String
  var service: String
}

struct CNRSocialProfilePayload: Codable {
  var urlString: String
  var username: String
  var userIdentifier: String
  var service: String
}

struct CNRDateComponentsPayload: Codable {
  var era: Int?
  var year: Int?
  var month: Int?
  var day: Int?
  var hour: Int?
  var minute: Int?
  var second: Int?
  var isLeapMonth: Bool?
  var calendarIdentifier: String?
}

func cnrEncodePhoneNumber(_ phoneNumber: CNPhoneNumber) -> CNRPhoneNumberPayload {
  CNRPhoneNumberPayload(stringValue: phoneNumber.stringValue)
}

func cnrDecodePhoneNumber(_ payload: CNRPhoneNumberPayload) -> CNPhoneNumber {
  CNPhoneNumber(stringValue: payload.stringValue)
}

func cnrCalendarIdentifierString(_ identifier: Calendar.Identifier?) -> String? {
  guard let identifier else { return nil }
  switch identifier {
  case .gregorian:
    return "gregorian"
  case .buddhist:
    return "buddhist"
  case .chinese:
    return "chinese"
  case .coptic:
    return "coptic"
  case .ethiopicAmeteMihret:
    return "ethiopicAmeteMihret"
  case .ethiopicAmeteAlem:
    return "ethiopicAmeteAlem"
  case .hebrew:
    return "hebrew"
  case .iso8601:
    return "iso8601"
  case .indian:
    return "indian"
  case .islamic:
    return "islamic"
  case .islamicCivil:
    return "islamicCivil"
  case .japanese:
    return "japanese"
  case .persian:
    return "persian"
  case .republicOfChina:
    return "republicOfChina"
  case .islamicTabular:
    return "islamicTabular"
  case .islamicUmmAlQura:
    return "islamicUmmAlQura"
  default:
    return String(describing: identifier)
  }
}

func cnrCalendarIdentifier(_ rawValue: String) -> Calendar.Identifier? {
  switch rawValue {
  case "gregorian":
    return .gregorian
  case "buddhist":
    return .buddhist
  case "chinese":
    return .chinese
  case "coptic":
    return .coptic
  case "ethiopicAmeteMihret":
    return .ethiopicAmeteMihret
  case "ethiopicAmeteAlem":
    return .ethiopicAmeteAlem
  case "hebrew":
    return .hebrew
  case "iso8601":
    return .iso8601
  case "indian":
    return .indian
  case "islamic":
    return .islamic
  case "islamicCivil":
    return .islamicCivil
  case "japanese":
    return .japanese
  case "persian":
    return .persian
  case "republicOfChina":
    return .republicOfChina
  case "islamicTabular":
    return .islamicTabular
  case "islamicUmmAlQura":
    return .islamicUmmAlQura
  default:
    return nil
  }
}

func cnrEncodeDateComponents(_ components: DateComponents?) -> CNRDateComponentsPayload? {
  guard let components else { return nil }
  return CNRDateComponentsPayload(
    era: components.era,
    year: components.year,
    month: components.month,
    day: components.day,
    hour: components.hour,
    minute: components.minute,
    second: components.second,
    isLeapMonth: components.isLeapMonth,
    calendarIdentifier: cnrCalendarIdentifierString(components.calendar?.identifier)
  )
}

func cnrOptionalDateComponent(_ value: Int) -> Int? {
  value == NSDateComponentUndefined ? nil : value
}

func cnrEncodeDateComponents(_ components: NSDateComponents?) -> CNRDateComponentsPayload? {
  guard let components else { return nil }
  return CNRDateComponentsPayload(
    era: cnrOptionalDateComponent(components.era),
    year: cnrOptionalDateComponent(components.year),
    month: cnrOptionalDateComponent(components.month),
    day: cnrOptionalDateComponent(components.day),
    hour: cnrOptionalDateComponent(components.hour),
    minute: cnrOptionalDateComponent(components.minute),
    second: cnrOptionalDateComponent(components.second),
    isLeapMonth: components.isLeapMonth,
    calendarIdentifier: cnrCalendarIdentifierString(components.calendar?.identifier)
  )
}

func cnrDecodeDateComponents(_ payload: CNRDateComponentsPayload) -> DateComponents {
  var components = DateComponents()
  components.era = payload.era
  components.year = payload.year
  components.month = payload.month
  components.day = payload.day
  components.hour = payload.hour
  components.minute = payload.minute
  components.second = payload.second
  components.isLeapMonth = payload.isLeapMonth
  if let calendarIdentifier = payload.calendarIdentifier,
    let identifier = cnrCalendarIdentifier(calendarIdentifier)
  {
    components.calendar = Calendar(identifier: identifier)
  }
  return components
}

func cnrDecodeNSDateComponents(_ payload: CNRDateComponentsPayload) -> NSDateComponents {
  cnrDecodeDateComponents(payload) as NSDateComponents
}

func cnrEncodePostalAddress(_ address: CNPostalAddress) -> CNRPostalAddressPayload {
  CNRPostalAddressPayload(
    street: address.street,
    subLocality: address.subLocality,
    city: address.city,
    subAdministrativeArea: address.subAdministrativeArea,
    state: address.state,
    postalCode: address.postalCode,
    country: address.country,
    isoCountryCode: address.isoCountryCode
  )
}

func cnrDecodePostalAddress(_ payload: CNRPostalAddressPayload) -> CNPostalAddress {
  let address = CNMutablePostalAddress()
  address.street = payload.street
  address.subLocality = payload.subLocality
  address.city = payload.city
  address.subAdministrativeArea = payload.subAdministrativeArea
  address.state = payload.state
  address.postalCode = payload.postalCode
  address.country = payload.country
  address.isoCountryCode = payload.isoCountryCode
  return address.copy() as! CNPostalAddress
}

func cnrEncodeInstantMessageAddress(_ address: CNInstantMessageAddress)
  -> CNRInstantMessageAddressPayload
{
  CNRInstantMessageAddressPayload(username: address.username, service: address.service)
}

func cnrDecodeInstantMessageAddress(_ payload: CNRInstantMessageAddressPayload)
  -> CNInstantMessageAddress
{
  CNInstantMessageAddress(username: payload.username, service: payload.service)
}

func cnrEncodeSocialProfile(_ profile: CNSocialProfile) -> CNRSocialProfilePayload {
  CNRSocialProfilePayload(
    urlString: profile.urlString,
    username: profile.username,
    userIdentifier: profile.userIdentifier,
    service: profile.service
  )
}

func cnrDecodeSocialProfile(_ payload: CNRSocialProfilePayload) -> CNSocialProfile {
  CNSocialProfile(
    urlString: payload.urlString,
    username: payload.username,
    userIdentifier: payload.userIdentifier,
    service: payload.service
  )
}

func cnrPostalAddressKeyConstant(_ key: CNRPostalAddressKey) -> String {
  switch key {
  case .street:
    CNPostalAddressStreetKey
  case .subLocality:
    CNPostalAddressSubLocalityKey
  case .city:
    CNPostalAddressCityKey
  case .subAdministrativeArea:
    CNPostalAddressSubAdministrativeAreaKey
  case .state:
    CNPostalAddressStateKey
  case .postalCode:
    CNPostalAddressPostalCodeKey
  case .country:
    CNPostalAddressCountryKey
  case .isoCountryCode:
    CNPostalAddressISOCountryCodeKey
  }
}

func cnrInstantMessageKeyConstant(_ key: CNRInstantMessageAddressKey) -> String {
  switch key {
  case .username:
    CNInstantMessageAddressUsernameKey
  case .service:
    CNInstantMessageAddressServiceKey
  }
}

func cnrSocialProfileKeyConstant(_ key: CNRSocialProfileKey) -> String {
  switch key {
  case .urlString:
    CNSocialProfileURLStringKey
  case .username:
    CNSocialProfileUsernameKey
  case .userIdentifier:
    CNSocialProfileUserIdentifierKey
  case .service:
    CNSocialProfileServiceKey
  }
}

@_cdecl("cn_labeled_value_localized_string_for_label")
public func cn_labeled_value_localized_string_for_label(
  _ label: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
  guard let label else { return nil }
  return cnrCString(CNLabeledValue<NSString>.localizedString(forLabel: String(cString: label)))
}

@_cdecl("cn_postal_address_localized_string_for_key")
public func cn_postal_address_localized_string_for_key(
  _ keyJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let key = try cnrDecodeJSON(keyJSON, as: CNRPostalAddressKey.self)
    return cnrCString(CNPostalAddress.localizedString(forKey: cnrPostalAddressKeyConstant(key)))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_instant_message_localized_string_for_key")
public func cn_instant_message_localized_string_for_key(
  _ keyJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let key = try cnrDecodeJSON(keyJSON, as: CNRInstantMessageAddressKey.self)
    return cnrCString(
      CNInstantMessageAddress.localizedString(forKey: cnrInstantMessageKeyConstant(key)))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_instant_message_localized_string_for_service")
public func cn_instant_message_localized_string_for_service(
  _ service: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
  guard let service else { return nil }
  return cnrCString(CNInstantMessageAddress.localizedString(forService: String(cString: service)))
}

@_cdecl("cn_social_profile_localized_string_for_key")
public func cn_social_profile_localized_string_for_key(
  _ keyJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let key = try cnrDecodeJSON(keyJSON, as: CNRSocialProfileKey.self)
    return cnrCString(CNSocialProfile.localizedString(forKey: cnrSocialProfileKeyConstant(key)))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_social_profile_localized_string_for_service")
public func cn_social_profile_localized_string_for_service(
  _ service: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
  guard let service else { return nil }
  return cnrCString(CNSocialProfile.localizedString(forService: String(cString: service)))
}
