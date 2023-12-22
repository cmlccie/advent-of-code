import XCTest

@testable import day8

class Part1Tests: XCTestCase {

    func testPart1Example1() {
        let result = day8.part1(inputPath: "Tests/part1_example1.txt")
        XCTAssertEqual(result, 2)
    }

    func testPart1Example2() {
        let result = day8.part1(inputPath: "Tests/part1_example2.txt")
        XCTAssertEqual(result, 6)
    }

    func testDirections() {
        let directions = day8.Directions("LLR")
        let firstNineDirections = Array(directions.prefix(9))
        let expected: [Character] = ["L", "L", "R", "L", "L", "R", "L", "L", "R"]
        XCTAssertEqual(firstNineDirections, expected)
    }

    func testParseInputDirections() {
        let input = day8.getInput(from: "Tests/part1_example1.txt")
        let (directions, _) = day8.parseInput(input)
        XCTAssertEqual(directions.directions, "RL")
    }

    func testParseInputFile() {
        let input = day8.getInput(from: "input.txt")
        let (directions, network) = day8.parseInput(input)

        XCTAssertEqual(
            directions.directions,
            "LRRLRLRRLRRRLRLRLRRLRRRLRRRLRRLRRRLRLRLRLRLRLRLRRRLRRLRRRLLLLRRRLRLLLRRRLLRLLRRRLRRRLRLRRLRRRLRRRLLRRRLRLRRRLLRRRLRLLRRRLRRLLRLRLRLRRRLRLLRLRLRRRLRLLRLRLRRRLLRRRLRRLRRRLRLRRLRLRRLRLRRLRRRLLRRRLLLRRRLLRRLRRLRRLRLLRRLRRRLRRLRLRLRRLRRLLLRRLRLRRRLRRRLRRRLLLRLRRRLLRRRLRLLRRRR"
        )

        XCTAssertEqual(network.nodes.count, 702)

        let nodeRegex = ##/(?<value>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)/##
        let lines = input.components(separatedBy: "\n")
        for line in lines[2...] {
            guard line.count > 0 else {
                continue
            }
            if let parsedLine = try? nodeRegex.wholeMatch(in: line) {
                let node = network.nodes[String(parsedLine.value)]!
                XCTAssertEqual(node.value, String(parsedLine.value))
                XCTAssertEqual(node.left?.value, String(parsedLine.left))
                XCTAssertEqual(node.right?.value, String(parsedLine.right))
            }
            else {
                fatalError("Could not parse line: \(line)")
            }
        }
    }

    func testPart1Solution() {
        let answer = day8.part1(inputPath: "input.txt")
        XCTAssertEqual(answer, 19_783)
    }
}
