mod hand;

use hand::Card;
use hand::Hand;
use hand::Rank;
use hand::Suit;

fn main() {
	let h = Hand {
		cards: [
			Card {
				suit: Suit::CLUBS,
				rank: Rank::SIX
			},
			Card {
				suit: Suit::SPADES,
				rank: Rank::SIX
			},
			Card {
				suit: Suit::HEARTS,
				rank: Rank::EIGHT
			},
			Card {
				suit: Suit::DIAMONDS,
				rank: Rank::SEVEN
			},
			Card {
				suit: Suit::HEARTS,
				rank: Rank::TEN
			}
		]
	};
	
	if h.check_repeating_cards() {
		println!("Repeating cards!");
	} else {
		if h.check_flush() {
			if h.check_straight() {
				println!("Straight Flush");
			} else {
				println!("Flush");
			}
		} else {
			if h.check_straight() {
				println!("Straight");
			} else {
				let same_kind = h.check_same_kind();
				if same_kind.0 {
					println!("Four of a kind");
				} else if same_kind.1 && (same_kind.2 == 1) {
					println!("Full House");
				} else if same_kind.1 {
					println!("Three of a kind");
				} else if same_kind.2 == 2 {
					println!("Two Pairs");
				} else if same_kind.2 == 1 {
					println!("Pair");
				}
			}
		}
	}
	
	println!("Hand: {}", h.to_string());
}