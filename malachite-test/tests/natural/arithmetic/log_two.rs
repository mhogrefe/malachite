use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{CeilingLogTwo, FloorLogTwo};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::log_two::{limbs_ceiling_log_two, limbs_floor_log_two};
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{positive_unsigneds, vecs_of_unsigned_var_1};
use malachite_test::inputs::natural::positive_naturals;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_floor_log_two() {
    let test = |limbs, out| {
        assert_eq!(limbs_floor_log_two(limbs), out);
    };
    test(&[0b1], 0);
    test(&[0b10], 1);
    test(&[0b11], 1);
    test(&[0b100], 2);
    test(&[0, 0b1], 32);
    test(&[0, 0b1101], 35);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_floor_log_two_fail() {
    limbs_floor_log_two(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_ceiling_log_two() {
    let test = |limbs, out| {
        assert_eq!(limbs_ceiling_log_two(limbs), out);
    };
    test(&[0b1], 0);
    test(&[0b10], 1);
    test(&[0b11], 2);
    test(&[0b100], 2);
    test(&[0, 0b1], 32);
    test(&[0, 0b1101], 36);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_ceiling_log_two_fail() {
    limbs_ceiling_log_two(&[]);
}

#[test]
fn test_floor_log_two() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().floor_log_two(), out);
    };
    test("1", 0);
    test("100", 6);
    test("1000000000000", 39);
    test("4294967295", 31);
    test("4294967296", 32);
    test("18446744073709551615", 63);
    test("18446744073709551616", 64);
}

#[test]
#[should_panic]
fn floor_log_two_fail() {
    Natural::ZERO.floor_log_two();
}

#[test]
fn test_ceiling_log_two() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().ceiling_log_two(), out);
    };
    test("1", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 32);
    test("4294967297", 33);
    test("18446744073709551615", 64);
    test("18446744073709551616", 64);
    test("18446744073709551617", 65);
}

#[test]
#[should_panic]
fn ceiling_log_two_fail() {
    Natural::ZERO.ceiling_log_two();
}

#[test]
fn limbs_floor_log_two_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let floor_log_two = limbs_floor_log_two(limbs);
        assert_eq!(limbs.len() == 1, floor_log_two < Limb::WIDTH);
        assert_eq!(floor_log_two, limbs_significant_bits(limbs) - 1);
        assert_eq!(
            floor_log_two,
            Natural::from_limbs_asc(limbs).floor_log_two()
        );
    });
}

#[test]
fn limbs_ceiling_log_two_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let ceiling_log_two = limbs_ceiling_log_two(limbs);
        assert_eq!(
            limbs.len() == 1 || limbs == &[0, 1],
            ceiling_log_two <= Limb::WIDTH
        );
        assert_eq!(
            ceiling_log_two,
            Natural::from_limbs_asc(limbs).ceiling_log_two()
        );
    });
}

#[test]
fn floor_log_two_properties() {
    test_properties(positive_naturals, |x| {
        let floor_log_two = x.floor_log_two();
        assert_eq!(*x <= Limb::MAX, floor_log_two < Limb::WIDTH);
        assert_eq!(floor_log_two, x.significant_bits() - 1);
        assert_eq!(floor_log_two, limbs_floor_log_two(&x.to_limbs_asc()));
        assert!(Natural::ONE << floor_log_two <= *x);
        assert!(*x < Natural::ONE << (floor_log_two + 1));
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        assert_eq!(u.floor_log_two(), Natural::from(u).floor_log_two());
    });
}

#[test]
fn ceiling_log_two_properties() {
    test_properties(positive_naturals, |x| {
        let ceiling_log_two = x.ceiling_log_two();
        assert_eq!(*x <= Limb::MAX, ceiling_log_two <= Limb::WIDTH);
        assert_eq!(ceiling_log_two, limbs_ceiling_log_two(&x.to_limbs_asc()));
        if ceiling_log_two != 0 {
            assert!(Natural::ONE << (ceiling_log_two - 1) < *x);
        }
        assert!(*x <= Natural::ONE << ceiling_log_two);
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        assert_eq!(u.ceiling_log_two(), Natural::from(u).ceiling_log_two());
    });
}
