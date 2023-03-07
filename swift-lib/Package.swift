// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "swift-lib",
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "swift-lib",
            type: .static,
            targets: ["swift-lib"]),
    ],
    dependencies: [
        .package(url: "https://github.com/Brendonovich/swift-rs", from: "1.0.0")
    ],
    targets: [
        .target(
            name: "swift-lib",
            dependencies: [.product(name: "SwiftRs", package: "swift-rs")],
            path: "src"
        ),
    ]
)
