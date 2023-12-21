import Foundation

/*--------------------------------------------------------------------------------------
  Cards
--------------------------------------------------------------------------------------*/

enum Card: Int, Equatable {
    case Two = 2
    case Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace

    static func from(_ char: Character) -> Card {
        switch char {
        case "2":
            return .Two
        case "3":
            return .Three
        case "4":
            return .Four
        case "5":
            return .Five
        case "6":
            return .Six
        case "7":
            return .Seven
        case "8":
            return .Eight
        case "9":
            return .Nine
        case "T":
            return .Ten
        case "J":
            return .Jack
        case "Q":
            return .Queen
        case "K":
            return .King
        case "A":
            return .Ace
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

enum HandType: Int {
    case HighCard, OnePair, TwoPairs, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind
}

struct Hand {
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
            self.handType = .FiveOfAKind
        case [4, 1]:
            self.handType = .FourOfAKind
        case [3, 2]:
            self.handType = .FullHouse
        case [3, 1, 1]:
            self.handType = .ThreeOfAKind
        case [2, 2, 1]:
            self.handType = .TwoPairs
        case [2, 1, 1, 1]:
            self.handType = .OnePair
        case [1, 1, 1, 1, 1]:
            self.handType = .HighCard
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

func getInput(from path: String) -> String {
    let fileData = FileManager.default.contents(atPath: path)!
    return String(data: fileData, encoding: .utf8)!
}

func parseInput(_ input: String) -> [Hand] {
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

func main(inputPath: String) -> Int {
    let input = getInput(from: inputPath)

    var handList = parseInput(input)
    handList.sort(by: <)

    let winnings = handList.enumerated().map { (index, hand) in
        return hand.bid * (index + 1)
    }.reduce(0, +)

    // Print the results
    for hand in handList {
        print(hand)
    }

    print("Winnings: \(winnings)")

    return winnings
}

_ = main(inputPath: "input.txt")
