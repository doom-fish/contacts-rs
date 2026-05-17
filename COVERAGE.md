# Contacts.framework coverage audit

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped / deferred

> Scope note: this crate now has one Swift bridge file, one Rust module, at least one example, and at least one test for each requested logical area. Extra public framework surface that is not part of the requested split is called out explicitly below.

| Area | API surface | Status | Notes |
| --- | --- | --- | --- |
| Store | `CNContactStore` creation, authorization, request-access, `CNEntityType`, default container, current history token | ✅ | Implemented in `src/store.rs` + `swift-bridge/Sources/ContactsBridge/Store.swift`. |
| Store | Contact enumeration, limited fetches, identifier fetch, save execution | ✅ | Uses JSON payload bridging through the Swift layer. |
| Store | Group/container fetches and save-request helpers for contacts, groups, members, subgroups | ✅ | Includes `CNSaveRequest` wrappers for add/update/delete/member/subgroup operations. |
| Store | `unifiedMeContactWithKeysToFetch:` | ✅ | Bridged through Objective-C selector dispatch from Swift so it works even when the imported Swift surface is awkward on this toolchain. |
| Contact | `CNContactType`, `CNContactSortOrder`, `CNContact` value snapshot fields | ✅ | Covers names, organization, phonetics, note, images, dates, relations, social profiles, and instant-message addresses. |
| Contact | `isKeyAvailable:`, `areKeysAvailable:`, `localizedStringForKey:` | ✅ | Implemented as safe Rust helpers over fetched-key metadata and Swift localization calls. |
| Contact | `descriptorForAllComparatorKeys` and `CNContact+NSItemProvider` | ✅ | Exposed via `CNContact::descriptor_for_all_comparator_keys()` plus item-provider type-identifier/data helpers. |
| Contact | `comparatorForNameSortOrder:`, `isUnifiedWithContactWithIdentifier:` | 🟡 | These behavior-only selectors remain modeled indirectly; the top-level public declaration audit in `COVERAGE_AUDIT.md` is now exhaustive. |
| MutableContact | `CNMutableContact` writable properties | ✅ | Includes clear flags for image/birthday/non-Gregorian birthday plus array setters for multi-value properties. |
| Group | `CNGroup`, `CNMutableGroup`, group fetch/save operations | ✅ | Includes subgroup/member save operations through `CNSaveRequest`. |
| Container | `CNContainer`, `CNContainerType`, container fetches | ✅ | Includes predicate-backed fetches. |
| FetchRequest | `CNContactFetchRequest` init, predicate, keys, mutable/unify/sort flags | ✅ | Added predicate support plus generic key-descriptor helpers for formatter/comparator/vCard fetches. |
| FetchRequest | `CNFetchRequest` abstract base class and `CNKeyDescriptor` protocol | ✅ | Exposed as `contacts::CNFetchRequest` plus `contacts::CNKeyDescriptor` builders for contact and change-history fetch requests, including raw `NSString` key descriptors. |
| FormatAndPrint | `CNContactFormatter` required-key descriptors, string formatting, attributed formatting, name order, delimiter | ✅ | Returned attributed strings include the property metadata exported by Contacts. |
| FormatAndPrint | `CNPostalAddressFormatter` string/attributed formatting and style selection | ✅ | Implemented via in-memory `CNPostalAddress` conversion through the Swift bridge. |
| ChangeNotifications | `CNContactStoreDidChangeNotification` | ✅ | Exposed as `contact_store_did_change_notification_name()`. |
| ChangeNotifications | `CNChangeHistoryFetchRequest`, `CNFetchResult`, `CNChangeHistoryEvent`, `CNChangeHistoryEventVisitor` Rust models | ✅ | The Rust-side request/result/event types and visitor trait are present and covered by tests/examples. |
| ChangeNotifications | `enumeratorForChangeHistoryFetchRequest:error:` and event bridging | ✅ | Implemented via Objective-C selector dispatch from Swift, then encoded into flattened Rust `CNChangeHistoryEvent` values plus the returned history token. |
| Properties | `CNLabeledValue`, `CNPhoneNumber`, `CNPostalAddress`, `CNMutablePostalAddress`, `CNInstantMessageAddress`, `CNSocialProfile`, `CNDateComponents` | ✅ | Safe Rust value wrappers with serde support, conversions, and localization helpers. |
| Properties | `CNContactProperty` | 🟡 | The payload type exists, but picker-produced property payloads are not yet returned by the bridge. |
| Properties | Property/localized label/service helpers | ✅ | Covers `CNContact.localizedString(forKey:)`, `CNLabeledValue.localizedString(forLabel:)`, postal/social/instant-message localization helpers. |
| Properties | Raw `NSString * const` property/service/label families | ✅ | Exhaustively exposed via typed Rust enums/helpers for keys, labels, services, error metadata, and relation labels. |
| Predicates | `CNContact+Predicates` factories | ✅ | Name, email, phone, identifier, group, and container predicates are covered. |
| Predicates | `CNGroup+Predicates` factories | ✅ | Identifier, subgroup, and container predicates are covered. |
| Predicates | `CNContainer+Predicates` factories | ✅ | Identifier, contact, and group predicates are covered. |
| VCardSerialization | `CNContactVCardSerialization` required-key descriptor, serialize, deserialize | ✅ | Round-trips `CNContact` values to/from vCard bytes. |
| ContactRelation | `CNContactRelation` value type | ✅ | Safe Rust wrapper plus example/test coverage. |
| ContactRelation | Contact-relation label constant family | ✅ | Exhaustively exposed via `contacts::CNContactRelationLabel`. |
| UserDefaults | `CNContactsUserDefaults` | ✅ | Implemented in `src/user_defaults.rs` + `swift-bridge/Sources/ContactsBridge/UserDefaults.swift`. |
| Error | `CNErrorCode`, `CNErrorDomain`, and `CNErrorUserInfo*` constants | ✅ | `contacts::CNErrorCode`, `contacts::contacts_error_domain()`, and `contacts::CNErrorUserInfoKey` provide typed coverage for Contacts error metadata. |
