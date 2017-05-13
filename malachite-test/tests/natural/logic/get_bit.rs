use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::natural::logic::get_bit::num_get_bit;
use num;
use std::str::FromStr;

#[test]
pub fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(num_get_bit(&num::BigUint::from_str(n).unwrap(), index), out);
    };

    test("0", 0, false);
    test("0", 100, false);
    test("123", 2, false);
    test("123", 3, true);
    test("123", 100, false);
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
}
