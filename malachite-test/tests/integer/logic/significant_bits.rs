use common::LARGE_LIMIT;
use malachite_base::num::SignificantBits;
use malachite_base::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use malachite_test::integer::logic::significant_bits::select_inputs;
use num::BigInt;
use rugint;
use std::u32;
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(BigInt::from_str(n).unwrap().bits() as u64, out);
        assert_eq!(
            u64::from(rugint::Integer::from_str(n).unwrap().significant_bits()),
            out
        );
    };
    test("0", 0);
    test("100", 7);
    test("-100", 7);
    test("1000000000000", 40);
    test("-1000000000000", 40);
}

#[test]
fn significant_bits_properties() {
    // x.significant_bits() is equivalent for malachite, num, and rugint.
    // (|x| < 2^32) == (x.significant_bits() <= 32)
    // if x != 0, (x.significant_bits() == n) == (2^(n-1) <= |x| < 2^n)
    let one_integer = |x: Integer| {
        let significant_bits = x.significant_bits();
        assert_eq!(integer_to_bigint(&x).bits() as u64, significant_bits);
        assert_eq!(
            u64::from(integer_to_rugint_integer(&x).significant_bits()),
            significant_bits
        );

        let x_abs = x.abs();
        assert_eq!(x_abs <= u32::MAX, significant_bits <= 32);
        if x_abs != 0 {
            let n = significant_bits as u32;
            assert!(Natural::ONE << (n - 1) <= x_abs);
            assert!(x_abs < Natural::ONE << n);
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
