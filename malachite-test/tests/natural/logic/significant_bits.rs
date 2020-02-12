use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::FloorLogTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::log_two::limbs_floor_log_two;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::base::{unsigneds, vecs_of_unsigned_var_1};
use malachite_test::inputs::natural::naturals;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_significant_bits() {
    let test = |limbs, out| {
        assert_eq!(limbs_significant_bits(limbs), out);
    };
    test(&[0b1], 1);
    test(&[0b10], 2);
    test(&[0b11], 2);
    test(&[0b100], 3);
    test(&[0, 0b1], 33);
    test(&[0, 0b1101], 36);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_significant_bits_fail() {
    limbs_significant_bits(&[]);
}

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(
            u64::wrapping_from(BigUint::from_str(n).unwrap().bits()),
            out
        );
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
fn limbs_significant_bits_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let significant_bits = limbs_significant_bits(limbs);
        assert_eq!(limbs.len() == 1, significant_bits <= Limb::WIDTH);
        assert_eq!(significant_bits, limbs_floor_log_two(limbs) + 1);
        assert_eq!(
            significant_bits,
            Natural::from_limbs_asc(limbs).significant_bits()
        );
    });
}

#[test]
fn significant_bits_properties() {
    test_properties(naturals, |x| {
        let significant_bits = x.significant_bits();
        assert_eq!(
            u64::wrapping_from(natural_to_biguint(x).bits()),
            significant_bits
        );
        assert_eq!(
            u64::from(natural_to_rug_integer(x).significant_bits()),
            significant_bits
        );
        assert_eq!(*x <= Limb::MAX, significant_bits <= Limb::WIDTH);
        if *x != 0 {
            assert_eq!(significant_bits, x.floor_log_two() + 1);
            assert_eq!(significant_bits, limbs_significant_bits(&x.to_limbs_asc()));
            assert!(Natural::ONE << (significant_bits - 1) <= *x);
            assert!(*x < Natural::ONE << significant_bits);
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(Natural::from(u).significant_bits(), u.significant_bits());
    });
}
