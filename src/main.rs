mod hand;

use hand::Hand;
use hand::Rank;
use hand::Suit;
use hand::Card;

use rand::Rng;
use chrono::DateTime;
use chrono::Utc;

fn main() {
	let mut hands: Vec<Hand> = Vec::with_capacity(2598960);
	
	let mut deck = Vec::new();
	for s in Suit::iterator() {
		for r in Rank::iterator() {
			deck.push(Card {rank:*r, suit: *s});
		}
	}
	
	for a in 0..deck.len() {
		for b in a+1..deck.len() {
			for c in b+1..deck.len() {
				for d in c+1..deck.len() {
					for e in d+1..deck.len() {
						hands.push(Hand {cards: [deck[a],deck[b],deck[c],deck[d],deck[e]]});
					}
				}
			}
		}
	}
	
	hands.sort();
	hands.reverse();
	
	let mut hands_ranked: Vec<(String, u32)> = Vec::with_capacity(2598960);
	for i in 0..hands.len() {
		if i == 0 {
			hands_ranked.push((hands[i].to_ordered_string(), 1));
		} else if hands[i] < hands[i-1] {
			hands_ranked.push((hands[i].to_ordered_string(), hands_ranked[hands_ranked.len()-1].1 + 1));
		} else {
			hands_ranked.push((hands[i].to_ordered_string(), hands_ranked[hands_ranked.len()-1].1));
		}
	}

	let mut hands_ranked_int = vec![0; 134217728];
	for i in 0..hands_ranked.len() {
		let h = Hand::new_from_string(hands_ranked[i].0.to_string());
		hands_ranked_int[h.to_int()] = hands_ranked[i].1;
	}
	
	let nr_h = 10_000_000;
	let mut rng = rand::thread_rng();
	let mut hands_test: Vec<Hand> = Vec::with_capacity(nr_h);
	let mut i = 0;
	while i < nr_h {
		let mut hand_array = [Card{rank: Rank::TWO, suit: Suit::CLUBS}; 5];
		let mut picked_elements = [52; 5];

		let mut j = 0;
		while j < 5 {
			let e = rng.gen_range(0, 52);

			let mut clash = false;
			for pe in &picked_elements {
				if e == *pe {
					clash = true;
					break;
				}
			}

			if clash {
				continue;
			}

			picked_elements[j] = e;
			hand_array[j] = deck[e];
			j += 1;
		}

		hands_test.push(Hand {cards: hand_array});
		i += 1;
	}
	
	let utc_start: DateTime<Utc> = Utc::now();
	for h in hands_test {
		hands_ranked_int[h.to_int()];
	}
	let utc_end: DateTime<Utc> = Utc::now();

	println!("Total Hands: {}\nMH/s: {}\n{:?}", nr_h, 
		((nr_h as f64/1000000 as f64) / (utc_end.signed_duration_since(utc_start).num_milliseconds() as f64 / 1000 as f64)), 
		utc_end.signed_duration_since(utc_start));
}