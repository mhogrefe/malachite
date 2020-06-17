use malachite_base::num::arithmetic::traits::{
    CeilingLogTwo, IsPowerOfTwo, NextPowerOfTwo, NextPowerOfTwoAssign, PowerOfTwo,
};
use malachite_nz::natural::arithmetic::next_power_of_two::{
    limbs_next_power_of_two, limbs_slice_next_power_of_two_in_place,
    limbs_vec_next_power_of_two_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_rug_integer, rug_integer_to_natural};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{unsigneds, vecs_of_unsigned_var_1};
use malachite_test::inputs::natural::naturals;

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
        if *n != 0 {
            assert!(&result >> 1 < *n);
            assert_eq!(Natural::power_of_two(n.ceiling_log_two()), result);
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        if let Some(power) = u.checked_next_power_of_two() {
            assert_eq!(Natural::from(u).next_power_of_two(), u.next_power_of_two());
            assert_eq!(power, Natural::from(u).next_power_of_two());
        }
    });
}
