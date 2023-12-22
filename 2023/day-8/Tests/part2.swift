import XCTest

@testable import day8

class Part2Tests: XCTestCase {

    func testPart2Example1() {
        let result = day8.part2(inputPath: "Tests/part2_example.txt")
        XCTAssertEqual(result, 6)
    }

    func testLcm() {
        XCTAssertEqual(day8.lcm([10, 15]), 30)
        XCTAssertEqual(day8.lcm([24, 10, 15]), 120)
    }

    func testPart2Solution() {
        let answer = day8.part2(inputPath: "input.txt")
        XCTAssertEqual(answer, 9_177_460_370_549)
    }
}
