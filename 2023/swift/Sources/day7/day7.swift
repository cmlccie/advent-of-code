import ArgumentParser

/*-------------------------------------------------------------------------------------------------
  Day 7: Camel Cards
-------------------------------------------------------------------------------------------------*/

extension SwiftAOC {
    struct Day7: ParsableCommand {
        static var configuration = CommandConfiguration(
            abstract: "Day 7: Camel Cards",
            subcommands: [Part1.self, Part2.self]
        )

        struct Part1: ParsableCommand {
            static var configuration = CommandConfiguration(abstract: "Day 7: Camel Cards - Part 1")

            @Argument(help: "Path to the input file.")
            var inputPath: String

            mutating func run() throws {
                _ = day7Part1(inputPath: inputPath)
            }
        }

        struct Part2: ParsableCommand {
            static var configuration = CommandConfiguration(abstract: "Day 7: Camel Cards - Part 2")

            @Argument(help: "Path to the input file.")
            var inputPath: String

            mutating func run() throws {
                _ = day7Part2(inputPath: inputPath)
            }
        }
    }
}
