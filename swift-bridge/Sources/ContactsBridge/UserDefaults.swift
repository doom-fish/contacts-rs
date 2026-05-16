import Contacts
import Foundation

@_cdecl("cn_contacts_user_defaults_shared")
public func cn_contacts_user_defaults_shared() -> UnsafeMutableRawPointer {
  cnrRetain(CNContactsUserDefaults.shared())
}

@_cdecl("cn_contacts_user_defaults_release")
public func cn_contacts_user_defaults_release(_ defaults: UnsafeMutableRawPointer?) {
  guard let defaults else { return }
  cnrRelease(defaults)
}

@_cdecl("cn_contacts_user_defaults_sort_order")
public func cn_contacts_user_defaults_sort_order(_ defaults: UnsafeMutableRawPointer?) -> Int32 {
  guard let defaults else { return -1 }
  return Int32(cnrBorrow(defaults, as: CNContactsUserDefaults.self).sortOrder.rawValue)
}

@_cdecl("cn_contacts_user_defaults_country_code")
public func cn_contacts_user_defaults_country_code(
  _ defaults: UnsafeMutableRawPointer?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let defaults else {
    cnrSetMessageError(outError, message: "missing CNContactsUserDefaults")
    return nil
  }

  return cnrCString(cnrBorrow(defaults, as: CNContactsUserDefaults.self).countryCode)
}
