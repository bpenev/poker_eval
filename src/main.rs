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
			Suit::CLUBS => "c",
			Suit::DIAMONDS => "d",
			Suit::HEARTS => "h",
			Suit::SPADES => "s"
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
			Rank::TWO => "2",
			Rank::THREE => "3",
			Rank::FOUR => "4",
			Rank::FIVE => "5",
			Rank::SIX => "6",
			Rank::SEVEN => "7",
			Rank::EIGHT => "8",
			Rank::NINE => "9",
			Rank::TEN => "T",
			Rank::JACK => "J",
			Rank::QUEEN => "Q",
			Rank::KING => "K",
			Rank::ACE => "A"
		}
	}
}

struct Card {
	suit: Suit,
	rank: Rank
}

impl Card {
	fn to_int32(&self) -> u32 {
		let mut bint = 0b0000_0000000000000u32;
		
		match self.suit {
			Suit::CLUBS => 		{bint |= 0b1000_0000000000000u32;},
			Suit::DIAMONDS => 	{bint |= 0b0100_0000000000000u32;},
			Suit::HEARTS => 	{bint |= 0b0010_0000000000000u32;},
			Suit::SPADES => 	{bint |= 0b0001_0000000000000u32;}
		}
		
		match self.rank {
			Rank::TWO => 	{bint |= 0b0000_1000000000000u32;},
			Rank::THREE => 	{bint |= 0b0000_0100000000000u32;},
			Rank::FOUR => 	{bint |= 0b0000_0010000000000u32;},
			Rank::FIVE => 	{bint |= 0b0000_0001000000000u32;},
			Rank::SIX => 	{bint |= 0b0000_0000100000000u32;},
			Rank::SEVEN => 	{bint |= 0b0000_0000010000000u32;},
			Rank::EIGHT => 	{bint |= 0b0000_0000001000000u32;},
			Rank::NINE => 	{bint |= 0b0000_0000000100000u32;},
			Rank::TEN => 	{bint |= 0b0000_0000000010000u32;},
			Rank::JACK => 	{bint |= 0b0000_0000000001000u32;},
			Rank::QUEEN => 	{bint |= 0b0000_0000000000100u32;},
			Rank::KING => 	{bint |= 0b0000_0000000000010u32;},
			Rank::ACE => 	{bint |= 0b0000_0000000000001u32;}
		}
		
		return bint;
	}
	
	fn to_string(&self) -> String {
		return format!("{}{}", self.rank.to_string(), self.suit.to_string());
	}
}

struct Hand {
	c1: Card,
	c2: Card,
	c3: Option<Card>,
	c4: Option<Card>,
	c5: Option<Card>,
	c6: Option<Card>,
	c7: Option<Card>
}

impl Hand {
	fn new(c1: Card, c2: Card) -> Hand {
		Hand {
			c1,
			c2,
			c3: None,
			c4: None,
			c5: None,
			c6: None,
			c7: None
		}
	}
	
	fn add_c3(mut self, c3: Card) -> Hand {
		self.c3 = Some(c3);
		self
	}
	
	fn add_c4(mut self, c4: Card) -> Hand {
		self.c4 = Some(c4);
		self
	}
	
	fn add_c5(mut self, c5: Card) -> Hand {
		self.c5 = Some(c5);
		self
	}
	
	fn add_c6(mut self, c6: Card) -> Hand {
		self.c6 = Some(c6);
		self
	}
	
	fn add_c7(mut self, c7: Card) -> Hand {
		self.c7 = Some(c7);
		self
	}
	
	fn get_card_count(&self) -> u8 {
		2 +
		match self.c3 {
			None => 0,
			_ => 1
		} +
		match self.c4 {
			None => 0,
			_ => 1
		} +
		match self.c5 {
			None => 0,
			_ => 1
		} +
		match self.c6 {
			None => 0,
			_ => 1
		} +
		match self.c7 {
			None => 0,
			_ => 1
		}
	}
	
	fn get_first_five_card_count(&self) -> u8 {
		2 +
		match self.c3 {
			None => 0,
			_ => 1
		} +
		match self.c4 {
			None => 0,
			_ => 1
		} +
		match self.c5 {
			None => 0,
			_ => 1
		}
	}
	
	fn to_string(&self) -> String {
		let mut c3 = "".to_string();
		if let Some(c) = &self.c3 {	
			c3 = format!(" {}", c.to_string());
		};
		
		let mut c4 = "".to_string();
		if let Some(c) = &self.c4 {	
			c4 = format!(" {}", c.to_string());
		};
		
		let mut c5 = "".to_string();
		if let Some(c) = &self.c5 {	
			c5 = format!(" {}", c.to_string());
		};
		
		let mut c6 = "".to_string();
		if let Some(c) = &self.c6 {	
			c6 = format!(" {}", c.to_string());
		};
		
		let mut c7 = "".to_string();
		if let Some(c) = &self.c7 {	
			c7 = format!(" {}", c.to_string());
		};
	
		return 
			format!("{}", self.c1.to_string()) + 
			&format!(" {}", self.c2.to_string()) +
			&c3 + &c4 + &c5 + &c6 + &c7;
			
	}
}

fn check_royal_flush(hand: &Hand) -> bool {
	if hand.get_first_five_card_count() != 5 || 
		hand.get_card_count() != 5 {
		panic!("First five cards in hand must be set and the rest not set to check for existing combinations");
	} else {
		if (&hand.c1.suit == &hand.c2.suit) && 
			(&hand.c1.suit != &hand.c3.as_ref().unwrap().suit) &&
			(&hand.c1.suit != &hand.c4.as_ref().unwrap().suit) &&
			(&hand.c1.suit != &hand.c5.as_ref().unwrap().suit) {
			return true;
		} else {
			return false;
		}
	}
}

fn get_best_hand(hand: Hand) -> Hand {
	return hand;
}

fn main() {
	let h = Hand::new(
		Card {
			suit: Suit::DIAMONDS,
			rank: Rank::ACE
		},
		Card {
			suit: Suit::CLUBS,
			rank: Rank::ACE
		}
	)
	.add_c3(
		Card {
			suit: Suit::SPADES,
			rank: Rank::ACE
		}
	)
	.add_c4(
		Card {
			suit: Suit::HEARTS,
			rank: Rank::ACE
		}
	)
	.add_c5(
		Card {
			suit: Suit::DIAMONDS,
			rank: Rank::KING
		}
	);
	
	let h = get_best_hand(h);
	
	if check_royal_flush(&h) {
		println!("Royal Flush");
	}
	
	println!("{}", h.to_string());
	print!("{}", h.get_card_count());
}
