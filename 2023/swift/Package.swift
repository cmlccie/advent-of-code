// swift-tools-version: 5.10

import PackageDescription

let package = Package(
    name: "swift_aoc",
    dependencies: [
        .package(url: "https://github.com/apple/swift-argument-parser.git", from: "1.2.0"),
        .package(url: "https://github.com/apple/swift-log.git", from: "1.5.4"),
    ],
    targets: [
        .executableTarget(
            name: "swift_aoc",
            dependencies: [
                .product(name: "ArgumentParser", package: "swift-argument-parser"),
                .product(name: "Logging", package: "swift-log"),
            ]
        ),
        .testTarget(name: "tests", dependencies: ["swift_aoc"]),
    ]
)
