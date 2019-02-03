mod hand;

use hand::Hand;

fn main() {
	let h = Hand::new_from_string("6c 6s 8h 8d 8c".to_string());
	
	println!("Hand: {}", h.to_string());

	if h.check_repeating_cards() {
		println!("Repeating cards!");
	}
}