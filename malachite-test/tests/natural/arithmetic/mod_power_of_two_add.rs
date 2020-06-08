use malachite_base::num::arithmetic::traits::{
    ModAdd, ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoAddAssign, ModPowerOfTwoIsReduced,
    ModPowerOfTwoNeg, ModPowerOfTwoShl, ModPowerOfTwoSub, PowerOfTwo,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::natural::arithmetic::mod_power_of_two_add::{
    limbs_mod_power_of_two_add, limbs_mod_power_of_two_add_greater,
    limbs_mod_power_of_two_add_in_place_either, limbs_mod_power_of_two_add_limb,
    limbs_slice_mod_power_of_two_add_greater_in_place_left,
    limbs_slice_mod_power_of_two_add_limb_in_place, limbs_vec_mod_power_of_two_add_in_place_left,
    limbs_vec_mod_power_of_two_add_limb_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    triples_of_limb_vec_limb_and_u64_var_1, triples_of_limb_vec_limb_and_u64_var_2,
    triples_of_limb_vec_limb_vec_and_u64_var_13, triples_of_limb_vec_limb_vec_and_u64_var_14,
    triples_of_unsigned_unsigned_and_small_u64_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_u64_var_1, quadruples_of_three_naturals_and_u64_var_1,
    triples_of_natural_natural_and_u64_var_1,
};

#[test]
fn limbs_mod_power_of_two_add_limb_properties() {
    test_properties(
        triples_of_limb_vec_limb_and_u64_var_1,
        |&(ref xs, y, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_add_limb(xs, y, pow)),
                Natural::from_limbs_asc(xs).mod_power_of_two_add(Natural::from(y), pow),
            );
        },
    );
}

#[test]
fn limbs_slice_mod_power_of_two_add_limb_in_place_properties() {
    test_properties(
        triples_of_limb_vec_limb_and_u64_var_1,
        |&(ref xs, y, pow)| {
            let mut xs = xs.to_vec();
            let old_xs = xs.clone();
            let carry = limbs_slice_mod_power_of_two_add_limb_in_place(&mut xs, y, pow);
            let n = Natural::from_limbs_asc(&old_xs).mod_power_of_two_add(Natural::from(y), pow);
            let mut expected_limbs = n.into_limbs_asc();
            assert_eq!(carry, expected_limbs.len() == xs.len() + 1);
            expected_limbs.resize(xs.len(), 0);
            assert_eq!(xs, expected_limbs);
        },
    );
}

#[test]
fn limbs_vec_mod_power_of_two_add_limb_in_place_properties() {
    test_properties(
        triples_of_limb_vec_limb_and_u64_var_2,
        |&(ref xs, y, pow)| {
            let mut xs = xs.to_vec();
            let old_xs = xs.clone();
            limbs_vec_mod_power_of_two_add_limb_in_place(&mut xs, y, pow);
            let n = Natural::from_limbs_asc(&old_xs).mod_power_of_two_add(Natural::from(y), pow);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

fn limbs_mod_power_of_two_add_helper(
    f: &dyn Fn(&[Limb], &[Limb], u64) -> Vec<Limb>,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
    pow: u64,
) {
    assert_eq!(
        Natural::from_owned_limbs_asc(f(xs, ys, pow)),
        Natural::from_limbs_asc(xs).mod_power_of_two_add(Natural::from_limbs_asc(ys), pow)
    );
}

#[test]
fn limbs_mod_power_of_two_add_greater_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_14,
        |&(ref xs, ref ys, pow)| {
            limbs_mod_power_of_two_add_helper(&limbs_mod_power_of_two_add_greater, xs, ys, pow);
        },
    );
}

#[test]
fn limbs_mod_power_of_two_add_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_13,
        |&(ref xs, ref ys, pow)| {
            limbs_mod_power_of_two_add_helper(&limbs_mod_power_of_two_add, xs, ys, pow);
        },
    );
}

#[test]
fn limbs_slice_mod_power_of_two_add_greater_in_place_left_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_14,
        |&(ref xs, ref ys, pow)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            let carry = limbs_slice_mod_power_of_two_add_greater_in_place_left(&mut xs, ys, pow);
            let n = Natural::from_owned_limbs_asc(xs_old)
                .mod_power_of_two_add(Natural::from_limbs_asc(ys), pow);
            let len = xs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, xs);
        },
    );
}

#[test]
fn limbs_vec_mod_power_of_two_add_in_place_left_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_13,
        |&(ref xs, ref ys, pow)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            limbs_vec_mod_power_of_two_add_in_place_left(&mut xs, ys, pow);
            assert_eq!(
                Natural::from_owned_limbs_asc(xs),
                Natural::from_owned_limbs_asc(xs_old)
                    .mod_power_of_two_add(Natural::from_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_two_add_in_place_either_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_13,
        |&(ref xs, ref ys, pow)| {
            let mut xs = xs.to_vec();
            let mut ys = ys.to_vec();
            let xs_old = xs.clone();
            let ys_old = ys.clone();
            let right = limbs_mod_power_of_two_add_in_place_either(&mut xs, &mut ys, pow);
            let n = Natural::from_limbs_asc(&xs_old)
                .mod_power_of_two_add(Natural::from_limbs_asc(&ys_old), pow);
            if right {
                assert_eq!(xs, xs_old);
                assert_eq!(Natural::from_owned_limbs_asc(ys), n);
            } else {
                assert_eq!(Natural::from_owned_limbs_asc(xs), n);
                assert_eq!(ys, ys_old);
            }
        },
    );
}

#[test]
fn mod_power_of_two_add_properties() {
    test_properties(
        triples_of_natural_natural_and_u64_var_1,
        |&(ref x, ref y, pow)| {
            assert!(x.mod_power_of_two_is_reduced(pow));
            assert!(y.mod_power_of_two_is_reduced(pow));
            let sum_val_val = x.clone().mod_power_of_two_add(y.clone(), pow);
            let sum_val_ref = x.clone().mod_power_of_two_add(y, pow);
            let sum_ref_val = x.mod_power_of_two_add(y.clone(), pow);
            let sum = x.mod_power_of_two_add(y, pow);
            assert!(sum_val_val.is_valid());
            assert!(sum_val_ref.is_valid());
            assert!(sum_ref_val.is_valid());
            assert!(sum.is_valid());
            assert!(sum.mod_power_of_two_is_reduced(pow));
            assert_eq!(sum_val_val, sum);
            assert_eq!(sum_val_ref, sum);
            assert_eq!(sum_ref_val, sum);

            assert_eq!((x + y).mod_power_of_two(pow), sum);
            let mut sum_alt = x + y;
            sum_alt.clear_bit(pow);
            assert_eq!(sum_alt, sum);
            assert_eq!(x.mod_add(y, Natural::power_of_two(pow)), sum);

            let mut mut_x = x.clone();
            mut_x.mod_power_of_two_add_assign(y.clone(), pow);
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, sum);
            let mut mut_x = x.clone();
            mut_x.mod_power_of_two_add_assign(y, pow);
            assert_eq!(mut_x, sum);
            assert!(mut_x.is_valid());

            assert_eq!(y.mod_power_of_two_add(x, pow), sum);
            assert_eq!(
                x.mod_power_of_two_sub(y.mod_power_of_two_neg(pow), pow),
                sum
            );
            assert_eq!((&sum).mod_power_of_two_sub(x, pow), *y);
            assert_eq!(sum.mod_power_of_two_sub(y, pow), *x);
        },
    );

    test_properties(pairs_of_natural_and_u64_var_1, |&(ref x, pow)| {
        assert_eq!(x.mod_power_of_two_add(Natural::ZERO, pow), *x);
        assert_eq!(Natural::ZERO.mod_power_of_two_add(x, pow), *x);
        assert_eq!(
            x.mod_power_of_two_add(x, pow),
            x.mod_power_of_two_shl(1, pow)
        );
    });

    test_properties(
        quadruples_of_three_naturals_and_u64_var_1,
        |&(ref x, ref y, ref z, pow)| {
            assert_eq!(
                x.mod_power_of_two_add(y, pow).mod_power_of_two_add(z, pow),
                x.mod_power_of_two_add(y.mod_power_of_two_add(z, pow), pow)
            );
        },
    );

    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_1::<Limb>,
        |&(x, y, pow)| {
            assert_eq!(
                x.mod_power_of_two_add(y, pow),
                Natural::from(x).mod_power_of_two_add(Natural::from(y), pow)
            );
        },
    );
}
