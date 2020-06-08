use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoIsReduced, ModPowerOfTwoNeg, ModPowerOfTwoSub,
    ModPowerOfTwoSubAssign, ModSub, PowerOfTwo,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::mod_power_of_two_sub::{
    limbs_mod_power_of_two_limb_sub_limbs, limbs_mod_power_of_two_limb_sub_limbs_in_place,
    limbs_mod_power_of_two_sub, limbs_mod_power_of_two_sub_in_place_either,
    limbs_mod_power_of_two_sub_in_place_left, limbs_mod_power_of_two_sub_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    triples_of_limb_limb_vec_and_u64_var_1, triples_of_limb_vec_limb_vec_and_u64_var_13,
    triples_of_limb_vec_limb_vec_and_u64_var_15, triples_of_unsigned_unsigned_and_small_u64_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_u64_var_1, triples_of_natural_natural_and_u64_var_1,
};

#[test]
fn limbs_mod_power_of_two_limb_sub_limbs_properties() {
    test_properties(
        triples_of_limb_limb_vec_and_u64_var_1,
        |&(x, ref ys, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_limb_sub_limbs(x, ys, pow)),
                Natural::from(x).mod_power_of_two_sub(Natural::from_limbs_asc(ys), pow),
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_two_limb_sub_limbs_in_place_properties() {
    test_properties(
        triples_of_limb_limb_vec_and_u64_var_1,
        |&(x, ref ys, pow)| {
            let mut ys = ys.to_vec();
            let old_ys = ys.clone();
            limbs_mod_power_of_two_limb_sub_limbs_in_place(x, &mut ys, pow);
            let n = Natural::from(x).mod_power_of_two_sub(Natural::from_limbs_asc(&old_ys), pow);
            let mut expected_limbs = n.into_limbs_asc();
            expected_limbs.resize(ys.len(), 0);
            assert_eq!(ys, expected_limbs);
        },
    );
}

#[test]
fn limbs_mod_power_of_two_sub_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_13,
        |&(ref xs, ref ys, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_sub(xs, ys, pow)),
                Natural::from_limbs_asc(xs).mod_power_of_two_sub(Natural::from_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_two_sub_in_place_left_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_13,
        |&(ref xs, ref ys, pow)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            limbs_mod_power_of_two_sub_in_place_left(&mut xs, ys, pow);
            assert_eq!(
                Natural::from_owned_limbs_asc(xs),
                Natural::from_owned_limbs_asc(xs_old)
                    .mod_power_of_two_sub(Natural::from_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_two_sub_in_place_right_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_15,
        |&(ref xs, ref ys, pow)| {
            let mut ys = ys.to_vec();
            let ys_old = ys.clone();
            limbs_mod_power_of_two_sub_in_place_right(xs, &mut ys, pow);
            assert_eq!(
                Natural::from_owned_limbs_asc(ys),
                Natural::from_limbs_asc(xs)
                    .mod_power_of_two_sub(Natural::from_owned_limbs_asc(ys_old), pow)
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_two_sub_in_place_either_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_15,
        |&(ref xs, ref ys, pow)| {
            let mut xs = xs.to_vec();
            let mut ys = ys.to_vec();
            let xs_old = xs.clone();
            let ys_old = ys.clone();
            let right = limbs_mod_power_of_two_sub_in_place_either(&mut xs, &mut ys, pow);
            let n = Natural::from_limbs_asc(&xs_old)
                .mod_power_of_two_sub(Natural::from_limbs_asc(&ys_old), pow);
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
fn mod_power_of_two_sub_properties() {
    test_properties(
        triples_of_natural_natural_and_u64_var_1,
        |&(ref x, ref y, pow)| {
            assert!(x.mod_power_of_two_is_reduced(pow));
            assert!(y.mod_power_of_two_is_reduced(pow));
            let diff_val_val = x.clone().mod_power_of_two_sub(y.clone(), pow);
            let diff_val_ref = x.clone().mod_power_of_two_sub(y, pow);
            let diff_ref_val = x.mod_power_of_two_sub(y.clone(), pow);
            let diff = x.mod_power_of_two_sub(y, pow);
            assert!(diff_val_val.is_valid());
            assert!(diff_val_ref.is_valid());
            assert!(diff_ref_val.is_valid());
            assert!(diff.is_valid());
            assert!(diff.mod_power_of_two_is_reduced(pow));
            assert_eq!(diff_val_val, diff);
            assert_eq!(diff_val_ref, diff);
            assert_eq!(diff_ref_val, diff);

            assert_eq!(
                (Integer::from(x) - Integer::from(y)).mod_power_of_two(pow),
                diff
            );
            let diff_alt = if x >= y {
                x - y
            } else {
                let mut x = x.clone();
                x.set_bit(pow);
                x - y
            };
            assert_eq!(diff_alt, diff);
            assert_eq!(x.mod_sub(y, Natural::power_of_two(pow)), diff);

            let mut mut_x = x.clone();
            mut_x.mod_power_of_two_sub_assign(y.clone(), pow);
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, diff);
            let mut mut_x = x.clone();
            mut_x.mod_power_of_two_sub_assign(y, pow);
            assert_eq!(mut_x, diff);
            assert!(mut_x.is_valid());

            assert_eq!(
                y.mod_power_of_two_sub(x, pow),
                (&diff).mod_power_of_two_neg(pow),
            );
            assert_eq!(
                x.mod_power_of_two_add(y.mod_power_of_two_neg(pow), pow),
                diff
            );
            assert_eq!((&diff).mod_power_of_two_add(y, pow), *x);
            assert_eq!(
                diff.mod_power_of_two_sub(x, pow),
                y.mod_power_of_two_neg(pow)
            );
        },
    );

    test_properties(pairs_of_natural_and_u64_var_1, |&(ref x, pow)| {
        assert_eq!(x.mod_power_of_two_sub(Natural::ZERO, pow), *x);
        assert_eq!(
            Natural::ZERO.mod_power_of_two_sub(x, pow),
            x.mod_power_of_two_neg(pow)
        );
        assert_eq!(x.mod_power_of_two_sub(x, pow), 0);
    });

    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_1::<Limb>,
        |&(x, y, pow)| {
            assert_eq!(
                x.mod_power_of_two_sub(y, pow),
                Natural::from(x).mod_power_of_two_sub(Natural::from(y), pow)
            );
        },
    );
}
