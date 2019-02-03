use super::*;

#[test]
fn check_pair() {
	let h = Hand::new_from_string("6d 7h 8h 7c Tc".to_string());
    assert_eq!(h.check_same_kind().2, 1);

    let h = Hand::new_from_string("Ad Ah 8h 7c Tc".to_string());
    assert_eq!(h.check_same_kind().2, 1);

    let h = Hand::new_from_string("6c 7c 8c Ac Td".to_string());
    assert_eq!(h.check_same_kind().2, 0);

    let h = Hand::new_from_string("6c 7c 8c Tc Td".to_string());
    assert_eq!(h.check_same_kind().2, 1);

    let h = Hand::new_from_string("Ad Ah 8h 8c Tc".to_string());
    assert_ne!(h.check_same_kind().2, 1);

    let h = Hand::new_from_string("Ad Ah 9d Ac 2d".to_string());
    assert_ne!(h.check_same_kind().2, 1);

    let h = Hand::new_from_string("Ad Ah 8h Ac As".to_string());
    assert_ne!(h.check_same_kind().2, 1);
}

#[test]
fn check_two_pairs() {
	let h = Hand::new_from_string("6c Ts Ad Kh Jc".to_string());
    assert_ne!(h.check_same_kind().2, 2);

    let h = Hand::new_from_string("Ac As 4d Kh Jc".to_string());
    assert_ne!(h.check_same_kind().2, 2);

    let h = Hand::new_from_string("Tc As Ad Kh Ac".to_string());
    assert_ne!(h.check_same_kind().2, 2);

    let h = Hand::new_from_string("Tc As Ad Ah Ac".to_string());
    assert_ne!(h.check_same_kind().2, 2);

    let h = Hand::new_from_string("Tc Ts Ad Kh Ac".to_string());
    assert_eq!(h.check_same_kind().2, 2);

    let h = Hand::new_from_string("Tc 8s Td 2h 2c".to_string());
    assert_eq!(h.check_same_kind().2, 2);

    let h = Hand::new_from_string("Tc 8s Td 8h 2c".to_string());
    assert_eq!(h.check_same_kind().2, 2);
}

#[test]
fn check_three_of_a_kind() {
	let h = Hand::new_from_string("6c 6s 6d Ah Tc".to_string());
    assert_eq!(h.check_same_kind().1, true);

    let h = Hand::new_from_string("6c 7c 8c 9c Td".to_string());
    assert_eq!(h.check_same_kind().1, false);

    let h = Hand::new_from_string("Ad Ah 8h Ac Tc".to_string());
    assert_eq!(h.check_same_kind().1, true);
}

#[test]
fn check_straight() {
	let h = Hand::new_from_string("6h 7c 8d 9c Tc".to_string());
    assert_eq!(h.check_straight(), (true, Rank::TEN));

    let h = Hand::new_from_string("Kh Qc Td Jc Ac".to_string());
    assert_eq!(h.check_straight(), (true, Rank::ACE));

    let h = Hand::new_from_string("2h 3c 4d Ac 5c".to_string());
    assert_eq!(h.check_straight(), (true, Rank::ACE));

    let h = Hand::new_from_string("6h 7c 8d 9c Jc".to_string());
    assert_eq!(h.check_straight().0, (false));
}

#[test]
fn check_flush() {
	let h = Hand::new_from_string("6c 7c 8c 9c Tc".to_string());
    assert_eq!(h.check_flush(), true);

    let h = Hand::new_from_string("6c 7c 8c 9c Td".to_string());
    assert_eq!(h.check_flush(), false);
}

#[test]
fn check_full_house() {
	let h = Hand::new_from_string("6c 6s 6d 7h 7c".to_string());
    assert_eq!(h.check_same_kind().1, true);
    assert_eq!(h.check_same_kind().2, 1);

    let h = Hand::new_from_string("Ac Ad 5s 5h Ah".to_string());
    assert_eq!(h.check_same_kind().1, true);
    assert_eq!(h.check_same_kind().2, 1);

    let h = Hand::new_from_string("Ac 5s Ad 5h Ah".to_string());
    assert_eq!(h.check_same_kind().1, true);
    assert_eq!(h.check_same_kind().2, 1);
}

#[test]
fn check_four_of_a_kind() {
	let h = Hand::new_from_string("6c 6s 6d 6h Tc".to_string());
    assert_eq!(h.check_same_kind().0, true);

    let h = Hand::new_from_string("6c 7c 8c 9c Td".to_string());
    assert_eq!(h.check_same_kind().0, false);
}

#[test]
fn check_straight_flush() {
	let h = Hand::new_from_string("6c 7c Tc 9c 8c".to_string());
    assert_eq!(h.check_flush(), true);
    assert_eq!(h.check_straight(), (true, Rank::TEN));

    let h = Hand::new_from_string("Ac Tc Kc Qc Jc".to_string());
    assert_eq!(h.check_flush(), true);
    assert_eq!(h.check_straight(), (true, Rank::ACE));

    let h = Hand::new_from_string("6c 7c Th 9c 8c".to_string());
    assert_ne!(h.check_flush(), true);
    assert_eq!(h.check_straight(), (true, Rank::TEN));

    let h = Hand::new_from_string("8c 7c Th 9c 6c".to_string());
    assert_eq!(h.check_flush(), false);
    assert_eq!(h.check_straight(), (true, Rank::TEN));
}

#[test]
fn check_hand_compare() {
	let h = Hand::new_from_string("6c 7c Tc 9c 8c".to_string());
	let o = Hand::new_from_string("6c 7c Tc 9c 8c".to_string());
	assert_eq!(h == o, true);

	let h = Hand::new_from_string("6c 7c Ac 9c 8c".to_string());
	let o = Hand::new_from_string("6c 7c Tc 9c 8c".to_string());
	assert_eq!(h == o, false);

	let h = Hand::new_from_string("6c 7c Ac 9c 8c".to_string());
	let o = Hand::new_from_string("6c 7c Tc 9h 8c".to_string());
	assert_eq!(h == o, false);

	let h = Hand::new_from_string("3h 4c 6c Td Tc".to_string());
	let o = Hand::new_from_string("3h 4c 6c 7d 9c".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("3h 3s 6c Td Tc".to_string());
	let o = Hand::new_from_string("3h 4c 6c Td Tc".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("3h Th 6c Td Tc".to_string());
	let o = Hand::new_from_string("3h 4c 6c Td Tc".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("3h Th 6c Td Tc".to_string());
	let o = Hand::new_from_string("3h 3c 6c Td Tc".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("3h Th 6c Td Tc".to_string());
	let o = Hand::new_from_string("3h 4c Ts Td Tc".to_string());
	assert_eq!(h > o, false);

	let h = Hand::new_from_string("3h Th 6c Td Tc".to_string());
	let o = Hand::new_from_string("3h 4c 6c Td Tc".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("6s 7s 8c 9s Ts".to_string());
	let o = Hand::new_from_string("3h Th 6c Td Tc".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("6s 7s 2s 9s Ts".to_string());
	let o = Hand::new_from_string("6s 7s 8c 9s Ts".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("2s 2c Jh Jd Js".to_string());
	let o = Hand::new_from_string("6s 7s 2s 9s Ts".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("5s 5c 5h 5d Js".to_string());
	let o = Hand::new_from_string("2s 2c Jh Jd Js".to_string());
	assert_eq!(h > o, true);

	let h = Hand::new_from_string("7s 8s 9s Ts Js".to_string());
	let o = Hand::new_from_string("5s 5c 5h 5d Js".to_string());
	assert_eq!(h > o, true);
}