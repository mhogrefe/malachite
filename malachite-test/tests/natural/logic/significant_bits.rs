use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().significant_bits(),
                   out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(num::BigUint::from_str(n).unwrap().bits() as u64, out);
    };
    test("0", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 33);
    test("18446744073709551615", 64);
    test("18446744073709551616", 65);
}
