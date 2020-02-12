use malachite_base::num::logic::traits::BitIterable;

#[test]
pub fn test_bits() {
    let mut bits = 105u8.bits();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], false);
    assert_eq!(bits[2], false);
    assert_eq!(bits[3], true);
    assert_eq!(bits[4], false);
    assert_eq!(bits[5], true);
    assert_eq!(bits[6], true);
    assert_eq!(bits[7], false);
    assert_eq!(bits[8], false);

    let mut bits = 105u32.bits();
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    let mut bits = (-105i8).bits();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], true);
    assert_eq!(bits[2], true);
    assert_eq!(bits[3], false);
    assert_eq!(bits[4], true);
    assert_eq!(bits[5], false);
    assert_eq!(bits[6], false);
    assert_eq!(bits[7], true);
    assert_eq!(bits[8], true);

    let mut bits = (-105i32).bits();
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);
}
