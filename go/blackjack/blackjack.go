package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	switch card {
	case "ace":
		return 11
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	case "ten", "jack", "queen", "king":
		return 10
	}
	return 0
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	player := ParseCard(card1) + ParseCard(card2)
	dealer := ParseCard(dealerCard)
	switch player {
	case 22:
		return "P"
	case 21:
		switch dealerCard {
		case "ace", "ten", "jack", "queen", "king":
			return "S"
		default:
			return "W"
		}
	case 17, 18, 19, 20:
		return "S"
	case 12, 13, 14, 15, 16:
		switch {
		case dealer >= 7:
			return "H"
		default:
			return "S"
		}
	default:
		return "H"
	}
}
