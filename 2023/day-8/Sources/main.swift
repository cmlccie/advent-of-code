import Foundation

/*--------------------------------------------------------------------------------------
  Directions
--------------------------------------------------------------------------------------*/

struct Directions: CustomStringConvertible {
    let directions: String

    init(_ directions: String) {
        self.directions = directions
    }

    var description: String {
        return directions
    }
}

extension Directions: Sequence {
    func makeIterator() -> DirectionsIterator {
        return DirectionsIterator(self)
    }
}

class DirectionsIterator: IteratorProtocol {
    let directions: Directions
    var index: String.Index

    init(_ directions: Directions) {
        self.directions = directions
        self.index = directions.directions.startIndex
    }

    func next() -> Character? {
        let next = directions.directions[index]
        index = directions.directions.index(after: index)

        if index == directions.directions.endIndex {
            index = directions.directions.startIndex
        }

        return next
    }
}

/*--------------------------------------------------------------------------------------
  Labeled Nodes
--------------------------------------------------------------------------------------*/

class Node: CustomStringConvertible {
    var value: String

    var left: Node?
    var right: Node?

    init(value: String) {
        self.value = value
    }

    func navigate(direction: Character) -> Node? {
        switch direction {
        case "L":
            return left
        case "R":
            return right
        default:
            return nil
        }
    }

    var description: String {
        return
            "\(value) = (\(left?.value ?? "nil"), \(right?.value ?? "nil"))"
    }
}

/*--------------------------------------------------------------------------------------
  Network
--------------------------------------------------------------------------------------*/

class Network {
    var nodes: [String: Node] = [:]

    func addNode(value: String, left leftValue: String, right rightValue: String) -> Node {
        let node = nodes[value] ?? Node(value: value)
        nodes[value] = node

        let left = nodes[leftValue] ?? Node(value: leftValue)
        node.left = left
        nodes[leftValue] = left

        let right = nodes[rightValue] ?? Node(value: rightValue)
        node.right = right
        nodes[rightValue] = right

        return node
    }
}

/*--------------------------------------------------------------------------------------
  Helper Functions
--------------------------------------------------------------------------------------*/

func getInput(from path: String) -> String {
    let fileData = FileManager.default.contents(atPath: path)!
    return String(data: fileData, encoding: .utf8)!
}

@available(macOS 13.0, *)
func parseInput(_ input: String) -> (Directions, Network) {
    let lines = input.components(separatedBy: "\n")
    let directions = Directions(lines[0])
    let network = Network()

    let nodeRegex = ##/(?<value>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)/##

    for line in lines[2...] {
        guard line.count > 0 else {
            continue
        }
        if let parsedLine = try? nodeRegex.wholeMatch(in: line) {
            _ = network.addNode(
                value: String(parsedLine.value),
                left: String(parsedLine.left),
                right: String(parsedLine.right)
            )
        }
        else {
            fatalError("Could not parse line: \(line)")
        }
    }

    return (directions, network)
}

/*--------------------------------------------------------------------------------------
  Part 1
--------------------------------------------------------------------------------------*/

@available(macOS 13.0, *)
func part1(inputPath: String) -> Int {
    let input = getInput(from: inputPath)
    let (directions, network) = parseInput(input)

    let startNode = network.nodes["AAA"]
    let endNode = network.nodes["ZZZ"]

    var lastNode = startNode
    var currentNode = startNode
    var steps = 0
    for direction in directions {
        lastNode = currentNode
        steps += 1
        currentNode = currentNode?.navigate(direction: direction)
        if let currentNode = currentNode {
            if currentNode === endNode {
                break
            }
        }
        else {
            fatalError("Could not navigate \(direction) from node \(String(describing: lastNode))")
        }
    }
    print("Part 1: Reached \(endNode!.value) after \(steps) steps.")

    return steps
}

/*--------------------------------------------------------------------------------------
  Part 2
--------------------------------------------------------------------------------------*/

func getPeriod(startingNode: Node, directions: Directions, network: Network) -> Int {
    var currentNode = startingNode
    var steps = 0
    for direction in directions {
        if currentNode.value.hasSuffix("Z") && steps % directions.directions.count == 0 {
            break
        }
        currentNode = currentNode.navigate(direction: direction)!
        steps += 1
    }
    return steps
}

func calculatePrimeFactors(of n: Int) -> [Int: Int] {
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

func lcm(_ numbers: [Int]) -> Int {
    let primeFactorsList = numbers.map { calculatePrimeFactors(of: $0) }
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

@available(macOS 13.0, *)
func part2(inputPath: String) -> Int {
    let input = getInput(from: inputPath)
    let (directions, network) = parseInput(input)

    let startingNodes = network.nodes.values.filter { node in node.value.hasSuffix("A") }
    print("Found \(startingNodes.count) starting nodes ending with A.")

    var periods: [Int] = []
    for startingNode in startingNodes {
        let period = getPeriod(startingNode: startingNode, directions: directions, network: network)
        periods.append(period)
        print("Period for \(startingNode.value) is \(period).")
    }

    let steps = lcm(periods)
    print("Part 2: We will reach the end after \(steps) steps.")

    return steps
}

if #available(macOS 13.0, *) {
    _ = part1(inputPath: "input.txt")
    _ = part2(inputPath: "input.txt")
}
else {
    fatalError("Requires macOS 13.0")
}
