use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().significant_bits(),
                   out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(num::BigInt::from_str(n).unwrap().bits() as u64, out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().significant_bits() as u64,
                   out);
    };
    test("0", 0);
    test("100", 7);
    test("-100", 7);
    test("1000000000000", 40);
    test("-1000000000000", 40);
}
