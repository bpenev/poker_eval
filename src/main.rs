mod hand;

extern crate rand;
extern crate chrono;

use std::env;
use std::time;

use rand::Rng;
use chrono::prelude::*;

use hand::Hand;
use hand::Rank;
use hand::Suit;
use hand::Card;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		panic!("One and only one argument <number of hands> required!");
	}

	let nr_h: usize = args[1].parse().unwrap();

	let mut hands: Vec<Hand> = Vec::with_capacity(nr_h);

	let mut rng = rand::thread_rng();

	let mut deck = Vec::new();
	for s in Suit::iterator() {
		for r in Rank::iterator() {
			deck.push(Card {rank:*r, suit: *s});
		}
	}

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

		hands.push(Hand {cards: hand_array});
		i += 1;
	}

	let mut r = 0;
	for h in &hands {
		if h.check_repeating_cards() {
			r += 1;
		}
	}

	let mut j = 0;
	let mut counts = (0,0,0);

	let utc_start: DateTime<Utc> = Utc::now();
	while j < hands.len()-1 {
		if hands[j] > hands[j+1] {
			counts.0 += 1;
		} else if hands[j] < hands[j+1] {
			counts.1 += 1;
		} else {
			counts.2 += 1;
		}

		j += 1;
	}
	let utc_end: DateTime<Utc> = Utc::now();

	println!("Total Hands: {}\nTotal Comparisons: {}\nFirst greater: {}\nSecond Greater: {}\nEqual: {}\n{:?}", 
		nr_h, j, counts.0, counts.1, counts.2, utc_end.signed_duration_since(utc_start));
}