import Contacts
import Foundation

enum CNRContainerType: String, Codable {
  case unassigned
  case local
  case exchange
  case cardDav
}

struct CNRContainerPayload: Codable {
  var identifier: String
  var name: String
  var containerType: CNRContainerType
}

func cnrPayloadContainerType(from containerType: CNContainerType) -> CNRContainerType {
  switch containerType {
  case .unassigned:
    .unassigned
  case .local:
    .local
  case .exchange:
    .exchange
  case .cardDAV:
    .cardDav
  @unknown default:
    .unassigned
  }
}

func cnrEncodeContainer(_ container: CNContainer) -> CNRContainerPayload {
  CNRContainerPayload(
    identifier: container.identifier,
    name: container.name,
    containerType: cnrPayloadContainerType(from: container.type)
  )
}
