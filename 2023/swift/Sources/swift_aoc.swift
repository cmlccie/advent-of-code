import ArgumentParser

// Advent of Code 2023 - Swift
@main
struct SwiftAOC: ParsableCommand {
    static var configuration = CommandConfiguration(
        abstract: "Advent of Code 2023 - Swift",
        subcommands: [Day7.self]
    )
}
