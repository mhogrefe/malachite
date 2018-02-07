use common::test_properties;
use malachite_base::num::SignificantBits;
use malachite_base::num::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::integer::integers;
use num::BigInt;
use rug;
use std::u32;
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(BigInt::from_str(n).unwrap().bits() as u64, out);
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

#[test]
fn significant_bits_properties() {
    test_properties(integers, |x| {
        let significant_bits = x.significant_bits();
        assert_eq!(integer_to_bigint(x).bits() as u64, significant_bits);
        assert_eq!(
            u64::from(integer_to_rug_integer(x).significant_bits()),
            significant_bits
        );

        let x_abs = x.abs_ref();
        assert_eq!(x_abs <= u32::MAX, significant_bits <= 32);
        if x_abs != 0 {
            let n = significant_bits as u32;
            assert!(Natural::ONE << (n - 1) <= x_abs);
            assert!(x_abs < Natural::ONE << n);
        }
    });
}
