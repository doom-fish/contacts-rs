// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "ContactsBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "ContactsBridge",
            type: .static,
            targets: ["ContactsBridge"])
    ],
    targets: [
        .target(
            name: "ContactsBridge",
            path: "Sources/ContactsBridge",
            publicHeadersPath: "include")
    ]
)
