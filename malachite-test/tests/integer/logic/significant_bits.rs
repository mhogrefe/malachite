use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::{Abs, One, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use num::BigInt;
use rug;

use common::test_properties;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;

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

        let x_abs = x.abs();
        assert_eq!(
            x_abs <= Limb::MAX,
            significant_bits <= u64::from(Limb::WIDTH)
        );
        if x_abs != 0 as Limb {
            assert!(Natural::ONE << (significant_bits - 1) <= x_abs);
            assert!(x_abs < Natural::ONE << significant_bits);
        }
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(Integer::from(i).significant_bits(), i.significant_bits());
    });
}
