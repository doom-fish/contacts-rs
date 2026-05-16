# Changelog

## 0.2.1 - 2026-05-16

- Added `CNContactsUserDefaults` with safe accessors for the shared defaults sort order and country code.
- Added public `CNFetchRequest` and `CNKeyDescriptor` wrappers plus builder helpers for contact and change-history fetch requests.
- Added `CNContact` `NSItemProvider` helpers for type identifiers and in-memory item-provider round-tripping.
- Added typed `CNErrorCode` mappings, a thirteenth numbered example, and integration coverage for the new APIs.

## 0.2.0 - 2026-05-16

- Split the Swift bridge into logical files for Store, Contact, MutableContact, Group, Container, FetchRequest, FormatAndPrint, ChangeNotifications, Properties, Predicates, VCardSerialization, and ContactRelation.
- Reworked the Rust API into matching modules while keeping a compatibility `types` re-export surface.
- Expanded `CNContact` / `CNMutableContact` coverage to include phonetic names, notes, images, multiple date fields, relations, social profiles, and instant-message addresses.
- Added formatter wrappers, predicate builders, history request types, notification helpers, and vCard round-tripping.
- Added twelve numbered examples plus per-area integration tests.
- Added a framework coverage audit in `COVERAGE.md`.

## 0.1.0 - 2026-05-16

- Initial release.
- Added safe Rust bindings for `CNContactStore`, `CNContact`, `CNMutableContact`, `CNContactFetchRequest`, `CNGroup`, `CNContainer`, and `CNSaveRequest`.
- Added a non-interactive smoke example that reports authorization status and enumerates up to five contacts when already authorized.
