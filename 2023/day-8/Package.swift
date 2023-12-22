// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "day8",
    targets: [
        .executableTarget(name: "day8"),
        .testTarget(name: "day8Tests", dependencies: ["day8"]),
    ]
)
