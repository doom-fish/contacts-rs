import Contacts
import Foundation

enum CNRSaveOperationKind: String, Codable {
  case addContact
  case updateContact
  case deleteContact
  case addGroup
  case updateGroup
  case deleteGroup
  case addSubgroup
  case removeSubgroup
  case addMember
  case removeMember
}

struct CNRSaveOperationPayload: Codable {
  var kind: CNRSaveOperationKind
  var contact: CNRMutableContactPayload?
  var identifier: String?
  var group: CNRMutableGroupPayload?
  var containerIdentifier: String?
  var groupIdentifier: String?
  var subgroupIdentifier: String?
  var contactIdentifier: String?
}

struct CNRSaveRequestPayload: Codable {
  var operations: [CNRSaveOperationPayload]
  var transactionAuthor: String?
  var shouldRefetchContacts: Bool
}

func cnrFetchMutableContact(store: CNContactStore, identifier: String) throws -> CNMutableContact {
  let contact = try store.unifiedContact(
    withIdentifier: identifier,
    keysToFetch: cnrKeyDescriptors(from: cnrMutableContactKeys(), extraDescriptors: []))
  return contact.mutableCopy() as! CNMutableContact
}

func cnrFetchContact(store: CNContactStore, identifier: String) throws -> CNContact {
  try store.unifiedContact(
    withIdentifier: identifier, keysToFetch: [CNContactIdentifierKey as NSString])
}

func cnrFetchGroup(store: CNContactStore, identifier: String) throws -> CNGroup {
  guard
    let group = try store.groups(
      matching: CNGroup.predicateForGroups(withIdentifiers: [identifier])
    ).first
  else {
    throw NSError(
      domain: CNErrorDomain,
      code: 200,
      userInfo: [NSLocalizedDescriptionKey: "group does not exist"]
    )
  }
  return group
}

func cnrFetchContacts(
  store: CNContactStore,
  requestPayload: CNRContactFetchRequestPayload,
  limit: Int
) throws -> [CNRContactPayload] {
  let keyDescriptors = cnrKeyDescriptors(
    from: requestPayload.keysToFetch,
    extraDescriptors: requestPayload.extraDescriptors
  )
  let request = CNContactFetchRequest(keysToFetch: keyDescriptors)
  request.mutableObjects = requestPayload.mutableObjects
  request.unifyResults = requestPayload.unifyResults
  request.sortOrder = cnrNativeSortOrder(requestPayload.sortOrder)
  if let predicate = requestPayload.predicate {
    request.predicate = cnrNativePredicate(predicate)
  }

  let requestedKeys = cnrResolvedContactKeys(
    contactKeys: requestPayload.keysToFetch,
    extraDescriptors: requestPayload.extraDescriptors
  )

  var contacts: [CNRContactPayload] = []
  try store.enumerateContacts(with: request) { contact, stop in
    contacts.append(cnrEncodeContact(contact, requestedKeys: requestedKeys))
    if limit > 0 && contacts.count >= limit {
      stop.pointee = true
    }
  }
  return contacts
}

@_cdecl("cn_authorization_status")
public func cn_authorization_status() -> Int32 {
  Int32(CNContactStore.authorizationStatus(for: .contacts).rawValue)
}

@_cdecl("cn_request_access")
public func cn_request_access(
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  let semaphore = DispatchSemaphore(value: 0)
  let store = CNContactStore()
  var granted = false
  var capturedError: Error?

  store.requestAccess(for: .contacts) { didGrant, error in
    granted = didGrant
    capturedError = error
    semaphore.signal()
  }

  _ = semaphore.wait(timeout: .now() + .seconds(30))
  if let capturedError {
    cnrSetError(outError, capturedError)
  }
  return granted
}

@_cdecl("cn_store_new")
public func cn_store_new() -> UnsafeMutableRawPointer {
  cnrRetain(CNContactStore())
}

@_cdecl("cn_store_release")
public func cn_store_release(_ store: UnsafeMutableRawPointer?) {
  guard let store else { return }
  cnrRelease(store)
}

@_cdecl("cn_store_default_container_identifier")
public func cn_store_default_container_identifier(
  _ store: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else { return nil }
  let identifier = cnrBorrow(store, as: CNContactStore.self).defaultContainerIdentifier()
  guard !identifier.isEmpty else { return nil }
  return cnrCString(identifier)
}

@_cdecl("cn_store_current_history_token")
public func cn_store_current_history_token(
  _ store: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else { return nil }
  guard let token = cnrBorrow(store, as: CNContactStore.self).currentHistoryToken else {
    return nil
  }
  return cnrCString(token.base64EncodedString())
}

@_cdecl("cn_store_groups_json")
public func cn_store_groups_json(
  _ store: UnsafeMutableRawPointer?,
  _ predicateJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else {
    cnrSetMessageError(outError, message: "missing CNContactStore")
    return nil
  }

  do {
    let predicate = try predicateJSON.flatMap { pointer in
      try cnrDecodeJSON(pointer, as: CNRGroupPredicatePayload.self)
    }.map(cnrNativePredicate)
    let groups = try cnrBorrow(store, as: CNContactStore.self)
      .groups(matching: predicate)
      .map(cnrEncodeGroup)
    return cnrCString(try cnrEncodeJSON(groups))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_store_containers_json")
public func cn_store_containers_json(
  _ store: UnsafeMutableRawPointer?,
  _ predicateJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else {
    cnrSetMessageError(outError, message: "missing CNContactStore")
    return nil
  }

  do {
    let predicate = try predicateJSON.flatMap { pointer in
      try cnrDecodeJSON(pointer, as: CNRContainerPredicatePayload.self)
    }.map(cnrNativePredicate)
    let containers = try cnrBorrow(store, as: CNContactStore.self)
      .containers(matching: predicate)
      .map(cnrEncodeContainer)
    return cnrCString(try cnrEncodeJSON(containers))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_store_fetch_contacts_json")
public func cn_store_fetch_contacts_json(
  _ store: UnsafeMutableRawPointer?,
  _ requestJSON: UnsafePointer<CChar>?,
  _ limit: Int,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else {
    cnrSetMessageError(outError, message: "missing CNContactStore")
    return nil
  }

  do {
    let payload = try cnrDecodeJSON(requestJSON, as: CNRContactFetchRequestPayload.self)
    let contacts = try cnrFetchContacts(
      store: cnrBorrow(store, as: CNContactStore.self),
      requestPayload: payload,
      limit: limit
    )
    return cnrCString(try cnrEncodeJSON(contacts))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_store_fetch_contacts_result_json")
public func cn_store_fetch_contacts_result_json(
  _ store: UnsafeMutableRawPointer?,
  _ requestJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else {
    cnrSetMessageError(outError, message: "missing CNContactStore")
    return nil
  }

  do {
    let payload = try cnrDecodeJSON(requestJSON, as: CNRContactFetchRequestPayload.self)
    let contactStore = cnrBorrow(store, as: CNContactStore.self)
    let contacts = try cnrFetchContacts(store: contactStore, requestPayload: payload, limit: 0)
    let token = contactStore.currentHistoryToken ?? Data()
    let result = CNRFetchResultPayload(value: contacts, currentHistoryToken: token)
    return cnrCString(try cnrEncodeJSON(result))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_store_fetch_contact_by_identifier_json")
public func cn_store_fetch_contact_by_identifier_json(
  _ store: UnsafeMutableRawPointer?,
  _ identifier: UnsafePointer<CChar>?,
  _ keysJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else {
    cnrSetMessageError(outError, message: "missing CNContactStore")
    return nil
  }
  guard let identifier else {
    cnrSetMessageError(outError, message: "missing contact identifier")
    return nil
  }

  do {
    let keys = try cnrDecodeJSON(keysJSON, as: [CNRContactKey].self)
    let requestedKeys = Set(keys.isEmpty ? cnrDefaultFetchKeys() : keys)
    let keyDescriptors = cnrKeyDescriptors(from: keys, extraDescriptors: [])
    let contactStore = cnrBorrow(store, as: CNContactStore.self)
    do {
      let contact = try contactStore.unifiedContact(
        withIdentifier: String(cString: identifier), keysToFetch: keyDescriptors)
      return cnrCString(
        try cnrEncodeJSON(Optional(cnrEncodeContact(contact, requestedKeys: requestedKeys))))
    } catch {
      let nsError = error as NSError
      if nsError.domain == CNErrorDomain, nsError.code == 200 {
        return cnrCString("null")
      }
      throw error
    }
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

private typealias CNRUnifiedMeContactIMP = @convention(c) (
  AnyObject, Selector, NSArray, UnsafeMutablePointer<NSError?>?
) -> Unmanaged<AnyObject>?

@_cdecl("cn_store_unified_me_contact_json")
public func cn_store_unified_me_contact_json(
  _ store: UnsafeMutableRawPointer?,
  _ keysJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else {
    cnrSetMessageError(outError, message: "missing CNContactStore")
    return nil
  }

  do {
    let keys = try cnrDecodeJSON(keysJSON, as: [CNRContactKey].self)
    let requestedKeys = Set(keys.isEmpty ? cnrDefaultFetchKeys() : keys)
    let keyDescriptors = cnrKeyDescriptors(from: keys, extraDescriptors: [])
    let contactStore = cnrBorrow(store, as: CNContactStore.self)
    let selector = NSSelectorFromString("unifiedMeContactWithKeysToFetch:error:")

    guard contactStore.responds(to: selector) else {
      throw NSError(
        domain: "contacts-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "CNContactStore does not respond to unifiedMeContactWithKeysToFetch:error:"]
      )
    }

    let function = unsafeBitCast(contactStore.method(for: selector), to: CNRUnifiedMeContactIMP.self)
    var nsError: NSError?
    let contact = function(contactStore, selector, keyDescriptors as NSArray, &nsError)?
      .takeUnretainedValue() as? CNContact

    if let nsError {
      if nsError.domain == CNErrorDomain, nsError.code == 200 {
        return cnrCString("null")
      }
      throw nsError
    }

    guard let contact else {
      return cnrCString("null")
    }

    return cnrCString(
      try cnrEncodeJSON(Optional(cnrEncodeContact(contact, requestedKeys: requestedKeys))))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}

@_cdecl("cn_store_execute_save_request")
public func cn_store_execute_save_request(
  _ store: UnsafeMutableRawPointer?,
  _ requestJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
  guard let store else {
    cnrSetMessageError(outError, message: "missing CNContactStore")
    return CNR_ERROR
  }

  do {
    let payload = try cnrDecodeJSON(requestJSON, as: CNRSaveRequestPayload.self)
    let contactStore = cnrBorrow(store, as: CNContactStore.self)
    let saveRequest = CNSaveRequest()
    saveRequest.transactionAuthor = payload.transactionAuthor
    saveRequest.shouldRefetchContacts = payload.shouldRefetchContacts

    for operation in payload.operations {
      switch operation.kind {
      case .addContact:
        guard let contactPayload = operation.contact else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [NSLocalizedDescriptionKey: "missing contact payload"])
        }
        let contact = CNMutableContact()
        cnrApplyMutableContactPayload(contactPayload, to: contact)
        saveRequest.add(contact, toContainerWithIdentifier: operation.containerIdentifier)
      case .updateContact:
        guard let contactPayload = operation.contact, let identifier = contactPayload.identifier
        else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [NSLocalizedDescriptionKey: "updateContact requires identifier"])
        }
        let contact = try cnrFetchMutableContact(store: contactStore, identifier: identifier)
        cnrApplyMutableContactPayload(contactPayload, to: contact)
        saveRequest.update(contact)
      case .deleteContact:
        guard let identifier = operation.identifier else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [NSLocalizedDescriptionKey: "deleteContact requires identifier"])
        }
        let contact = try cnrFetchMutableContact(store: contactStore, identifier: identifier)
        saveRequest.delete(contact)
      case .addGroup:
        guard let groupPayload = operation.group else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [NSLocalizedDescriptionKey: "missing group payload"])
        }
        let group = CNMutableGroup()
        cnrApplyMutableGroupPayload(groupPayload, to: group)
        saveRequest.add(group, toContainerWithIdentifier: operation.containerIdentifier)
      case .updateGroup:
        guard let groupPayload = operation.group, let identifier = groupPayload.identifier else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [NSLocalizedDescriptionKey: "updateGroup requires identifier"])
        }
        let existing =
          try cnrFetchGroup(store: contactStore, identifier: identifier).mutableCopy()
          as! CNMutableGroup
        cnrApplyMutableGroupPayload(groupPayload, to: existing)
        saveRequest.update(existing)
      case .deleteGroup:
        guard let identifier = operation.identifier else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [NSLocalizedDescriptionKey: "deleteGroup requires identifier"])
        }
        let group = try cnrFetchGroup(store: contactStore, identifier: identifier)
        saveRequest.delete(group.mutableCopy() as! CNMutableGroup)
      case .addSubgroup:
        guard let subgroupIdentifier = operation.subgroupIdentifier,
          let groupIdentifier = operation.groupIdentifier
        else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [
              NSLocalizedDescriptionKey: "addSubgroup requires subgroup and group identifiers"
            ])
        }
        saveRequest.addSubgroup(
          try cnrFetchGroup(store: contactStore, identifier: subgroupIdentifier),
          to: try cnrFetchGroup(store: contactStore, identifier: groupIdentifier))
      case .removeSubgroup:
        guard let subgroupIdentifier = operation.subgroupIdentifier,
          let groupIdentifier = operation.groupIdentifier
        else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [
              NSLocalizedDescriptionKey: "removeSubgroup requires subgroup and group identifiers"
            ])
        }
        saveRequest.removeSubgroup(
          try cnrFetchGroup(store: contactStore, identifier: subgroupIdentifier),
          from: try cnrFetchGroup(store: contactStore, identifier: groupIdentifier))
      case .addMember:
        guard let contactIdentifier = operation.contactIdentifier,
          let groupIdentifier = operation.groupIdentifier
        else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [
              NSLocalizedDescriptionKey: "addMember requires contact and group identifiers"
            ])
        }
        saveRequest.addMember(
          try cnrFetchContact(store: contactStore, identifier: contactIdentifier),
          to: try cnrFetchGroup(store: contactStore, identifier: groupIdentifier))
      case .removeMember:
        guard let contactIdentifier = operation.contactIdentifier,
          let groupIdentifier = operation.groupIdentifier
        else {
          throw NSError(
            domain: "contacts-rs", code: -1,
            userInfo: [
              NSLocalizedDescriptionKey: "removeMember requires contact and group identifiers"
            ])
        }
        saveRequest.removeMember(
          try cnrFetchContact(store: contactStore, identifier: contactIdentifier),
          from: try cnrFetchGroup(store: contactStore, identifier: groupIdentifier))
      }
    }

    try contactStore.execute(saveRequest)
    return CNR_OK
  } catch {
    cnrSetError(outError, error)
    return CNR_ERROR
  }
}
