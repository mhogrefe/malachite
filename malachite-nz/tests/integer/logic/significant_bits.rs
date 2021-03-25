use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(u64::wrapping_from(BigInt::from_str(n).unwrap().bits()), out);
        assert_eq!(
            u64::from(rug::Integer::from_str(n).unwrap().significant_bits()),
            out
        );
    };
    test("0", 0);
    test("100", 7);
    test("-100", 7);
    test("1000000000000", 40);
    test("-1000000000000", 40);
}
