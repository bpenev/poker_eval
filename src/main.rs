#[derive(PartialEq)]
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
		let mut byte_int_suit = 0b0000u8;
		let mut byte_int_rank = 0b0000000000000u16;

		
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
	cards: [Option<Card>; 7]
}

impl Hand {
	fn new() -> Hand {
		Hand {
			cards: [None, None, None, None, None, None, None]
		}
	}

	fn get_cards(&self) -> &[Option<Card>; 7] {
		&self.cards
	}

	fn set_cards(&mut self, cards: [Option<Card>; 7]) {
		self.cards = cards;
	}
	
	fn to_string(&self) -> String {
		let mut result_string = "".to_string();
		let mut space = "".to_string();
		for card in &self.cards {
			match card {
				None => continue,
				_ => {result_string += &(space + &card.as_ref().unwrap().to_string());}
			}
			space = " ".to_string();
		}

		return result_string;
	}

	fn get_card_count(&self) -> u8 {
		let mut count = 0;
		for card in &self.cards {
			match card {
				None => continue,
				_ => {count += 1;}
			}
		}

		return count;
	}

	fn get_first_five_card_count(&self) -> u8 {
		let mut count = 0;
		let mut i = 0;
		while i < 5 {
			match &self.cards[i] {
				None => continue,
				_ => {count += 1;}
			}

			i += 1;
		}

		return count;
	}

	fn check_flush(&self) -> bool {
		let cards = self.get_cards();

		let mut suit_bytes = 0b0000u8;
		let mut i = 0;
		while i < 5 {
			let card_byte_int = cards[i].as_ref().unwrap().to_byte_int();
			suit_bytes |= card_byte_int.0;
			if suit_bytes != card_byte_int.0 {
				return false;
			}

			i += 1;
		}

		return true;
	}

	fn check_straight(&self) -> bool {
		let cards = self.get_cards();

		let mut rank_bytes = 0b0000000000000u16;
		let mut i = 0;
		while i < 5 {
			let card_byte_int = cards[i].as_ref().unwrap().to_byte_int();
			rank_bytes |= card_byte_int.1;
			i += 1;
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

	fn get_high_card(&self) -> &Option<Card> {
		let cards = self.get_cards();
		let mut max_card_rank_bits = 0b000000000000u16;
		let mut max_card = &cards[0];

		for card in cards {
			if let Some(c) = card {
				let card_byte_rank = c.to_byte_int().1;
				if card_byte_rank > max_card_rank_bits {
					max_card_rank_bits = card_byte_rank;
					max_card = card;
				}
			}
		}

		return max_card;
	}

	fn check_repeating_cards(&self) -> bool {
		let cards = self.get_cards();
		let mut suit_bytes = 0b0000u8;
		let mut rank_bytes = 0b0000000000000u16;

		for card in cards {
			if let Some(c) = card {
				let card_byte_int = c.to_byte_int();
				if suit_bytes | card_byte_int.0 == suit_bytes {
					if rank_bytes | card_byte_int.1 == rank_bytes {
						return true;
					} else {
						rank_bytes |= card_byte_int.1;
					}
				} else {
					suit_bytes |= card_byte_int.0;
					rank_bytes |= card_byte_int.1;
				}
			}
		}

		return false;
	}

	fn get_best_hand(&self) -> &Hand {
		return self;
	}
}

fn main() {
	let mut h = Hand::new();

	let current_hand_cards = [
		Some(
			Card {
				suit: Suit::DIAMONDS,
				rank: Rank::SIX
			}
		),
		Some(
			Card {
				suit: Suit::DIAMONDS,
				rank: Rank::TWO
			}
		),
		Some(
			Card {
				suit: Suit::DIAMONDS,
				rank: Rank::THREE
			}
		),
		Some(
			Card {
				suit: Suit::DIAMONDS,
				rank: Rank::FOUR
			}
		),
		Some(
			Card {
				suit: Suit::DIAMONDS,
				rank: Rank::FIVE
			}
		),
		None,
		None
	];

	h.set_cards(current_hand_cards);
	
	let h = h.get_best_hand();
	
	if h.check_repeating_cards() {
		println!("Repeating cards!");
	} else if h.get_first_five_card_count() != 5 || h.get_card_count() != 5 {
		println!("First five cards in hand must be set and the rest not set to check for existing combinations");
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
			}
		}

		if let Some(hc) = h.get_high_card() {
			println!("High Card: {}", hc.to_string());
		}
	}
	
	println!("Hand: {}", h.to_string());
	println!("Card Count: {}", h.get_card_count());
}
