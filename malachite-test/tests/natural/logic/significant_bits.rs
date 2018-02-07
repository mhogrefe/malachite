use common::test_properties;
use malachite_base::num::{PrimitiveInteger, SignificantBits};
use malachite_base::num::One;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::natural::naturals;
use num::BigUint;
use rug;
use std::str::FromStr;
use std::u32;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(BigUint::from_str(n).unwrap().bits() as u64, out);
        assert_eq!(
            u64::from(rug::Integer::from_str(n).unwrap().significant_bits()),
            out
        );
    };
    test("0", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 33);
    test("18446744073709551615", 64);
    test("18446744073709551616", 65);
}

#[test]
fn significant_bits_properties() {
    test_properties(naturals, |x| {
        let significant_bits = x.significant_bits();
        assert_eq!(natural_to_biguint(x).bits() as u64, significant_bits);
        assert_eq!(
            u64::from(natural_to_rug_integer(x).significant_bits()),
            significant_bits
        );
        assert_eq!(*x <= u32::MAX, significant_bits <= u64::from(u32::WIDTH));
        if *x != 0 {
            let n = significant_bits as u32;
            assert!(Natural::ONE << (n - 1) <= *x);
            assert!(*x < Natural::ONE << n);
        }
    });
}
