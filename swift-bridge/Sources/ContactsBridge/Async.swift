import Contacts
import Foundation

// MARK: - requestAccess(for:completionHandler:) async wrapper
//
// CNContactStore.requestAccess(for:) is a completion-handler API that the
// Rust side wraps as a Future via `RequestAccessFuture`.  The @_cdecl thunk
// launches a Swift Task so the callback fires asynchronously, off the calling
// thread, without blocking.

@_cdecl("cn_request_access_async")
public func cn_request_access_async(
    _ entityTypeRawValue: Int32,
    _ cb: @convention(c) (Bool, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    Task {
        do {
            let entityType = try cnrEntityType(entityTypeRawValue)
            let store = CNContactStore()
            let granted = try await store.requestAccess(for: entityType)
            cb(granted, nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(false, $0, ctx) }
        }
    }
}

// MARK: - enumerateContacts(with:usingBlock:) async wrapper
//
// CNContactStore.enumerateContacts(with:usingBlock:) runs a block once per
// contact.  The Rust side wraps this as a one-shot Future that collects *all*
// matching contacts into a Vec.
//
// For incremental / streaming processing the Tier-2 pattern (exposing results
// as a `Stream`) is the natural fit; that will be added in a future release.
//
// The result JSON is heap-allocated via `cnrCString`; Rust frees it through
// `cn_string_free` inside `parse_json_ptr`.

@_cdecl("cn_enumerate_contacts_async")
public func cn_enumerate_contacts_async(
    _ store: UnsafeMutableRawPointer?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ limit: Int,
    _ cb: @convention(c) (UnsafeMutablePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let store else {
        "missing CNContactStore".withCString { cb(nil, $0, ctx) }
        return
    }

    // Copy the request JSON *synchronously* before creating the Task.
    // The Rust caller may drop the CString as soon as this function returns
    // (i.e. before the Task body executes), so we must not capture the raw
    // pointer across the Task boundary.
    let requestJSONString: String
    do {
        let payload = try cnrDecodeJSON(requestJSON, as: CNRContactFetchRequestPayload.self)
        requestJSONString = try cnrEncodeJSON(payload)
    } catch {
        error.localizedDescription.withCString { cb(nil, $0, ctx) }
        return
    }

    Task.detached(priority: .userInitiated) {
        do {
            let payload = try requestJSONString.withCString { ptr in
                try cnrDecodeJSON(UnsafePointer(ptr), as: CNRContactFetchRequestPayload.self)
            }
            let contacts = try cnrFetchContacts(
                store: cnrBorrow(store, as: CNContactStore.self),
                requestPayload: payload,
                limit: limit
            )
            let json = try cnrEncodeJSON(contacts)
            // cnrCString returns a heap-allocated CString; Rust frees it via cn_string_free.
            cb(cnrCString(json), nil, ctx)
        } catch {
            error.localizedDescription.withCString { cb(nil, $0, ctx) }
        }
    }
}
