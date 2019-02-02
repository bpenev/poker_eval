mod hand;

use hand::Hand;

fn main() {
	let h = Hand::new_from_string("6c 6s 8h 7d 7h".to_string());
	
	println!("Hand: {}", h.to_string());

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
}