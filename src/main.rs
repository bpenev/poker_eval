use std::collections::HashSet;

#[allow(dead_code)]
enum Suit {
	CLUBS,
	DIAMONDS,
	HEARTS,
	SPADES
}

impl Suit {
	fn to_string(&self) -> &str {
		match self {
			Suit::CLUBS => 		"c",
			Suit::DIAMONDS => 	"d",
			Suit::HEARTS => 	"h",
			Suit::SPADES => 	"s"
		}
	}
}

#[allow(dead_code)]
enum Rank {
	TWO,
	THREE,
	FOUR,
	FIVE,
	SIX,
	SEVEN,
	EIGHT,
	NINE,
	TEN,
	JACK,
	QUEEN,
	KING,
	ACE
}

impl Rank {
	fn to_string(&self) -> &str {
		match self {
			Rank::TWO => 	"2",
			Rank::THREE => 	"3",
			Rank::FOUR => 	"4",
			Rank::FIVE => 	"5",
			Rank::SIX => 	"6",
			Rank::SEVEN => 	"7",
			Rank::EIGHT => 	"8",
			Rank::NINE => 	"9",
			Rank::TEN => 	"T",
			Rank::JACK => 	"J",
			Rank::QUEEN => 	"Q",
			Rank::KING => 	"K",
			Rank::ACE => 	"A"
		}
	}
}

struct Card {
	suit: Suit,
	rank: Rank
}

impl Card {
	fn to_byte_int(&self) -> (u8, u16) {
		let mut byte_int_suit = 0u8;
		let mut byte_int_rank = 0u16;

		
		match self.suit {
			Suit::CLUBS => 		{byte_int_suit |= 0b0001u8;},
			Suit::DIAMONDS => 	{byte_int_suit |= 0b0010u8;},
			Suit::HEARTS => 	{byte_int_suit |= 0b0100u8;},
			Suit::SPADES => 	{byte_int_suit |= 0b1000u8;}
		}
		
		match self.rank {
			Rank::TWO => 	{byte_int_rank |= 0b0000000000001u16;},
			Rank::THREE => 	{byte_int_rank |= 0b0000000000010u16;},
			Rank::FOUR => 	{byte_int_rank |= 0b0000000000100u16;},
			Rank::FIVE => 	{byte_int_rank |= 0b0000000001000u16;},
			Rank::SIX => 	{byte_int_rank |= 0b0000000010000u16;},
			Rank::SEVEN => 	{byte_int_rank |= 0b0000000100000u16;},
			Rank::EIGHT => 	{byte_int_rank |= 0b0000001000000u16;},
			Rank::NINE => 	{byte_int_rank |= 0b0000010000000u16;},
			Rank::TEN => 	{byte_int_rank |= 0b0000100000000u16;},
			Rank::JACK => 	{byte_int_rank |= 0b0001000000000u16;},
			Rank::QUEEN => 	{byte_int_rank |= 0b0010000000000u16;},
			Rank::KING => 	{byte_int_rank |= 0b0100000000000u16;},
			Rank::ACE => 	{byte_int_rank |= 0b1000000000000u16;}
		}
		
		return (byte_int_suit, byte_int_rank);
	}
	
	fn to_string(&self) -> String {
		return format!("{}{}", self.rank.to_string(), self.suit.to_string());
	}
}

struct Hand {
	cards: [Card; 5]
}

impl Hand {
	fn to_string(&self) -> String {
		let mut result_string = "".to_string();
		let mut space = "".to_string();
		for card in &self.cards {
			result_string += &(space + &card.to_string());
			space = " ".to_string();
		}

		return result_string;
	}

	fn check_repeating_cards(&self) -> bool {
		let mut uniq = HashSet::new();

		for card in &self.cards {
			let card_byte_int = card.to_byte_int();
			let card_byte_int_combined = ((card_byte_int.0 as u32) << 13) + card_byte_int.1 as u32;

			uniq.insert(card_byte_int_combined);
		}

		if uniq.len() == 5 {
			return false;
		}

		return true;
	}

	// four, three, #two of a kind
	fn check_same_kind(&self) -> (bool, bool, u8) {
		let mut rank_bytes = (0u16, 0u16);
		let mut same = (1,1);

		for card in &self.cards {
			let card_byte_int = card.to_byte_int();
			if rank_bytes.0 | card_byte_int.1 == rank_bytes.0 {
				rank_bytes.0 = card_byte_int.1;
				same.0 += 1;
			} else if same.0 == 1 {
				rank_bytes.0 |= card_byte_int.1;
			}

			if rank_bytes.1 | card_byte_int.1 == rank_bytes.1 && same.0 != 1 {
				rank_bytes.1 = card_byte_int.1;
				same.1 += 1;
			} else if same.1 == 1 && same.0 != 1  {
				rank_bytes.1 |= card_byte_int.1;
			}
		}

		if same.0 == 4 || same.1 == 4 {
			return (true, false, 0);
		} else if (same.0 == 3 || same.1 == 3) && (same.0 == 2 || same.1 == 2) {
			return (false, true, 1);
		} else if same.0 == 3 || same.1 == 3 {
			return (false, true, 0);
		} else if same.0 == 2 && same.1 == 2 {
			return (false, false, 2);
		} else if same.0 == 2 || same.1 == 2 {
			return (false, false, 1);
		}

		return (false, false, 0);
	}

	fn check_flush(&self) -> bool {
		let mut suit_bytes = 0u8;
		for card in &self.cards {
			let card_byte_int = card.to_byte_int();
			suit_bytes |= card_byte_int.0;
			if suit_bytes != card_byte_int.0 {
				return false;
			}
		}

		return true;
	}

	fn check_straight(&self) -> bool {
		let mut rank_bytes = 0u16;
		for card in &self.cards {
			let card_byte_int = card.to_byte_int();
			rank_bytes |= card_byte_int.1;
		}

		let mut straight_pattern = 0b0000000011111u16;
		let mut j = 0;
		while j < 9 {
			if rank_bytes == straight_pattern {
				return true;
			} else {
				straight_pattern = straight_pattern << 1;
			}

			j += 1;
		}

		// ACE as ONE
		if rank_bytes == 0b1000000001111u16 {
			return true;
		}

		return false;
	}
}

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
