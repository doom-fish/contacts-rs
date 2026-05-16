import Contacts
import Foundation

public let CNR_OK: Int32 = 0
public let CNR_ERROR: Int32 = -1

@_cdecl("cn_string_free")
public func cn_string_free(_ string: UnsafeMutablePointer<CChar>?) {
  guard let string else { return }
  free(string)
}

@inline(__always)
public func cnrCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
  string.withCString { strdup($0) }
}

@inline(__always)
public func cnrRetain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func cnrBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
  Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
public func cnrRelease(_ ptr: UnsafeMutableRawPointer) {
  Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

public struct CNRErrorPayload: Codable {
  public var domain: String
  public var code: Int
  public var message: String
}

public func cnrEncodeJSON<T: Encodable>(_ value: T) throws -> String {
  let encoder = JSONEncoder()
  let data = try encoder.encode(value)
  guard let string = String(data: data, encoding: .utf8) else {
    throw NSError(
      domain: "contacts-rs",
      code: -1,
      userInfo: [NSLocalizedDescriptionKey: "failed to encode JSON as UTF-8"]
    )
  }
  return string
}

public func cnrDecodeJSON<T: Decodable>(_ json: UnsafePointer<CChar>?, as _: T.Type) throws -> T {
  guard let json else {
    throw NSError(
      domain: "contacts-rs",
      code: -1,
      userInfo: [NSLocalizedDescriptionKey: "missing JSON payload"]
    )
  }

  let data = Data(String(cString: json).utf8)
  return try JSONDecoder().decode(T.self, from: data)
}

public func cnrErrorPayload(from error: Error) -> CNRErrorPayload {
  let nsError = error as NSError
  return CNRErrorPayload(
    domain: nsError.domain,
    code: nsError.code,
    message: nsError.localizedDescription
  )
}

public func cnrSetError(
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
  _ error: Error
) {
  guard let outError else { return }

  if let json = try? cnrEncodeJSON(cnrErrorPayload(from: error)) {
    outError.pointee = cnrCString(json)
  } else {
    outError.pointee = cnrCString((error as NSError).localizedDescription)
  }
}

public func cnrSetMessageError(
  _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
  message: String,
  domain: String = "contacts-rs",
  code: Int = -1
) {
  guard let outError else { return }
  let payload = CNRErrorPayload(domain: domain, code: code, message: message)
  if let json = try? cnrEncodeJSON(payload) {
    outError.pointee = cnrCString(json)
  } else {
    outError.pointee = cnrCString(message)
  }
}
