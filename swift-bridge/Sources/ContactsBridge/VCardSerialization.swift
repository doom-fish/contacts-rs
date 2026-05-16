import Contacts
import Foundation

@_cdecl("cn_contact_vcard_data_from_contacts_json")
public func cn_contact_vcard_data_from_contacts_json(
  _ contactsJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  do {
    let payloads = try cnrDecodeJSON(contactsJSON, as: [CNRContactPayload].self)
    let contacts = payloads.map { cnrMutableContact(from: $0) as CNContact }
    let data = try CNContactVCardSerialization.data(with: contacts)
    return cnrCString(data.base64EncodedString())
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_contact_vcard_contacts_from_base64")
public func cn_contact_vcard_contacts_from_base64(
  _ base64Data: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let base64Data else {
    cnrSetMessageError(outError, message: "missing vCard payload")
    return nil
  }

  do {
    guard let data = Data(base64Encoded: String(cString: base64Data)) else {
      throw NSError(
        domain: "contacts-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "invalid base64 vCard payload"]
      )
    }
    let contacts = try CNContactVCardSerialization.contacts(with: data)
    let payloads = contacts.map { cnrEncodeContact($0, requestedKeys: Set(cnrAllContactKeys())) }
    return cnrCString(try cnrEncodeJSON(payloads))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}
