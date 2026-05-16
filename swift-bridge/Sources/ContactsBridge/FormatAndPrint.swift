import Contacts
import Foundation

enum CNRContactFormatterStylePayload: String, Codable {
  case fullName
  case phoneticFullName
}

enum CNRContactDisplayNameOrderPayload: String, Codable {
  case userDefault
  case givenNameFirst
  case familyNameFirst
}

enum CNRPostalAddressFormatterStylePayload: String, Codable {
  case mailingAddress
}

struct CNRAttributedStringRunPayload: Codable {
  var location: Int
  var length: Int
  var value: String
  var property: String?
  var localizedPropertyName: String?
}

struct CNRAttributedStringPayload: Codable {
  var string: String
  var runs: [CNRAttributedStringRunPayload]
}

func cnrNativeContactFormatterStyle(_ style: CNRContactFormatterStylePayload)
  -> CNContactFormatterStyle
{
  switch style {
  case .fullName:
    .fullName
  case .phoneticFullName:
    .phoneticFullName
  }
}

func cnrPayloadDisplayNameOrder(from order: CNContactDisplayNameOrder)
  -> CNRContactDisplayNameOrderPayload
{
  switch order {
  case .userDefault:
    .userDefault
  case .givenNameFirst:
    .givenNameFirst
  case .familyNameFirst:
    .familyNameFirst
  @unknown default:
    .userDefault
  }
}

func cnrNativePostalAddressFormatterStyle(_ style: CNRPostalAddressFormatterStylePayload)
  -> CNPostalAddressFormatterStyle
{
  switch style {
  case .mailingAddress:
    .mailingAddress
  }
}

func cnrMutableContact(from payload: CNRContactPayload) -> CNMutableContact {
  let mutable = CNMutableContact()
  let mutablePayload = CNRMutableContactPayload(
    identifier: payload.identifier,
    contactType: payload.contactType,
    namePrefix: payload.namePrefix,
    givenName: payload.givenName,
    middleName: payload.middleName,
    familyName: payload.familyName,
    previousFamilyName: payload.previousFamilyName,
    nameSuffix: payload.nameSuffix,
    nickname: payload.nickname,
    organizationName: payload.organizationName,
    departmentName: payload.departmentName,
    jobTitle: payload.jobTitle,
    phoneticGivenName: payload.phoneticGivenName,
    phoneticMiddleName: payload.phoneticMiddleName,
    phoneticFamilyName: payload.phoneticFamilyName,
    phoneticOrganizationName: payload.phoneticOrganizationName,
    note: payload.note,
    imageData: payload.imageData,
    clearImageData: false,
    phoneNumbers: payload.phoneNumbers,
    emailAddresses: payload.emailAddresses,
    postalAddresses: payload.postalAddresses,
    dates: payload.dates,
    urlAddresses: payload.urlAddresses,
    contactRelations: payload.contactRelations,
    socialProfiles: payload.socialProfiles,
    instantMessageAddresses: payload.instantMessageAddresses,
    birthday: payload.birthday,
    clearBirthday: false,
    nonGregorianBirthday: payload.nonGregorianBirthday,
    clearNonGregorianBirthday: false
  )
  cnrApplyMutableContactPayload(mutablePayload, to: mutable)
  return mutable
}

func cnrEncodeAttributedString(_ attributedString: NSAttributedString) -> CNRAttributedStringPayload
{
  var runs: [CNRAttributedStringRunPayload] = []
  attributedString.enumerateAttributes(
    in: NSRange(location: 0, length: attributedString.length),
    options: []
  ) { attributes, range, _ in
    let value = attributedString.attributedSubstring(from: range).string
    let property =
      attributes[NSAttributedString.Key(rawValue: CNContactPropertyAttribute)] as? String
      ?? attributes[NSAttributedString.Key(rawValue: CNPostalAddressPropertyAttribute)] as? String
    let localizedPropertyName =
      attributes[
        NSAttributedString.Key(rawValue: CNPostalAddressLocalizedPropertyNameAttribute)
      ] as? String
    runs.append(
      CNRAttributedStringRunPayload(
        location: range.location,
        length: range.length,
        value: value,
        property: property,
        localizedPropertyName: localizedPropertyName
      )
    )
  }
  return CNRAttributedStringPayload(string: attributedString.string, runs: runs)
}

@_cdecl("cn_contact_formatter_string_from_contact_json")
public func cn_contact_formatter_string_from_contact_json(
  _ contactJSON: UnsafePointer<CChar>?,
  _ styleJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let contact = try cnrDecodeJSON(contactJSON, as: CNRContactPayload.self)
    let style = try cnrDecodeJSON(styleJSON, as: CNRContactFormatterStylePayload.self)
    let formatted = CNContactFormatter.string(
      from: cnrMutableContact(from: contact), style: cnrNativeContactFormatterStyle(style))
    guard let formatted else { return nil }
    return cnrCString(formatted)
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_contact_formatter_attributed_string_from_contact_json")
public func cn_contact_formatter_attributed_string_from_contact_json(
  _ contactJSON: UnsafePointer<CChar>?,
  _ styleJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let contact = try cnrDecodeJSON(contactJSON, as: CNRContactPayload.self)
    let style = try cnrDecodeJSON(styleJSON, as: CNRContactFormatterStylePayload.self)
    guard
      let attributed = CNContactFormatter.attributedString(
        from: cnrMutableContact(from: contact),
        style: cnrNativeContactFormatterStyle(style),
        defaultAttributes: nil
      )
    else {
      return cnrCString("null")
    }
    return cnrCString(try cnrEncodeJSON(Optional(cnrEncodeAttributedString(attributed))))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_contact_formatter_name_order_from_contact_json")
public func cn_contact_formatter_name_order_from_contact_json(
  _ contactJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
  do {
    let contact = try cnrDecodeJSON(contactJSON, as: CNRContactPayload.self)
    let order = CNContactFormatter.nameOrder(for: cnrMutableContact(from: contact))
    switch cnrPayloadDisplayNameOrder(from: order) {
    case .userDefault:
      return 0
    case .givenNameFirst:
      return 1
    case .familyNameFirst:
      return 2
    }
  } catch {
    cnrSetError(outError, error)
    return -1
  }
}

@_cdecl("cn_contact_formatter_delimiter_for_contact_json")
public func cn_contact_formatter_delimiter_for_contact_json(
  _ contactJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let contact = try cnrDecodeJSON(contactJSON, as: CNRContactPayload.self)
    return cnrCString(CNContactFormatter.delimiter(for: cnrMutableContact(from: contact)))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_postal_address_formatter_string_from_postal_address_json")
public func cn_postal_address_formatter_string_from_postal_address_json(
  _ addressJSON: UnsafePointer<CChar>?,
  _ styleJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let address = try cnrDecodeJSON(addressJSON, as: CNRPostalAddressPayload.self)
    let style = try cnrDecodeJSON(styleJSON, as: CNRPostalAddressFormatterStylePayload.self)
    return cnrCString(
      CNPostalAddressFormatter.string(
        from: cnrDecodePostalAddress(address),
        style: cnrNativePostalAddressFormatterStyle(style)
      )
    )
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_postal_address_formatter_attributed_string_from_postal_address_json")
public func cn_postal_address_formatter_attributed_string_from_postal_address_json(
  _ addressJSON: UnsafePointer<CChar>?,
  _ styleJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let address = try cnrDecodeJSON(addressJSON, as: CNRPostalAddressPayload.self)
    let style = try cnrDecodeJSON(styleJSON, as: CNRPostalAddressFormatterStylePayload.self)
    let attributed = CNPostalAddressFormatter.attributedString(
      from: cnrDecodePostalAddress(address),
      style: cnrNativePostalAddressFormatterStyle(style),
      withDefaultAttributes: [:]
    )
    return cnrCString(try cnrEncodeJSON(cnrEncodeAttributedString(attributed)))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}
