mod hand;

use hand::Hand;
use hand::Rank;
use hand::Suit;
use hand::Card;

use fnv::FnvHashMap;

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
	
	let mut hands_ranked: Vec<(Hand, u32)> = Vec::with_capacity(2598960);
	for i in 0..hands.len() {
		if i == 0 {
			hands_ranked.push((hands[i], 1));
		} else if hands[i] < hands[i-1] {
			hands_ranked.push((hands[i], hands_ranked[hands_ranked.len()-1].1 + 1));
		} else {
			hands_ranked.push((hands[i], hands_ranked[hands_ranked.len()-1].1));
		}
	}
	
	let mut hand_map = FnvHashMap::with_capacity_and_hasher(2598960, Default::default());
	for h in hands_ranked {
		hand_map.insert(h.0.to_ordered_string(), h.1);
	}
	
	let nr_h = 1000000;
	let mut rng = rand::thread_rng();
	let mut hands_test: Vec<String> = Vec::with_capacity(nr_h);
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

		hands_test.push((Hand {cards: hand_array}).to_ordered_string());
		i += 1;
	}
	
	let utc_start: DateTime<Utc> = Utc::now();
	let mut hrank = 0;
	for h in hands_test {
		if let Some(r) = hand_map.get(&h) {
			hrank += r;
		} else {
			panic!("Hand not in map! {}", h);
		}
	}
	let utc_end: DateTime<Utc> = Utc::now();
	println!("Total Hands: {}\nAverage Rank: {}\nMH/s: {}", nr_h, hrank as f32 / nr_h as f32, 
		((nr_h/1000000) as f32 / (utc_end.signed_duration_since(utc_start).num_milliseconds() as f32 / 1000 as f32)));
}