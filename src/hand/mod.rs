use std::cmp::Ordering;
use std::slice::Iter;

use fnv::FnvHashMap;
use fnv::FnvHashSet;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum Suit {
	CLUBS,
	DIAMONDS,
	HEARTS,
	SPADES
}

impl Suit {
	#[allow(dead_code)]
	pub fn to_string(&self) -> &str {
		match self {
			Suit::CLUBS => 		"c",
			Suit::DIAMONDS => 	"d",
			Suit::HEARTS => 	"h",
			Suit::SPADES => 	"s"
		}
	}

	pub fn iterator() -> Iter<'static, Suit> {
        static SUIT: [Suit;  4] = [Suit::CLUBS,Suit::DIAMONDS,Suit::HEARTS,Suit::SPADES];
        SUIT.into_iter()
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Rank {
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

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
		if self.to_int() > other.to_int() {
			return Ordering::Greater;
		} else if self.to_int() < other.to_int() {
			return Ordering::Less;
		} else {
			return Ordering::Equal;
		}
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Rank {
	#[allow(dead_code)]
	pub fn to_string(&self) -> &str {
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

	pub fn to_int(&self) -> u8 {
		match self {
			Rank::TWO => 	2,
			Rank::THREE => 	3,
			Rank::FOUR => 	4,
			Rank::FIVE => 	5,
			Rank::SIX => 	6,
			Rank::SEVEN => 	7,
			Rank::EIGHT => 	8,
			Rank::NINE => 	9,
			Rank::TEN => 	10,
			Rank::JACK => 	11,
			Rank::QUEEN => 	12,
			Rank::KING => 	13,
			Rank::ACE => 	14
		}
	}

	pub fn iterator() -> Iter<'static, Rank> {
        static RANK: [Rank; 13] = [Rank::TWO,Rank::THREE,Rank::FOUR,Rank::FIVE,
        	Rank::SIX,Rank::SEVEN,Rank::EIGHT,Rank::NINE,Rank::TEN,Rank::JACK,
        	Rank::QUEEN,Rank::KING,Rank::ACE];
        RANK.into_iter()
	}
}

#[derive(Copy, Clone)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank > other.rank {
        	return Ordering::Greater;
        } else if self.rank < other.rank {
        	return Ordering::Less;
        } else {
        	if self.suit > other.suit {
        		return Ordering::Greater;
        	} else if self.suit < other.suit {
        		return Ordering::Less;
        	} else {
        		return Ordering::Equal;
        	}
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.suit == other.suit && self.rank == other.rank;
    }
}

impl Eq for Card { }

impl Card {
	pub fn new_from_string(s: String) -> Card {
		let mut card_chars = s.chars();
		let rank_char = card_chars.next().unwrap();
		let suit_char = card_chars.next().unwrap();

		return Card {
			suit: match suit_char {
				'c' => Suit::CLUBS,
				'd' => Suit::DIAMONDS,
				'h' => Suit::HEARTS,
				's' => Suit::SPADES,
				_ => panic!("Invalid suit character provided!")
			},
			rank: match rank_char {
				'2' => Rank::TWO,
				'3' => Rank::THREE,
				'4' => Rank::FOUR,
				'5' => Rank::FIVE,
				'6' => Rank::SIX,
				'7' => Rank::SEVEN,
				'8' => Rank::EIGHT,
				'9' => Rank::NINE,
				'T' => Rank::TEN,
				'J' => Rank::JACK,
				'Q' => Rank::QUEEN,
				'K' => Rank::KING,
				'A' => Rank::ACE,
				_ => panic!("Invalid rank character provided!")
			}
		};
	}

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
	
	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		return format!("{}{}", self.rank.to_string(), self.suit.to_string());
	}

	fn to_special_int(&self) -> u32 {
		match self.suit {
			Suit::CLUBS => {
					match self.rank {
						Rank::TWO => 	1,
						Rank::THREE => 	2,
						Rank::FOUR => 	4,
						Rank::FIVE => 	8,
						Rank::SIX => 	16,
						Rank::SEVEN => 	32,
						Rank::EIGHT => 	64,
						Rank::NINE => 	128,
						Rank::TEN => 	256,
						Rank::JACK => 	512,
						Rank::QUEEN => 	1024,
						Rank::KING => 	2048,
						Rank::ACE => 	4096
					}
				},
			Suit::DIAMONDS => {
					match self.rank {
						Rank::TWO => 	8192,
						Rank::THREE => 	16384,
						Rank::FOUR => 	32768,
						Rank::FIVE => 	65536,
						Rank::SIX => 	131072,
						Rank::SEVEN => 	262144,
						Rank::EIGHT => 	524288,
						Rank::NINE => 	1048576,
						Rank::TEN => 	2097152,
						Rank::JACK => 	2781920,
						Rank::QUEEN => 	4194304,
						Rank::KING => 	8109704,
						Rank::ACE => 	8388608
					}
				},
			Suit::HEARTS => {
					match self.rank {
						Rank::TWO => 	14486720,
						Rank::THREE => 	16777216,
						Rank::FOUR => 	18423568,
						Rank::FIVE => 	28682752,
						Rank::SIX => 	33554432,
						Rank::SEVEN => 	46487664,
						Rank::EIGHT => 	53141048,
						Rank::NINE => 	60478720,
						Rank::TEN => 	63825204,
						Rank::JACK => 	67108864,
						Rank::QUEEN => 	75991814,
						Rank::KING => 	77304384,
						Rank::ACE => 	82667776
					}
				},
			Suit::SPADES => {
					match self.rank {
						Rank::TWO => 	88115619,
						Rank::THREE => 	90823064,
						Rank::FOUR => 	90920448,
						Rank::FIVE => 	99726952,
						Rank::SIX => 	101451626,
						Rank::SEVEN => 	104451072,
						Rank::EIGHT => 	110836741,
						Rank::NINE => 	113779648,
						Rank::TEN => 	118997632,
						Rank::JACK => 	128070680,
						Rank::QUEEN => 	128184096,
						Rank::KING => 	130150400,
						Rank::ACE => 	134086656
					}
				}
		}
	}
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum HandRank {
	STRAIGHT_FLUSH,
	FOUR_OF_A_KIND,
	FULL_HOUSE,
	FLUSH,
	STRAIGHT,
	THREE_OF_A_KIND,
	TWO_PAIRS,
	PAIR,
	HIGH_CARD
}

impl Ord for HandRank {
	fn cmp(&self, other: &Self) -> Ordering {
		if self == other {
			return Ordering::Equal;
		} else if self.to_int() > other.to_int() {
			return Ordering::Greater;
		} else {
			return Ordering::Less;
		}
	}
}

impl PartialOrd for HandRank {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl HandRank {
	pub fn to_int(&self) -> u8 {
		match self {
			HandRank::STRAIGHT_FLUSH 	=> 1,
			HandRank::FOUR_OF_A_KIND 	=> 2,
			HandRank::FULL_HOUSE		=> 3,
			HandRank::FLUSH 			=> 4,
			HandRank::STRAIGHT 			=> 5,
			HandRank::THREE_OF_A_KIND	=> 6,
			HandRank::TWO_PAIRS 		=> 7,
			HandRank::PAIR 				=> 8,
			HandRank::HIGH_CARD 		=> 9
		}
	}
}

#[derive(Copy, Clone)]
pub struct Hand {
	pub cards: [Card; 5]
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.cards == other.cards {
			return Ordering::Equal;
		} else {
			return self.compare(other);
		}
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Hand {
	fn eq(&self, other: &Self) -> bool {
		let exact_match = self.cards.iter().zip(other.cards.iter()).all(|(a,b)| a == b);
		if exact_match || self.compare(other) == Ordering::Equal {
			return true;
		} else {
			return false;
		}
	}
}

impl Eq for Hand { }

impl Hand {
	#[allow(dead_code)]
	pub fn new_from_string(s: String) -> Hand {
		let mut cardsv: Vec<Card> = Vec::new();
		for card_string in s.split(" ") {
			cardsv.push(Card::new_from_string(card_string.to_string()));
		}

		return Hand {
			cards: [
				cardsv[0],
				cardsv[1],
				cardsv[2],
				cardsv[3],
				cardsv[4]
			]
		}
	}

	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		let mut result_string = "".to_string();
		let mut space = "".to_string();

		for card in &self.cards {
			result_string += &(space + &card.to_string());
			space = " ".to_string();
		}

		return result_string;
	}
	
	pub fn to_ordered_string(&self) -> String {
		let mut ordered_cards = self.cards.clone();
		ordered_cards.sort();
		let ordered_hand = Hand {cards: ordered_cards};
		return ordered_hand.to_string();
	}

	pub fn to_int(&self) -> usize {
		let mut res = 0usize;
		for i in 0..5 {
			res ^= self.cards[i].to_special_int() as usize;
		}

		return res;
	}

	#[allow(dead_code)]
	pub fn check_repeating_cards(&self) -> bool {
		let mut uniq = FnvHashSet::default();

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

	pub fn check_same_kind(&self) -> FnvHashMap<u16, u8> {
		let mut rank_freq = FnvHashMap::with_capacity_and_hasher(5, Default::default());
		for card in &self.cards {
			let freq = rank_freq.entry(card.to_byte_int().1).or_insert(0);
    		*freq += 1;
		}

		return rank_freq;
	}

	#[allow(dead_code)]
	pub fn check_same_kind_tuple(&self) -> (bool, bool, u8) {
		return self.get_freq_tuple(&self.check_same_kind());
	}

	pub fn check_flush(&self) -> bool {
		let suit = &self.cards[0].suit;
		let mut i = 1;
		while i < 5 {
			if &self.cards[i].suit != suit {
				return false;
			}

			i += 1;
		}

		return true;
	}

	pub fn check_straight(&self) -> (bool, Rank) {
		let mut rank_bytes = 0u16;

		for card in &self.cards {
			let card_byte_int = card.to_byte_int();
			rank_bytes |= card_byte_int.1;
		}

		let mut straight_pattern = 0b11111u16;
		let mut j = 0;
		while j < 9 {
			if rank_bytes == straight_pattern {
				return (true, match j {
						0 => Rank::SIX,
						1 => Rank::SEVEN,
						2 => Rank::EIGHT,
						3 => Rank::NINE,
						4 => Rank::TEN,
						5 => Rank::JACK,
						6 => Rank::QUEEN,
						7 => Rank::KING,
						8 => Rank::ACE,
						_ => Rank::TWO
					}
				);
			} else {
				straight_pattern = straight_pattern << 1;
			}

			j += 1;
		}

		// ACE as ONE
		if rank_bytes == 0b1000000001111u16 {
			return (true, Rank::ACE);
		}

		return (false, Rank::TWO);
	}

	fn get_hand_rank(&self) -> (HandRank, Option<FnvHashMap<u16, u8>>) {
		if self.check_flush() {
			if self.check_straight().0 {
				return (HandRank::STRAIGHT_FLUSH, None);
			} else {
				return (HandRank::FLUSH, None);
			}
		} else {
			if self.check_straight().0 {
				return (HandRank::STRAIGHT, None);
			} else {
				let same_kind = self.check_same_kind();
				let freq = self.get_freq_tuple(&same_kind);

				if freq.0 {
					return (HandRank::FOUR_OF_A_KIND, Some(same_kind));
				} else if freq.1 && (freq.2 == 1) {
					return (HandRank::FULL_HOUSE, Some(same_kind));
				} else if freq.1 {
					return (HandRank::THREE_OF_A_KIND, Some(same_kind));
				} else if freq.2 == 2 {
					return (HandRank::TWO_PAIRS, Some(same_kind));
				} else if freq.2 == 1 {
					return (HandRank::PAIR, Some(same_kind));
				} else {
					return (HandRank::HIGH_CARD, Some(same_kind));
				}
			}
		}
	}

	fn get_freq_tuple(&self, freq_map: &FnvHashMap<u16, u8>) -> (bool, bool, u8) {
		let mut freq = (false,false,0);
		for (_, rank) in freq_map {
			if *rank == 4 {
				freq.0 = true;
			}

			if *rank == 3 {
				freq.1 = true;
			}

			if *rank == 2 {
				freq.2 += 1;
			}
		}

		return freq;
	}

	fn compare(&self, other: &Hand) -> Ordering {
		let self_hand_rank = self.get_hand_rank().0;
		let other_hand_rank = other.get_hand_rank().0;

		if self_hand_rank < other_hand_rank {
			return Ordering::Greater;
		} else if self_hand_rank > other_hand_rank {
			return Ordering::Less;
		} else {
			if self_hand_rank == HandRank::STRAIGHT_FLUSH ||
				self_hand_rank == HandRank::FLUSH ||
				self_hand_rank == HandRank::STRAIGHT ||
				self_hand_rank == HandRank::HIGH_CARD {

					let mut self_combined_ranks = 0u16;
					for card in &self.cards {
						self_combined_ranks |= card.to_byte_int().1;
					}

					let mut other_combined_ranks = 0u16;
					for card in &other.cards {
						other_combined_ranks |= card.to_byte_int().1;
					}

					if self_combined_ranks > other_combined_ranks {
						return Ordering::Greater;
					} else if self_combined_ranks < other_combined_ranks {
						return Ordering::Less;
					} else {
						return Ordering::Equal;
					}
			} else {
				let mut combos = [(0u16, 0u16), (0u16, 0u16), (0u16, 0u16), (0u16, 0u16)];

				for (scard, sval) in self.get_hand_rank().1.unwrap() {
					combos[(4-sval) as usize].0 |= scard;
				}

				for (ocard, oval) in other.get_hand_rank().1.unwrap() {
					combos[(4-oval) as usize].1 |= ocard;
				}

				let mut i = 0;
				let mut j;
				while i < 4 {
					if combos[i].0 == 0 {
						i += 1;
						continue;
					}

					if combos[i].0 > combos[i].1 {
						return Ordering::Greater;
					} else if combos[i].0 < combos[i].1 {
						return Ordering::Less;
					} else {
						j = i + 1;
						while j < 4 {
							if combos[j].0 == 0 {
								j += 1;
								continue;
							}

							if combos[j].0 > combos[j].1 {
								return Ordering::Greater;
							} else if combos[j].0 < combos[j].1 {
								return Ordering::Less;
							}

							j += 1;
						}
					}

					i += 1;
				}

				return Ordering::Equal;
			}
		}
	}
}