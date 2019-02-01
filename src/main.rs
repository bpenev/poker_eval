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
			Suit::CLUBS => 		{byte_int_suit |= 0b1000u8;},
			Suit::DIAMONDS => 	{byte_int_suit |= 0b0100u8;},
			Suit::HEARTS => 	{byte_int_suit |= 0b0010u8;},
			Suit::SPADES => 	{byte_int_suit |= 0b0001u8;}
		}
		
		match self.rank {
			Rank::TWO => 	{byte_int_rank |= 0b1000000000000u16;},
			Rank::THREE => 	{byte_int_rank |= 0b0100000000000u16;},
			Rank::FOUR => 	{byte_int_rank |= 0b0010000000000u16;},
			Rank::FIVE => 	{byte_int_rank |= 0b0001000000000u16;},
			Rank::SIX => 	{byte_int_rank |= 0b0000100000000u16;},
			Rank::SEVEN => 	{byte_int_rank |= 0b0000010000000u16;},
			Rank::EIGHT => 	{byte_int_rank |= 0b0000001000000u16;},
			Rank::NINE => 	{byte_int_rank |= 0b0000000100000u16;},
			Rank::TEN => 	{byte_int_rank |= 0b0000000010000u16;},
			Rank::JACK => 	{byte_int_rank |= 0b0000000001000u16;},
			Rank::QUEEN => 	{byte_int_rank |= 0b0000000000100u16;},
			Rank::KING => 	{byte_int_rank |= 0b0000000000010u16;},
			Rank::ACE => 	{byte_int_rank |= 0b0000000000001u16;}
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
}

fn check_royal_flush(hand: &Hand) -> bool {
	let cards = hand.get_cards();

	if hand.get_first_five_card_count() != 5 || 
		hand.get_card_count() != 5 {
		panic!("First five cards in hand must be set and the rest not set to check for existing combinations");
	} else {
		return false;
	}
}

fn get_best_hand(hand: Hand) -> Hand {
	return hand;
}

fn main() {
	let mut h = Hand::new();

	let current_hand_cards = [
		Some(
			Card {
				suit: Suit::DIAMONDS,
				rank: Rank::ACE
			}
		),
		Some(
			Card {
				suit: Suit::CLUBS,
				rank: Rank::ACE
			}
		),
		Some(
			Card {
				suit: Suit::SPADES,
				rank: Rank::ACE
			}
		),
		Some(
			Card {
				suit: Suit::HEARTS,
				rank: Rank::ACE
			}
		),
		Some(
			Card {
				suit: Suit::DIAMONDS,
				rank: Rank::KING
			}
		),
		None,
		None
	];

	h.set_cards(current_hand_cards);
	
	let h = get_best_hand(h);
	
	if check_royal_flush(&h) {
		println!("Royal Flush");
	}
	
	println!("{}", h.to_string());
	print!("{}", h.get_card_count());
}
