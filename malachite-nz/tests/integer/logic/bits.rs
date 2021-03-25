use malachite_base::num::logic::traits::BitIterable;
use malachite_nz::integer::Integer;

#[test]
fn test_bits() {
    let n = Integer::from(-105);
    let mut bits = n.bits();
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

    let mut bits = n.bits();
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
