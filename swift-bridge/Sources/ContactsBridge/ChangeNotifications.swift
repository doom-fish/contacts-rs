import Contacts
import Foundation

struct CNRFetchResultPayload<Value: Encodable>: Encodable {
  var value: Value
  var currentHistoryToken: Data
}

struct CNRChangeHistoryFetchRequestPayload: Codable {
  var startingToken: Data?
  var additionalContactKeys: [CNRContactKey]
  var additionalKeyDescriptors: [CNRAdditionalKeyDescriptorPayload]
  var shouldUnifyResults: Bool
  var mutableObjects: Bool
  var includeGroupChanges: Bool
  var excludedTransactionAuthors: [String]
}

enum CNRChangeHistoryEventPayload: Encodable {
  case dropEverything
  case addContact(contact: CNRContactPayload, containerIdentifier: String?)
  case updateContact(contact: CNRContactPayload)
  case deleteContact(contactIdentifier: String)
  case addGroup(group: CNRGroupPayload, containerIdentifier: String)
  case updateGroup(group: CNRGroupPayload)
  case deleteGroup(groupIdentifier: String)
  case addMemberToGroup(member: CNRContactPayload, group: CNRGroupPayload)
  case removeMemberFromGroup(member: CNRContactPayload, group: CNRGroupPayload)
  case addSubgroupToGroup(subgroup: CNRGroupPayload, group: CNRGroupPayload)
  case removeSubgroupFromGroup(subgroup: CNRGroupPayload, group: CNRGroupPayload)

  private enum CodingKeys: String, CodingKey {
    case kind
    case contact
    case containerIdentifier
    case contactIdentifier
    case group
    case subgroup
    case groupIdentifier
    case member
  }

  private enum Kind: String, Codable {
    case dropEverything
    case addContact
    case updateContact
    case deleteContact
    case addGroup
    case updateGroup
    case deleteGroup
    case addMemberToGroup
    case removeMemberFromGroup
    case addSubgroupToGroup
    case removeSubgroupFromGroup
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    switch self {
    case .dropEverything:
      try container.encode(Kind.dropEverything, forKey: .kind)
    case .addContact(let contact, let containerIdentifier):
      try container.encode(Kind.addContact, forKey: .kind)
      try container.encode(contact, forKey: .contact)
      try container.encode(containerIdentifier, forKey: .containerIdentifier)
    case .updateContact(let contact):
      try container.encode(Kind.updateContact, forKey: .kind)
      try container.encode(contact, forKey: .contact)
    case .deleteContact(let contactIdentifier):
      try container.encode(Kind.deleteContact, forKey: .kind)
      try container.encode(contactIdentifier, forKey: .contactIdentifier)
    case .addGroup(let group, let containerIdentifier):
      try container.encode(Kind.addGroup, forKey: .kind)
      try container.encode(group, forKey: .group)
      try container.encode(containerIdentifier, forKey: .containerIdentifier)
    case .updateGroup(let group):
      try container.encode(Kind.updateGroup, forKey: .kind)
      try container.encode(group, forKey: .group)
    case .deleteGroup(let groupIdentifier):
      try container.encode(Kind.deleteGroup, forKey: .kind)
      try container.encode(groupIdentifier, forKey: .groupIdentifier)
    case .addMemberToGroup(let member, let group):
      try container.encode(Kind.addMemberToGroup, forKey: .kind)
      try container.encode(member, forKey: .member)
      try container.encode(group, forKey: .group)
    case .removeMemberFromGroup(let member, let group):
      try container.encode(Kind.removeMemberFromGroup, forKey: .kind)
      try container.encode(member, forKey: .member)
      try container.encode(group, forKey: .group)
    case .addSubgroupToGroup(let subgroup, let group):
      try container.encode(Kind.addSubgroupToGroup, forKey: .kind)
      try container.encode(subgroup, forKey: .subgroup)
      try container.encode(group, forKey: .group)
    case .removeSubgroupFromGroup(let subgroup, let group):
      try container.encode(Kind.removeSubgroupFromGroup, forKey: .kind)
      try container.encode(subgroup, forKey: .subgroup)
      try container.encode(group, forKey: .group)
    }
  }
}

@_cdecl("cn_contact_store_did_change_notification_name")
public func cn_contact_store_did_change_notification_name() -> UnsafeMutablePointer<CChar>? {
  cnrCString(Notification.Name.CNContactStoreDidChange.rawValue)
}

private typealias CNRChangeHistoryEnumeratorIMP = @convention(c) (
  AnyObject, Selector, CNChangeHistoryFetchRequest, UnsafeMutablePointer<NSError?>?
) -> Unmanaged<AnyObject>?

func cnrEncodeChangeHistoryEvent(
  _ event: CNChangeHistoryEvent,
  requestedKeys: Set<CNRContactKey>
) -> CNRChangeHistoryEventPayload {
  switch event {
  case is CNChangeHistoryDropEverythingEvent:
    .dropEverything
  case let event as CNChangeHistoryAddContactEvent:
    .addContact(
      contact: cnrEncodeContact(event.contact, requestedKeys: requestedKeys),
      containerIdentifier: event.containerIdentifier)
  case let event as CNChangeHistoryUpdateContactEvent:
    .updateContact(contact: cnrEncodeContact(event.contact, requestedKeys: requestedKeys))
  case let event as CNChangeHistoryDeleteContactEvent:
    .deleteContact(contactIdentifier: event.contactIdentifier)
  case let event as CNChangeHistoryAddGroupEvent:
    .addGroup(group: cnrEncodeGroup(event.group), containerIdentifier: event.containerIdentifier)
  case let event as CNChangeHistoryUpdateGroupEvent:
    .updateGroup(group: cnrEncodeGroup(event.group))
  case let event as CNChangeHistoryDeleteGroupEvent:
    .deleteGroup(groupIdentifier: event.groupIdentifier)
  case let event as CNChangeHistoryAddMemberToGroupEvent:
    .addMemberToGroup(
      member: cnrEncodeContact(event.member, requestedKeys: requestedKeys),
      group: cnrEncodeGroup(event.group))
  case let event as CNChangeHistoryRemoveMemberFromGroupEvent:
    .removeMemberFromGroup(
      member: cnrEncodeContact(event.member, requestedKeys: requestedKeys),
      group: cnrEncodeGroup(event.group))
  case let event as CNChangeHistoryAddSubgroupToGroupEvent:
    .addSubgroupToGroup(
      subgroup: cnrEncodeGroup(event.subgroup),
      group: cnrEncodeGroup(event.group))
  case let event as CNChangeHistoryRemoveSubgroupFromGroupEvent:
    .removeSubgroupFromGroup(
      subgroup: cnrEncodeGroup(event.subgroup),
      group: cnrEncodeGroup(event.group))
  default:
    .dropEverything
  }
}

@_cdecl("cn_store_fetch_change_history_json")
public func cn_store_fetch_change_history_json(
  _ store: UnsafeMutableRawPointer?,
  _ requestJSON: UnsafePointer<CChar>?,
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let store else {
    cnrSetMessageError(outError, message: "missing CNContactStore")
    return nil
  }

  do {
    let payload = try cnrDecodeJSON(requestJSON, as: CNRChangeHistoryFetchRequestPayload.self)
    let request = CNChangeHistoryFetchRequest()
    request.startingToken = payload.startingToken
    request.additionalContactKeyDescriptors = cnrKeyDescriptors(
      from: payload.additionalContactKeys,
      extraDescriptors: payload.additionalKeyDescriptors
    )
    request.shouldUnifyResults = payload.shouldUnifyResults
    request.mutableObjects = payload.mutableObjects
    request.includeGroupChanges = payload.includeGroupChanges
    if !payload.excludedTransactionAuthors.isEmpty {
      request.excludedTransactionAuthors = payload.excludedTransactionAuthors
    }

    let requestedKeys = cnrResolvedContactKeys(
      contactKeys: payload.additionalContactKeys,
      extraDescriptors: payload.additionalKeyDescriptors
    )
    let contactStore = cnrBorrow(store, as: CNContactStore.self)
    let selector = NSSelectorFromString("enumeratorForChangeHistoryFetchRequest:error:")

    guard contactStore.responds(to: selector) else {
      throw NSError(
        domain: "contacts-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "CNContactStore does not respond to enumeratorForChangeHistoryFetchRequest:error:"]
      )
    }

    let function = unsafeBitCast(
      contactStore.method(for: selector),
      to: CNRChangeHistoryEnumeratorIMP.self
    )
    var nsError: NSError?
    let resultObject = function(contactStore, selector, request, &nsError)?.takeUnretainedValue()

    if let nsError {
      throw nsError
    }

    guard let resultObject = resultObject as? NSObject else {
      throw NSError(
        domain: "contacts-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "missing CNFetchResult from Contacts change-history fetch"]
      )
    }

    guard let enumerator = resultObject.value(forKey: "value") as? NSEnumerator else {
      throw NSError(
        domain: "contacts-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "CNFetchResult.value was not an NSEnumerator"]
      )
    }

    guard let currentHistoryToken = resultObject.value(forKey: "currentHistoryToken") as? Data
    else {
      throw NSError(
        domain: "contacts-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "CNFetchResult.currentHistoryToken was missing"]
      )
    }

    let events = enumerator.allObjects.compactMap { $0 as? CNChangeHistoryEvent }.map {
      cnrEncodeChangeHistoryEvent($0, requestedKeys: requestedKeys)
    }
    let result = CNRFetchResultPayload(value: events, currentHistoryToken: currentHistoryToken)
    return cnrCString(try cnrEncodeJSON(result))
  } catch {
    cnrSetError(outError, error)
    return nil
  }
}
