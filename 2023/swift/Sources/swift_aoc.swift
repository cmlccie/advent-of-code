import ArgumentParser
import Logging

// Advent of Code 2023 - Swift
@main
struct SwiftAOC: ParsableCommand {
    static var configuration = CommandConfiguration(
        abstract: "Advent of Code 2023 - Swift",
        subcommands: [Day7.self, Day8.self]
    )

    struct StandardArgs: ParsableArguments {
        @Argument(help: "Path to the input file.")
        var inputPath: String

        @Flag(help: "Log level.")
        var logLevel: Logger.Level = .notice
    }

    static func initLogging(args: StandardArgs) {
        LoggingSystem.bootstrap { label in
            var logHandler = StreamLogHandler.standardError(label: label)
            logHandler.logLevel = args.logLevel
            return logHandler
        }
    }
}

extension Logger.Level: EnumerableFlag {}
