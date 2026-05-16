import Contacts
import Foundation

struct CNRGroupPayload: Codable {
  var identifier: String
  var name: String
}

struct CNRMutableGroupPayload: Codable {
  var identifier: String?
  var name: String?
}

func cnrEncodeGroup(_ group: CNGroup) -> CNRGroupPayload {
  CNRGroupPayload(identifier: group.identifier, name: group.name)
}

func cnrApplyMutableGroupPayload(_ payload: CNRMutableGroupPayload, to group: CNMutableGroup) {
  if let name = payload.name {
    group.name = name
  }
}
