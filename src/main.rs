mod hand;

extern crate rand;
extern crate chrono;
extern crate num_cpus;

use std::env;
use std::thread;
use std::sync::mpsc;

use math::round;

use rand::Rng;
use chrono::DateTime;
use chrono::Utc;

use hand::Hand;
use hand::Rank;
use hand::Suit;
use hand::Card;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		panic!("Only one argument <number of hands> must be supplied!");
	}
	let nr_h: usize = args[1].parse().unwrap();
	let num_of_threads: usize = num_cpus::get();

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

	if r > 0 {
		panic!("Cards repeat in hands.");
	}

	let utc_start: DateTime<Utc> = Utc::now();

	let (tx, rx) = mpsc::channel();

	let mut hands_part: Vec<Vec<Hand>> = Vec::new();
	for _ in 1..num_of_threads {
		for chunk in hands.chunks(round::ceil(hands.len() as f64 / num_of_threads as f64, 0) as usize) {
			hands_part.push(chunk.to_vec());
		}
	}

	for i in 0..num_of_threads {
		let tx_thread = mpsc::Sender::clone(&tx);
		let chunk_thread = hands_part[i].clone();

		thread::spawn(move || {
			let mut j = 0;
			let mut counts = (0,0,0);
	        while j < chunk_thread.len()-1 {
				if chunk_thread[j] > chunk_thread[j+1] {
					counts.0 += 1;
				} else if chunk_thread[j] < chunk_thread[j+1] {
					counts.1 += 1;
				} else {
					counts.2 += 1;
				}

				j += 1;
			}

			tx_thread.send(counts).unwrap();
	    });
	}

	let mut counts = (0,0,0);
	let mut rx_count = 0;
	for received in rx {
	    counts.0 += received.0;
	    counts.1 += received.1;
	    counts.2 += received.2;

	    rx_count += 1;
	    if rx_count == num_of_threads {
	    	break;
	    }
	}

	let utc_end: DateTime<Utc> = Utc::now();

	println!("Total Hands: {}\nFirst greater: {}\nSecond greater: {}\nEqual: {}\n{:?}", 
		nr_h, counts.0, counts.1, counts.2, utc_end.signed_duration_since(utc_start));
}