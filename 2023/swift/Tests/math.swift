import XCTest

@testable import swift_aoc

/*-------------------------------------------------------------------------------------------------
  Test Custom Math Functions
-------------------------------------------------------------------------------------------------*/

class CustomMathFunctions: XCTestCase {
    func testPrimeFactors() {
        XCTAssertEqual(swift_aoc.primeFactors(of: 10), [2: 1, 5: 1])
        XCTAssertEqual(swift_aoc.primeFactors(of: 15), [3: 1, 5: 1])
        XCTAssertEqual(swift_aoc.primeFactors(of: 24), [2: 3, 3: 1])
        XCTAssertEqual(swift_aoc.primeFactors(of: 100), [2: 2, 5: 2])
        XCTAssertEqual(swift_aoc.primeFactors(of: 2_983_747), [13: 1, 229_519: 1])
    }

    func testLcm() {
        XCTAssertEqual(swift_aoc.lcm([10, 15]), 30)
        XCTAssertEqual(swift_aoc.lcm([24, 10, 15]), 120)
    }
}
