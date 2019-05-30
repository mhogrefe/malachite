use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    CeilingLogTwo, IsPowerOfTwo, NextPowerOfTwo, NextPowerOfTwoAssign,
};
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::arithmetic::next_power_of_two::{
    limbs_next_power_of_two, limbs_slice_next_power_of_two_in_place,
    limbs_vec_next_power_of_two_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use common::test_properties;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{unsigneds, vecs_of_unsigned_var_1};
use malachite_test::inputs::natural::naturals;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_next_power_of_two_and_limbs_vec_next_power_of_two_in_place() {
    let test = |limbs: &[Limb], out: &[Limb]| {
        assert_eq!(limbs_next_power_of_two(limbs), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_next_power_of_two_in_place(&mut limbs);
        assert_eq!(limbs, out);
    };
    test(&[3], &[4]);
    test(&[6, 7], &[0, 8]);
    test(&[100, 101, 102], &[0, 0, 128]);
    test(&[123, 456], &[0, 512]);
    test(&[123, 456, 0xffff_ffff], &[0, 0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_next_power_of_two_fail() {
    limbs_next_power_of_two(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_next_power_of_two_in_place_fail() {
    limbs_slice_next_power_of_two_in_place(&mut []);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_next_power_of_two_in_place_fail() {
    limbs_vec_next_power_of_two_in_place(&mut Vec::new());
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_next_power_of_two_in_place() {
    let test = |limbs: &[Limb], carry: bool, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_next_power_of_two_in_place(&mut limbs), carry);
        assert_eq!(limbs, out);
    };
    test(&[3], false, &[4]);
    test(&[6, 7], false, &[0, 8]);
    test(&[100, 101, 102], false, &[0, 0, 128]);
    test(&[123, 456], false, &[0, 512]);
    test(&[123, 456, 0xffff_ffff], true, &[0, 0, 0]);
}

#[test]
fn test_next_power_of_two() {
    let test = |u, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.next_power_of_two_assign();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().next_power_of_two();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).next_power_of_two();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap().next_power_of_two();
        assert_eq!(n.to_string(), out);
    };
    test("0", "1");
    test("1", "1");
    test("2", "2");
    test("3", "4");
    test("4", "4");
    test("5", "8");
    test("6", "8");
    test("7", "8");
    test("8", "8");
    test("9", "16");
    test("10", "16");
    test("123", "128");
    test("1000", "1024");
    test("1000000", "1048576");
    test("1000000000", "1073741824");
    test("1000000000000", "1099511627776");
    test("1073741823", "1073741824");
    test("1073741824", "1073741824");
    test("1073741825", "2147483648");
    test("2147483647", "2147483648");
    test("2147483648", "2147483648");
    test("2147483649", "4294967296");
    test("21344980687", "34359738368");
}

#[test]
fn limbs_next_power_of_two_properties() {
    test_properties(vecs_of_unsigned_var_1, |ref limbs| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_next_power_of_two(limbs)),
            Natural::from_limbs_asc(limbs).next_power_of_two(),
        );
    });
}

#[test]
fn limbs_slice_next_power_of_two_in_place_properties() {
    test_properties(vecs_of_unsigned_var_1, |&ref limbs| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        let carry = limbs_slice_next_power_of_two_in_place(&mut limbs);
        let n = Natural::from_limbs_asc(&old_limbs).next_power_of_two();
        let mut expected_limbs = n.into_limbs_asc();
        assert_eq!(carry, expected_limbs.len() == limbs.len() + 1);
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(limbs, expected_limbs);
    });
}

#[test]
fn limbs_vec_next_power_of_two_in_place_properties() {
    test_properties(vecs_of_unsigned_var_1, |ref limbs| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        limbs_vec_next_power_of_two_in_place(&mut limbs);
        let n = Natural::from_limbs_asc(&old_limbs).next_power_of_two();
        assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
    });
}

#[test]
fn next_power_of_two_properties() {
    test_properties(naturals, |n| {
        let mut mut_n = n.clone();
        mut_n.next_power_of_two_assign();
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = n.next_power_of_two();
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().next_power_of_two();
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = rug_integer_to_natural(&natural_to_rug_integer(n).next_power_of_two());
        assert_eq!(result_alt, result);

        assert!(result.is_power_of_two());
        assert!(result >= *n);
        if *n != 0 as Limb {
            assert!(&result >> 1 < *n);
            assert_eq!(Natural::ONE << n.ceiling_log_two(), result);
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        if let Some(power) = u.checked_next_power_of_two() {
            assert_eq!(power, Natural::from(u).next_power_of_two());
        }
    });
}
