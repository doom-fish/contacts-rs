# Changelog

## [0.3.6] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.3.5] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.4] - 2026-05-19

- Bump MSRV from 1.70 to 1.76 to match fleet baseline.

## [0.3.3] - 2026-05-18

- Added concise rustdoc coverage to every public item in non-generated `src/` modules, including re-export module headers and public builder APIs.
- Reached 100.0% rustdoc documentation coverage for non-generated source (`cargo +nightly rustdoc --lib --all-features -- -Z unstable-options --show-coverage`).

## [0.3.2] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

## 0.3.1 - 2026-05-17

### Fixed

- Removed broken doc link to `pollster::block_on` in `AsyncCNContactStore` (was causing `cargo doc` warning).

## 0.3.0 - 2026-05-17

### Added — Async API (Tier 1)

New `async` Cargo feature enabling executor-agnostic `Future` wrappers for
`CNContactStore` completion-handler APIs.

| Apple API | Rust type | Notes |
|-----------|-----------|-------|
| `CNContactStore.requestAccess(for:completionHandler:)` | `RequestAccessFuture` | Completion handler → `Future<bool>` |
| `CNContactStore.enumerateContacts(with:usingBlock:)` | `EnumerateContactsFuture` | Collects all matching contacts into `Vec<CNContact>` |

The `async_api` module is gated behind `features = ["async"]`.  A
`Stream`-based Tier-2 wrapper for incremental enumeration will follow in a
future release.

Two new examples:
- `14_async_request_access` — async authorization request
- `15_async_enumerate_contacts` — async contact enumeration with limit

Sync-only APIs (`containers(matching:)`, `fetch_change_history`) have no async
wrapper — they are already instant on the calling thread.

Change notifications (`NotificationCenter`) are deferred to Tier 2 (Stream).

## 0.2.2 - 2026-05-17

- Reached 100% top-level `Contacts.framework` declaration coverage in `COVERAGE_AUDIT.md`.
- Added exhaustive typed wrappers for remaining Contacts constant families, including container/group keys, error-domain and user-info keys, phone/email/URL/date labels, instant-message/social-profile services, and every contact-relation label.
- Added `CNMutablePostalAddress`, `CNEntityType`, `CNChangeHistoryEventVisitor`, and raw `NSString` `CNKeyDescriptor` support.

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
