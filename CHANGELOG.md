# Changelog

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
