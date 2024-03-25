// swift-tools-version: 5.10

import PackageDescription

let package = Package(
    name: "swift_aoc",
    dependencies: [
        .package(url: "https://github.com/apple/swift-argument-parser.git", from: "1.2.0")
    ],
    targets: [
        .executableTarget(
            name: "swift_aoc",
            dependencies: [
                .product(name: "ArgumentParser", package: "swift-argument-parser")
            ]
        ),
        .testTarget(name: "tests", dependencies: ["swift_aoc"]),
    ]
)
