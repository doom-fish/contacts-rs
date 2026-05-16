import Contacts
import Foundation

enum CNRContactPredicatePayload: Codable {
  case matchingName(name: String)
  case matchingEmailAddress(emailAddress: String)
  case matchingPhoneNumber(phoneNumber: CNRPhoneNumberPayload)
  case withIdentifiers(identifiers: [String])
  case inGroupWithIdentifier(groupIdentifier: String)
  case inContainerWithIdentifier(containerIdentifier: String)

  private enum CodingKeys: String, CodingKey {
    case kind
    case name
    case emailAddress
    case phoneNumber
    case identifiers
    case groupIdentifier
    case containerIdentifier
  }

  private enum Kind: String, Codable {
    case matchingName
    case matchingEmailAddress
    case matchingPhoneNumber
    case withIdentifiers
    case inGroupWithIdentifier
    case inContainerWithIdentifier
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    switch try container.decode(Kind.self, forKey: .kind) {
    case .matchingName:
      self = .matchingName(name: try container.decode(String.self, forKey: .name))
    case .matchingEmailAddress:
      self = .matchingEmailAddress(
        emailAddress: try container.decode(String.self, forKey: .emailAddress))
    case .matchingPhoneNumber:
      self = .matchingPhoneNumber(
        phoneNumber: try container.decode(CNRPhoneNumberPayload.self, forKey: .phoneNumber))
    case .withIdentifiers:
      self = .withIdentifiers(
        identifiers: try container.decode([String].self, forKey: .identifiers))
    case .inGroupWithIdentifier:
      self = .inGroupWithIdentifier(
        groupIdentifier: try container.decode(String.self, forKey: .groupIdentifier))
    case .inContainerWithIdentifier:
      self = .inContainerWithIdentifier(
        containerIdentifier: try container.decode(String.self, forKey: .containerIdentifier))
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    switch self {
    case .matchingName(let name):
      try container.encode(Kind.matchingName, forKey: .kind)
      try container.encode(name, forKey: .name)
    case .matchingEmailAddress(let emailAddress):
      try container.encode(Kind.matchingEmailAddress, forKey: .kind)
      try container.encode(emailAddress, forKey: .emailAddress)
    case .matchingPhoneNumber(let phoneNumber):
      try container.encode(Kind.matchingPhoneNumber, forKey: .kind)
      try container.encode(phoneNumber, forKey: .phoneNumber)
    case .withIdentifiers(let identifiers):
      try container.encode(Kind.withIdentifiers, forKey: .kind)
      try container.encode(identifiers, forKey: .identifiers)
    case .inGroupWithIdentifier(let groupIdentifier):
      try container.encode(Kind.inGroupWithIdentifier, forKey: .kind)
      try container.encode(groupIdentifier, forKey: .groupIdentifier)
    case .inContainerWithIdentifier(let containerIdentifier):
      try container.encode(Kind.inContainerWithIdentifier, forKey: .kind)
      try container.encode(containerIdentifier, forKey: .containerIdentifier)
    }
  }
}

enum CNRGroupPredicatePayload: Codable {
  case withIdentifiers(identifiers: [String])
  case subgroupsInGroupWithIdentifier(parentGroupIdentifier: String)
  case groupsInContainerWithIdentifier(containerIdentifier: String)

  private enum CodingKeys: String, CodingKey {
    case kind
    case identifiers
    case parentGroupIdentifier
    case containerIdentifier
  }

  private enum Kind: String, Codable {
    case withIdentifiers
    case subgroupsInGroupWithIdentifier
    case groupsInContainerWithIdentifier
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    switch try container.decode(Kind.self, forKey: .kind) {
    case .withIdentifiers:
      self = .withIdentifiers(
        identifiers: try container.decode([String].self, forKey: .identifiers))
    case .subgroupsInGroupWithIdentifier:
      self = .subgroupsInGroupWithIdentifier(
        parentGroupIdentifier: try container.decode(String.self, forKey: .parentGroupIdentifier))
    case .groupsInContainerWithIdentifier:
      self = .groupsInContainerWithIdentifier(
        containerIdentifier: try container.decode(String.self, forKey: .containerIdentifier))
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    switch self {
    case .withIdentifiers(let identifiers):
      try container.encode(Kind.withIdentifiers, forKey: .kind)
      try container.encode(identifiers, forKey: .identifiers)
    case .subgroupsInGroupWithIdentifier(let parentGroupIdentifier):
      try container.encode(Kind.subgroupsInGroupWithIdentifier, forKey: .kind)
      try container.encode(parentGroupIdentifier, forKey: .parentGroupIdentifier)
    case .groupsInContainerWithIdentifier(let containerIdentifier):
      try container.encode(Kind.groupsInContainerWithIdentifier, forKey: .kind)
      try container.encode(containerIdentifier, forKey: .containerIdentifier)
    }
  }
}

enum CNRContainerPredicatePayload: Codable {
  case withIdentifiers(identifiers: [String])
  case containerOfContactWithIdentifier(contactIdentifier: String)
  case containerOfGroupWithIdentifier(groupIdentifier: String)

  private enum CodingKeys: String, CodingKey {
    case kind
    case identifiers
    case contactIdentifier
    case groupIdentifier
  }

  private enum Kind: String, Codable {
    case withIdentifiers
    case containerOfContactWithIdentifier
    case containerOfGroupWithIdentifier
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    switch try container.decode(Kind.self, forKey: .kind) {
    case .withIdentifiers:
      self = .withIdentifiers(
        identifiers: try container.decode([String].self, forKey: .identifiers))
    case .containerOfContactWithIdentifier:
      self = .containerOfContactWithIdentifier(
        contactIdentifier: try container.decode(String.self, forKey: .contactIdentifier))
    case .containerOfGroupWithIdentifier:
      self = .containerOfGroupWithIdentifier(
        groupIdentifier: try container.decode(String.self, forKey: .groupIdentifier))
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    switch self {
    case .withIdentifiers(let identifiers):
      try container.encode(Kind.withIdentifiers, forKey: .kind)
      try container.encode(identifiers, forKey: .identifiers)
    case .containerOfContactWithIdentifier(let contactIdentifier):
      try container.encode(Kind.containerOfContactWithIdentifier, forKey: .kind)
      try container.encode(contactIdentifier, forKey: .contactIdentifier)
    case .containerOfGroupWithIdentifier(let groupIdentifier):
      try container.encode(Kind.containerOfGroupWithIdentifier, forKey: .kind)
      try container.encode(groupIdentifier, forKey: .groupIdentifier)
    }
  }
}

func cnrNativePredicate(_ predicate: CNRContactPredicatePayload) -> NSPredicate {
  switch predicate {
  case .matchingName(let name):
    CNContact.predicateForContacts(matchingName: name)
  case .matchingEmailAddress(let emailAddress):
    CNContact.predicateForContacts(matchingEmailAddress: emailAddress)
  case .matchingPhoneNumber(let phoneNumber):
    CNContact.predicateForContacts(matching: cnrDecodePhoneNumber(phoneNumber))
  case .withIdentifiers(let identifiers):
    CNContact.predicateForContacts(withIdentifiers: identifiers)
  case .inGroupWithIdentifier(let groupIdentifier):
    CNContact.predicateForContactsInGroup(withIdentifier: groupIdentifier)
  case .inContainerWithIdentifier(let containerIdentifier):
    CNContact.predicateForContactsInContainer(withIdentifier: containerIdentifier)
  }
}

func cnrNativePredicate(_ predicate: CNRGroupPredicatePayload) -> NSPredicate {
  switch predicate {
  case .withIdentifiers(let identifiers):
    CNGroup.predicateForGroups(withIdentifiers: identifiers)
  case .subgroupsInGroupWithIdentifier(let parentGroupIdentifier):
    CNGroup.predicateForSubgroupsInGroup(withIdentifier: parentGroupIdentifier)
  case .groupsInContainerWithIdentifier(let containerIdentifier):
    CNGroup.predicateForGroupsInContainer(withIdentifier: containerIdentifier)
  }
}

func cnrNativePredicate(_ predicate: CNRContainerPredicatePayload) -> NSPredicate {
  switch predicate {
  case .withIdentifiers(let identifiers):
    CNContainer.predicateForContainers(withIdentifiers: identifiers)
  case .containerOfContactWithIdentifier(let contactIdentifier):
    CNContainer.predicateForContainerOfContact(withIdentifier: contactIdentifier)
  case .containerOfGroupWithIdentifier(let groupIdentifier):
    CNContainer.predicateForContainerOfGroup(withIdentifier: groupIdentifier)
  }
}
