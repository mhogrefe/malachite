use common::test_properties;
use malachite_base::num::{CeilingLogTwo, FloorLogTwo, One, PrimitiveInteger, SignificantBits, Zero};
use malachite_nz::natural::arithmetic::log_two::{limbs_ceiling_log_two, limbs_floor_log_two};
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::vecs_of_unsigned_var_1;
use malachite_test::inputs::natural::positive_naturals;
use std::str::FromStr;
use std::u32;

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

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn limbs_floor_log_two_fail() {
    limbs_floor_log_two(&[]);
}

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

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
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
#[should_panic(expected = "Cannot take the base-2 logarithm of 0.")]
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
#[should_panic(expected = "Cannot take the base-2 logarithm of 0.")]
fn ceiling_log_two_fail() {
    Natural::ZERO.ceiling_log_two();
}

#[test]
fn limbs_floor_log_two_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let floor_log_two = limbs_floor_log_two(limbs);
        assert_eq!(limbs.len() == 1, floor_log_two < u64::from(u32::WIDTH));
        assert_eq!(floor_log_two, limbs_significant_bits(limbs) - 1);
        assert_eq!(
            floor_log_two,
            Natural::from_limbs_asc(limbs).floor_log_two()
        );
        //TODO
        /*let n = floor_log_two as u32;
        assert!(Natural::ONE << n <= *x);
        assert!(*x < Natural::ONE << (n + 1));*/    });
}

#[test]
fn limbs_ceiling_log_two_properties() {
    test_properties(vecs_of_unsigned_var_1, |limbs| {
        let ceiling_log_two = limbs_ceiling_log_two(limbs);
        assert_eq!(
            limbs.len() == 1 || limbs == &[0, 1],
            ceiling_log_two <= u64::from(u32::WIDTH)
        );
        assert_eq!(
            ceiling_log_two,
            Natural::from_limbs_asc(limbs).ceiling_log_two()
        );
        //TODO
        /*let n = floor_log_two as u32;
        assert!(Natural::ONE << n <= *x);
        assert!(*x < Natural::ONE << (n + 1))*/    });
}

#[test]
fn floor_log_two_properties() {
    test_properties(positive_naturals, |x| {
        let floor_log_two = x.floor_log_two();
        assert_eq!(*x <= u32::MAX, floor_log_two < u64::from(u32::WIDTH));
        assert_eq!(floor_log_two, x.significant_bits() - 1);
        assert_eq!(floor_log_two, limbs_floor_log_two(&x.to_limbs_asc()));
        let n = floor_log_two as u32;
        assert!(Natural::ONE << n <= *x);
        assert!(*x < Natural::ONE << (n + 1));
    });
}

#[test]
fn ceiling_log_two_properties() {
    test_properties(positive_naturals, |x| {
        let ceiling_log_two = x.ceiling_log_two();
        assert_eq!(*x <= u32::MAX, ceiling_log_two <= u64::from(u32::WIDTH));
        assert_eq!(ceiling_log_two, limbs_ceiling_log_two(&x.to_limbs_asc()));
        let n = ceiling_log_two as u32;
        if n != 0 {
            assert!(Natural::ONE << (n - 1) < *x);
        }
        assert!(*x <= Natural::ONE << n);
    });
}
