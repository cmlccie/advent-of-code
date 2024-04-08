import XCTest

@testable import swift_aoc

/*-------------------------------------------------------------------------------------------------
  Day 8: Haunted Wasteland
-------------------------------------------------------------------------------------------------*/

/*--------------------------------------------------------------------------------------
  Part1:
--------------------------------------------------------------------------------------*/

class Day8Part1: XCTestCase {
    override func setUp() {
        super.setUp()
        XCTAssert(loggingSystemBootstrapped)
    }

    func testPart1Example1() throws {
        let result = try day8Part1(inputPath: "../inputs/day8/part1_example_1.txt")
        XCTAssertEqual(result, 2)
    }

    func testPart1Example2() throws {
        let result = try day8Part1(inputPath: "../inputs/day8/part1_example_2.txt")
        XCTAssertEqual(result, 6)
    }

    func testPart1Solution() throws {
        let answer = try day8Part1(inputPath: "../inputs/day8/input.txt")
        XCTAssertEqual(answer, 19_783)
    }
}

/*--------------------------------------------------------------------------------------
  Part 2
--------------------------------------------------------------------------------------*/

class Day8Part2: XCTestCase {
    override func setUp() {
        super.setUp()
        XCTAssert(loggingSystemBootstrapped)
    }

    func testPart2Example1() throws {
        let result = try day8Part2(inputPath: "../inputs/day8/part2_example.txt")
        XCTAssertEqual(result, 6)
    }

    func testLcm() {
        XCTAssertEqual(swift_aoc.lcm([10, 15]), 30)
        XCTAssertEqual(swift_aoc.lcm([24, 10, 15]), 120)
    }

    func testPart2Solution() throws {
        let answer = try day8Part2(inputPath: "../inputs/day8/input.txt")
        XCTAssertEqual(answer, 9_177_460_370_549)
    }
}
