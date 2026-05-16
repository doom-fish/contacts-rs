# Contacts.framework coverage audit

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped / deferred

> Scope note: this crate now has one Swift bridge file, one Rust module, at least one example, and at least one test for each requested logical area. Extra public framework surface that is not part of the requested split is called out explicitly below.

| Area | API surface | Status | Notes |
| --- | --- | --- | --- |
| Store | `CNContactStore` creation, authorization, request-access, default container, current history token | ✅ | Implemented in `src/store.rs` + `swift-bridge/Sources/ContactsBridge/Store.swift`. |
| Store | Contact enumeration, limited fetches, identifier fetch, save execution | ✅ | Uses JSON payload bridging through the Swift layer. |
| Store | Group/container fetches and save-request helpers for contacts, groups, members, subgroups | ✅ | Includes `CNSaveRequest` wrappers for add/update/delete/member/subgroup operations. |
| Store | `unifiedMeContactWithKeysToFetch:` | ✅ | Bridged through Objective-C selector dispatch from Swift so it works even when the imported Swift surface is awkward on this toolchain. |
| Contact | `CNContactType`, `CNContactSortOrder`, `CNContact` value snapshot fields | ✅ | Covers names, organization, phonetics, note, images, dates, relations, social profiles, and instant-message addresses. |
| Contact | `isKeyAvailable:`, `areKeysAvailable:`, `localizedStringForKey:` | ✅ | Implemented as safe Rust helpers over fetched-key metadata and Swift localization calls. |
| Contact | `descriptorForAllComparatorKeys` | ✅ | Exposed via `CNContact::descriptor_for_all_comparator_keys()`. |
| Contact | `comparatorForNameSortOrder:`, `isUnifiedWithContactWithIdentifier:` | 🟡 | Comparator/link-unification behavior is not yet surfaced as a native bridge call. |
| MutableContact | `CNMutableContact` writable properties | ✅ | Includes clear flags for image/birthday/non-Gregorian birthday plus array setters for multi-value properties. |
| Group | `CNGroup`, `CNMutableGroup`, group fetch/save operations | ✅ | Includes subgroup/member save operations through `CNSaveRequest`. |
| Container | `CNContainer`, `CNContainerType`, container fetches | ✅ | Includes predicate-backed fetches. |
| FetchRequest | `CNContactFetchRequest` init, predicate, keys, mutable/unify/sort flags | ✅ | Added predicate support plus extra descriptor support for formatter/comparator/vCard keys. |
| FetchRequest | `CNFetchRequest` abstract base class | ⏭️ | Mirrored by the concrete safe wrappers instead of a stand-alone Rust type. |
| FormatAndPrint | `CNContactFormatter` required-key descriptors, string formatting, attributed formatting, name order, delimiter | ✅ | Returned attributed strings include the property metadata exported by Contacts. |
| FormatAndPrint | `CNPostalAddressFormatter` string/attributed formatting and style selection | ✅ | Implemented via in-memory `CNPostalAddress` conversion through the Swift bridge. |
| ChangeNotifications | `CNContactStoreDidChangeNotification` | ✅ | Exposed as `contact_store_did_change_notification_name()`. |
| ChangeNotifications | `CNChangeHistoryFetchRequest`, `CNFetchResult`, `CNChangeHistoryEvent` Rust models | ✅ | The Rust-side request/result/event types are present and covered by tests/examples. |
| ChangeNotifications | `enumeratorForChangeHistoryFetchRequest:error:` and event bridging | ✅ | Implemented via Objective-C selector dispatch from Swift, then encoded into flattened Rust `CNChangeHistoryEvent` values plus the returned history token. |
| Properties | `CNLabeledValue`, `CNPhoneNumber`, `CNPostalAddress`, `CNInstantMessageAddress`, `CNSocialProfile`, `CNDateComponents` | ✅ | Safe Rust value wrappers with serde support and localization helpers. |
| Properties | `CNContactProperty` | 🟡 | The payload type exists, but picker-produced property payloads are not yet returned by the bridge. |
| Properties | Property/localized label/service helpers | ✅ | Covers `CNContact.localizedString(forKey:)`, `CNLabeledValue.localizedString(forLabel:)`, postal/social/instant-message localization helpers. |
| Properties | Raw `NSString * const` property/service/label families | 🟡 | The crate focuses on typed enums plus localization helpers; not every exported Objective-C constant is surfaced as a dedicated Rust constant yet. |
| Predicates | `CNContact+Predicates` factories | ✅ | Name, email, phone, identifier, group, and container predicates are covered. |
| Predicates | `CNGroup+Predicates` factories | ✅ | Identifier, subgroup, and container predicates are covered. |
| Predicates | `CNContainer+Predicates` factories | ✅ | Identifier, contact, and group predicates are covered. |
| VCardSerialization | `CNContactVCardSerialization` required-key descriptor, serialize, deserialize | ✅ | Round-trips `CNContact` values to/from vCard bytes. |
| ContactRelation | `CNContactRelation` value type | ✅ | Safe Rust wrapper plus example/test coverage. |
| ContactRelation | Contact-relation label constant family | 🟡 | Custom labels work, but raw exported relation constants are not all surfaced as dedicated Rust constants yet. |
| Extra public API | `CNContactsUserDefaults` | ⏭️ | Extra public framework area outside the requested logical split. |
| Extra public API | `CNContact+NSItemProvider` | ⏭️ | NSItemProvider integration category is outside this crate's scope. |
| Extra public API | `CNError.h` constant families | ⏭️ | The crate preserves `NSError` domain/code/message details but does not yet wrap every `CNError` constant as typed Rust API. |
