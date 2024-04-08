import ArgumentParser
import Foundation
import Logging

/*--------------------------------------------------------------------------------------
  Cards
--------------------------------------------------------------------------------------*/

private enum Card: Int, Equatable {
    case two = 2
    case three, four, five, six, seven, eight, nine, ten, jack, queen, king, ace

    static func from(_ char: Character) -> Card {
        switch char {
        case "2":
            return .two
        case "3":
            return .three
        case "4":
            return .four
        case "5":
            return .five
        case "6":
            return .six
        case "7":
            return .seven
        case "8":
            return .eight
        case "9":
            return .nine
        case "T":
            return .ten
        case "J":
            return .jack
        case "Q":
            return .queen
        case "K":
            return .king
        case "A":
            return .ace
        default:
            fatalError("Invalid card: \(char)")
        }
    }
}

extension Card: Comparable {
    static func < (lhs: Card, rhs: Card) -> Bool {
        return lhs.rawValue < rhs.rawValue
    }
}

/*--------------------------------------------------------------------------------------
  Hands
--------------------------------------------------------------------------------------*/

private enum HandType: Int {
    case highCard, onePair, twoPairs, threeOfAKind, fullHouse, fourOfAKind, fiveOfAKind
}

private struct Hand {
    let hand: String
    let bid: Int

    let handType: HandType
    let cards: [Card]

    init(hand: String, bid: Int) {
        self.hand = hand
        self.bid = bid

        // Determine hand type
        var cards: [Card] = []
        var cardCounter: [Card: Int] = [:]

        for cardCharacter in hand {
            let card = Card.from(cardCharacter)
            cards.append(card)
            cardCounter[card] = (cardCounter[card] ?? 0) + 1
        }

        self.cards = cards

        let cardCounts: [Int] = cardCounter.values.sorted(by: >)
        switch cardCounts {
        case [5]:
            self.handType = .fiveOfAKind
        case [4, 1]:
            self.handType = .fourOfAKind
        case [3, 2]:
            self.handType = .fullHouse
        case [3, 1, 1]:
            self.handType = .threeOfAKind
        case [2, 2, 1]:
            self.handType = .twoPairs
        case [2, 1, 1, 1]:
            self.handType = .onePair
        case [1, 1, 1, 1, 1]:
            self.handType = .highCard
        default:
            fatalError("Invalid hand: \(hand)")
        }
    }
}

extension Hand: CustomStringConvertible {
    var description: String {
        return "\(hand) (\(handType)) \(bid)"
    }
}

extension Hand: Comparable {
    static func < (lhs: Hand, rhs: Hand) -> Bool {
        if lhs.handType != rhs.handType {
            return lhs.handType.rawValue < rhs.handType.rawValue
        }
        return lhs.cards.lexicographicallyPrecedes(rhs.cards)
    }
}

/*--------------------------------------------------------------------------------------
  Helper Functions
--------------------------------------------------------------------------------------*/

private func parseInput(_ input: String) -> [Hand] {
    return input.split(separator: "\n").map { line in
        let parts = line.split(separator: " ")
        let hand = String(parts[0])
        let bid = Int(parts[1])!
        return Hand(hand: hand, bid: bid)
    }
}

/*--------------------------------------------------------------------------------------
  Main
--------------------------------------------------------------------------------------*/

func day7Part1(inputPath: String) throws -> Int {
    let logger = Logger(label: "day7.part1")
    let input = try getInput(from: inputPath)

    var handList = parseInput(input)
    handList.sort(by: <)
    logger.debug("Hands: \(handList as AnyObject)")

    let winnings = handList.enumerated().map { (index, hand) in
        return hand.bid * (index + 1)
    }.reduce(0, +)

    print("Part 1 Winnings: \(winnings)")

    return winnings
}
