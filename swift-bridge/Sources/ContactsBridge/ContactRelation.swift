import Contacts
import Foundation

struct CNRContactRelationPayload: Codable {
  var name: String
}

func cnrEncodeContactRelation(_ relation: CNContactRelation) -> CNRContactRelationPayload {
  CNRContactRelationPayload(name: relation.name)
}

func cnrDecodeContactRelation(_ payload: CNRContactRelationPayload) -> CNContactRelation {
  CNContactRelation(name: payload.name)
}
