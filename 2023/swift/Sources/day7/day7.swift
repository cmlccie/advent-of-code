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

            @OptionGroup var args: SwiftAOC.StandardArgs

            mutating func run() throws {
                initLogging(args: args)
                _ = try day7Part1(inputPath: args.inputPath)
            }
        }

        struct Part2: ParsableCommand {
            static var configuration = CommandConfiguration(abstract: "Day 7: Camel Cards - Part 2")

            @OptionGroup var args: SwiftAOC.StandardArgs

            mutating func run() throws {
                initLogging(args: args)
                _ = try day7Part2(inputPath: args.inputPath)
            }
        }
    }
}
