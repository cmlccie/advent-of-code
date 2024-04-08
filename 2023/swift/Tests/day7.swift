import Logging
import XCTest

@testable import swift_aoc

final class Day7Part1: XCTestCase {
    override func setUp() {
        super.setUp()
        XCTAssert(loggingSystemBootstrapped)
    }

    func testSolution() throws {
        let answer = try day7Part1(inputPath: "../inputs/day7/input.txt")
        XCTAssertEqual(answer, 247_815_719)
    }
}

final class Day7Part2: XCTestCase {
    override func setUp() {
        super.setUp()
        XCTAssert(loggingSystemBootstrapped)
    }

    func testSolution() throws {
        let answer = try day7Part2(inputPath: "../inputs/day7/input.txt")
        XCTAssertEqual(answer, 248_747_492)
    }
}
