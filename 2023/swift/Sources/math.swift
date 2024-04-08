import Foundation

/*-------------------------------------------------------------------------------------------------
  Custom Math Functions
-------------------------------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
  Prime Factors
-----------------------------------------------------------------------------*/

internal func primeFactors(of n: Int) -> [Int: Int] {
    var n = n
    var factors: [Int] = []
    var divisor = 2
    while n > 1 {
        while n % divisor == 0 {
            factors.append(divisor)
            n /= divisor
        }
        divisor += 1
        if divisor * divisor > n {
            if n > 1 { factors.append(n) }
            break
        }
    }

    // Count the number of times each factor appears
    var factorCounts: [Int: Int] = [:]
    for factor in factors {
        factorCounts[factor] = (factorCounts[factor] ?? 0) + 1
    }

    return factorCounts
}

/*-----------------------------------------------------------------------------
  Least Common Multiple (LCM)
-----------------------------------------------------------------------------*/

internal func lcm(_ numbers: [Int]) -> Int {
    let primeFactorsList = numbers.map { primeFactors(of: $0) }
    var largestPrimeFactors: [Int: Int] = [:]
    for primeFactors in primeFactorsList {
        for (prime, power) in primeFactors {
            largestPrimeFactors[prime] = max(largestPrimeFactors[prime] ?? 0, power)
        }
    }

    let leastCommonMultiple = largestPrimeFactors.reduce(1) { result, factor in
        result * pow(Decimal(factor.key), factor.value)
    }
    return (leastCommonMultiple as NSDecimalNumber).intValue
}
