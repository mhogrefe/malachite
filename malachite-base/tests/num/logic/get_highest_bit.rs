use malachite_base::num::integers::PrimitiveInteger;

#[test]
pub fn test_get_highest_bit() {
    assert_eq!(0u8.get_highest_bit(), false);
    assert_eq!(123u32.get_highest_bit(), false);
    assert_eq!(4_000_000_000u32.get_highest_bit(), true);
    assert_eq!(2_000_000_000i32.get_highest_bit(), false);
    assert_eq!((-2_000_000_000i32).get_highest_bit(), true);
}
