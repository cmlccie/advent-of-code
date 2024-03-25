import XCTest

@testable import swift_aoc

class Day7Part1: XCTestCase {
    func testSolution() {
        let answer = swift_aoc.day7Part1(inputPath: "../inputs/day7/input.txt")
        XCTAssertEqual(answer, 247_815_719)
    }
}

class Day7Part2: XCTestCase {
    func testSolution() {
        let answer = swift_aoc.day7Part2(inputPath: "../inputs/day7/input.txt")
        XCTAssertEqual(answer, 248_747_492)
    }
}
