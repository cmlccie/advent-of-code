// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "day-7",
    products: [
        .executable(name: "part1", targets: ["part1"]),
        .executable(name: "part2", targets: ["part2"]),
    ],
    targets: [
        .executableTarget(name: "part1"),
        .executableTarget(name: "part2"),
    ]
)
