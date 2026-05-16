import Contacts
import Foundation

enum CNRContactKey: String, Codable, Hashable {
    case identifier
    case contactType
    case givenName
    case familyName
    case organizationName
    case emailAddresses
    case phoneNumbers
    case postalAddresses
    case urlAddresses
    case birthday
}

enum CNRContactSortOrder: String, Codable {
    case none
    case userDefault
    case givenName
    case familyName
}

enum CNRContactType: String, Codable {
    case person
    case organization
}

enum CNRContainerType: String, Codable {
    case unassigned
    case local
    case exchange
    case cardDav
}

struct CNRLabeledValuePayload<Value: Codable>: Codable {
    var label: String?
    var value: Value
}

struct CNRPostalAddressPayload: Codable {
    var street: String
    var subLocality: String
    var city: String
    var subAdministrativeArea: String
    var state: String
    var postalCode: String
    var country: String
    var isoCountryCode: String
}

struct CNRDateComponentsPayload: Codable {
    var era: Int?
    var year: Int?
    var month: Int?
    var day: Int?
    var hour: Int?
    var minute: Int?
    var second: Int?
    var isLeapMonth: Bool?
}

struct CNRContactPayload: Codable {
    var identifier: String?
    var contactType: CNRContactType?
    var givenName: String?
    var familyName: String?
    var organizationName: String?
    var emailAddresses: [CNRLabeledValuePayload<String>]?
    var phoneNumbers: [CNRLabeledValuePayload<String>]?
    var postalAddresses: [CNRLabeledValuePayload<CNRPostalAddressPayload>]?
    var urlAddresses: [CNRLabeledValuePayload<String>]?
    var birthday: CNRDateComponentsPayload?
}

struct CNRContactFetchRequestPayload: Codable {
    var keysToFetch: [CNRContactKey]
    var mutableObjects: Bool
    var unifyResults: Bool
    var sortOrder: CNRContactSortOrder
}

struct CNRGroupPayload: Codable {
    var identifier: String
    var name: String
}

struct CNRContainerPayload: Codable {
    var identifier: String
    var name: String
    var containerType: CNRContainerType
}

enum CNRSaveOperationKind: String, Codable {
    case addContact
    case updateContact
    case deleteContact
}

struct CNRSaveOperationPayload: Codable {
    var kind: CNRSaveOperationKind
    var contact: CNRContactPayload
    var containerIdentifier: String?
}

struct CNRSaveRequestPayload: Codable {
    var operations: [CNRSaveOperationPayload]
    var transactionAuthor: String?
    var shouldRefetchContacts: Bool
}

func cnrDefaultFetchKeys() -> [CNRContactKey] {
    [.givenName, .familyName, .organizationName]
}

func cnrAllMutableKeys() -> [CNRContactKey] {
    [.identifier, .contactType, .givenName, .familyName, .organizationName, .emailAddresses, .phoneNumbers, .postalAddresses, .urlAddresses, .birthday]
}

func cnrKeyDescriptors(from keys: [CNRContactKey]) -> [NSString] {
    let requestedKeys = keys.isEmpty ? cnrDefaultFetchKeys() : keys
    var descriptors: [NSString] = []
    var seen = Set<String>()

    func append(_ value: String) {
        if seen.insert(value).inserted {
            descriptors.append(value as NSString)
        }
    }

    append(CNContactIdentifierKey)

    for key in requestedKeys {
        switch key {
        case .identifier:
            append(CNContactIdentifierKey)
        case .contactType:
            append(CNContactTypeKey)
        case .givenName:
            append(CNContactGivenNameKey)
        case .familyName:
            append(CNContactFamilyNameKey)
        case .organizationName:
            append(CNContactOrganizationNameKey)
        case .emailAddresses:
            append(CNContactEmailAddressesKey)
        case .phoneNumbers:
            append(CNContactPhoneNumbersKey)
        case .postalAddresses:
            append(CNContactPostalAddressesKey)
        case .urlAddresses:
            append(CNContactUrlAddressesKey)
        case .birthday:
            append(CNContactBirthdayKey)
        }
    }

    return descriptors
}

func cnrNativeSortOrder(_ sortOrder: CNRContactSortOrder) -> CNContactSortOrder {
    switch sortOrder {
    case .none:
        return .none
    case .userDefault:
        return .userDefault
    case .givenName:
        return .givenName
    case .familyName:
        return .familyName
    }
}

func cnrPayloadContactType(from contactType: CNContactType) -> CNRContactType {
    switch contactType {
    case .person:
        return .person
    case .organization:
        return .organization
    @unknown default:
        return .person
    }
}

func cnrNativeContactType(_ contactType: CNRContactType) -> CNContactType {
    switch contactType {
    case .person:
        return .person
    case .organization:
        return .organization
    }
}

func cnrPayloadContainerType(from containerType: CNContainerType) -> CNRContainerType {
    switch containerType {
    case .unassigned:
        return .unassigned
    case .local:
        return .local
    case .exchange:
        return .exchange
    case .cardDAV:
        return .cardDav
    @unknown default:
        return .unassigned
    }
}

func cnrEncodeDateComponents(_ components: DateComponents?) -> CNRDateComponentsPayload? {
    guard let components else { return nil }
    return CNRDateComponentsPayload(
        era: components.era,
        year: components.year,
        month: components.month,
        day: components.day,
        hour: components.hour,
        minute: components.minute,
        second: components.second,
        isLeapMonth: components.isLeapMonth
    )
}

func cnrDecodeDateComponents(_ payload: CNRDateComponentsPayload) -> DateComponents {
    var components = DateComponents()
    components.era = payload.era
    components.year = payload.year
    components.month = payload.month
    components.day = payload.day
    components.hour = payload.hour
    components.minute = payload.minute
    components.second = payload.second
    components.isLeapMonth = payload.isLeapMonth
    return components
}

func cnrEncodePostalAddress(_ address: CNPostalAddress) -> CNRPostalAddressPayload {
    CNRPostalAddressPayload(
        street: address.street,
        subLocality: address.subLocality,
        city: address.city,
        subAdministrativeArea: address.subAdministrativeArea,
        state: address.state,
        postalCode: address.postalCode,
        country: address.country,
        isoCountryCode: address.isoCountryCode
    )
}

func cnrDecodePostalAddress(_ payload: CNRPostalAddressPayload) -> CNPostalAddress {
    let address = CNMutablePostalAddress()
    address.street = payload.street
    address.subLocality = payload.subLocality
    address.city = payload.city
    address.subAdministrativeArea = payload.subAdministrativeArea
    address.state = payload.state
    address.postalCode = payload.postalCode
    address.country = payload.country
    address.isoCountryCode = payload.isoCountryCode
    return address.copy() as! CNPostalAddress
}

func cnrEncodeContact(
    _ contact: CNContact,
    requestedKeys: Set<CNRContactKey>
) -> CNRContactPayload {
    CNRContactPayload(
        identifier: contact.identifier,
        contactType: requestedKeys.contains(.contactType) ? cnrPayloadContactType(from: contact.contactType) : nil,
        givenName: requestedKeys.contains(.givenName) ? contact.givenName : nil,
        familyName: requestedKeys.contains(.familyName) ? contact.familyName : nil,
        organizationName: requestedKeys.contains(.organizationName) ? contact.organizationName : nil,
        emailAddresses: requestedKeys.contains(.emailAddresses) ? contact.emailAddresses.map {
            CNRLabeledValuePayload(label: $0.label, value: $0.value as String)
        } : nil,
        phoneNumbers: requestedKeys.contains(.phoneNumbers) ? contact.phoneNumbers.map {
            CNRLabeledValuePayload(label: $0.label, value: $0.value.stringValue)
        } : nil,
        postalAddresses: requestedKeys.contains(.postalAddresses) ? contact.postalAddresses.map {
            CNRLabeledValuePayload(label: $0.label, value: cnrEncodePostalAddress($0.value))
        } : nil,
        urlAddresses: requestedKeys.contains(.urlAddresses) ? contact.urlAddresses.map {
            CNRLabeledValuePayload(label: $0.label, value: $0.value as String)
        } : nil,
        birthday: requestedKeys.contains(.birthday) ? cnrEncodeDateComponents(contact.birthday) : nil
    )
}

func cnrApplyContactPayload(_ payload: CNRContactPayload, to contact: CNMutableContact) {
    if let contactType = payload.contactType {
        contact.contactType = cnrNativeContactType(contactType)
    }
    if let givenName = payload.givenName {
        contact.givenName = givenName
    }
    if let familyName = payload.familyName {
        contact.familyName = familyName
    }
    if let organizationName = payload.organizationName {
        contact.organizationName = organizationName
    }
    if let emailAddresses = payload.emailAddresses {
        contact.emailAddresses = emailAddresses.map {
            CNLabeledValue(label: $0.label, value: $0.value as NSString)
        }
    }
    if let phoneNumbers = payload.phoneNumbers {
        contact.phoneNumbers = phoneNumbers.map {
            CNLabeledValue(label: $0.label, value: CNPhoneNumber(stringValue: $0.value))
        }
    }
    if let postalAddresses = payload.postalAddresses {
        contact.postalAddresses = postalAddresses.map {
            CNLabeledValue(label: $0.label, value: cnrDecodePostalAddress($0.value))
        }
    }
    if let urlAddresses = payload.urlAddresses {
        contact.urlAddresses = urlAddresses.map {
            CNLabeledValue(label: $0.label, value: $0.value as NSString)
        }
    }
    if let birthday = payload.birthday {
        contact.birthday = cnrDecodeDateComponents(birthday)
    }
}

func cnrFetchMutableContact(store: CNContactStore, identifier: String) throws -> CNMutableContact {
    let contact = try store.unifiedContact(withIdentifier: identifier, keysToFetch: cnrKeyDescriptors(from: cnrAllMutableKeys()))
    return contact.mutableCopy() as! CNMutableContact
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
    let identifier: String? = cnrBorrow(store, as: CNContactStore.self).defaultContainerIdentifier()
    guard let identifier, !identifier.isEmpty else { return nil }
    return cnrCString(identifier)
}

@_cdecl("cn_store_groups_json")
public func cn_store_groups_json(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        cnrSetMessageError(outError, message: "missing CNContactStore")
        return nil
    }

    do {
        let groups = try cnrBorrow(store, as: CNContactStore.self)
            .groups(matching: nil)
            .map { CNRGroupPayload(identifier: $0.identifier, name: $0.name) }
        return cnrCString(try cnrEncodeJSON(groups))
    } catch {
        cnrSetError(outError, error)
        return nil
    }
}

@_cdecl("cn_store_containers_json")
public func cn_store_containers_json(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        cnrSetMessageError(outError, message: "missing CNContactStore")
        return nil
    }

    do {
        let containers = try cnrBorrow(store, as: CNContactStore.self)
            .containers(matching: nil)
            .map {
                CNRContainerPayload(
                    identifier: $0.identifier,
                    name: $0.name,
                    containerType: cnrPayloadContainerType(from: $0.type)
                )
            }
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
        let keyDescriptors = cnrKeyDescriptors(from: payload.keysToFetch)
        let request = CNContactFetchRequest(keysToFetch: keyDescriptors)
        request.mutableObjects = payload.mutableObjects
        request.unifyResults = payload.unifyResults
        request.sortOrder = cnrNativeSortOrder(payload.sortOrder)

        let requestedKeys = Set(payload.keysToFetch.isEmpty ? cnrDefaultFetchKeys() : payload.keysToFetch)
        var contacts: [CNRContactPayload] = []
        try cnrBorrow(store, as: CNContactStore.self).enumerateContacts(with: request) { contact, stop in
            contacts.append(cnrEncodeContact(contact, requestedKeys: requestedKeys))
            if limit > 0 && contacts.count >= limit {
                stop.pointee = true
            }
        }

        return cnrCString(try cnrEncodeJSON(contacts))
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
        let keyDescriptors = cnrKeyDescriptors(from: keys)
        let contactStore = cnrBorrow(store, as: CNContactStore.self)
        do {
            let contact = try contactStore.unifiedContact(withIdentifier: String(cString: identifier), keysToFetch: keyDescriptors)
            let payload = cnrEncodeContact(contact, requestedKeys: requestedKeys)
            return cnrCString(try cnrEncodeJSON(Optional(payload)))
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
                let contact = CNMutableContact()
                cnrApplyContactPayload(operation.contact, to: contact)
                saveRequest.add(contact, toContainerWithIdentifier: operation.containerIdentifier)
            case .updateContact:
                guard let identifier = operation.contact.identifier else {
                    throw NSError(
                        domain: "contacts-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "updateContact requires identifier"]
                    )
                }
                let contact = try cnrFetchMutableContact(store: contactStore, identifier: identifier)
                cnrApplyContactPayload(operation.contact, to: contact)
                saveRequest.update(contact)
            case .deleteContact:
                guard let identifier = operation.contact.identifier else {
                    throw NSError(
                        domain: "contacts-rs",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "deleteContact requires identifier"]
                    )
                }
                let contact = try cnrFetchMutableContact(store: contactStore, identifier: identifier)
                saveRequest.delete(contact)
            }
        }

        try contactStore.execute(saveRequest)
        return CNR_OK
    } catch {
        cnrSetError(outError, error)
        return CNR_ERROR
    }
}
