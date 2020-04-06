use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModAdd, ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoAddAssign, ModPowerOfTwoIsReduced,
    ModPowerOfTwoNeg, ModPowerOfTwoSub, PowerOfTwo,
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

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_add_limb() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_two_add_limb(xs, y, pow), out);
    };
    test(&[], 0, 0, &[]);
    test(&[], 0, 5, &[]);
    test(&[], 5, 3, &[5]);
    test(&[123, 456], 789, 41, &[912, 456]);
    test(&[0xffff_ffff], 2, 33, &[1, 1]);
    test(&[0xffff_ffff], 2, 32, &[1]);
    test(&[0xffff_ffff, 3], 2, 34, &[1, 0]);
    test(&[0xffff_ffff, 3], 2, 35, &[1, 4]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mod_power_of_two_add_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb], carry: bool| {
        let mut xs = xs.to_vec();
        assert_eq!(
            limbs_slice_mod_power_of_two_add_limb_in_place(&mut xs, y, pow),
            carry
        );
        assert_eq!(xs, out);
    };
    test(&[], 0, 0, &[], false);
    test(&[], 0, 5, &[], false);
    test(&[], 5, 3, &[], true);
    test(&[123, 456], 789, 41, &[912, 456], false);
    test(&[0xffff_ffff], 2, 33, &[1], true);
    test(&[0xffff_ffff], 2, 32, &[1], false);
    test(&[0xffff_ffff, 3], 2, 34, &[1, 0], false);
    test(&[0xffff_ffff, 3], 2, 35, &[1, 4], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_mod_power_of_two_add_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_vec_mod_power_of_two_add_limb_in_place(&mut xs, y, pow);
        assert_eq!(xs, out);
    };
    test(&[123, 456], 789, 41, &[912, 456]);
    test(&[0xffff_ffff], 2, 33, &[1, 1]);
    test(&[0xffff_ffff], 2, 32, &[1]);
    test(&[0xffff_ffff, 3], 2, 34, &[1, 0]);
    test(&[0xffff_ffff, 3], 2, 35, &[1, 4]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_mod_power_of_two_add_limb_in_place_fail() {
    limbs_vec_mod_power_of_two_add_limb_in_place(&mut vec![], 10, 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_add_greater() {
    let test = |xs, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_two_add_greater(xs, ys, pow), out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[2], &[3], 2, &[1]);
    test(&[1, 2, 3], &[6, 7], 100, &[7, 9, 3]);
    test(
        &[100, 101, 0xffff_ffff],
        &[102, 101, 2],
        97,
        &[202, 202, 1, 1],
    );
    test(&[100, 101, 0xffff_ffff], &[102, 101, 2], 96, &[202, 202, 1]);
    test(&[0xffff_ffff], &[2], 33, &[1, 1]);
    test(&[0xffff_ffff], &[2], 32, &[1]);
    test(&[0xffff_ffff, 3], &[2], 34, &[1, 0]);
    test(&[0xffff_ffff, 3], &[2], 35, &[1, 4]);
    test(&[0xffff_ffff, 0xffff_ffff], &[2], 65, &[1, 0, 1]);
    test(&[0xffff_ffff, 0xffff_ffff], &[2], 64, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn test_limbs_mod_power_of_two_add_greater_fail() {
    limbs_mod_power_of_two_add_greater(&[6, 7], &[1, 2, 3], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_add_and_limbs_vec_mod_power_of_two_add_in_place_left() {
    let test = |xs_before, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_two_add(xs_before, ys, pow), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_mod_power_of_two_add_in_place_left(&mut xs, ys, pow);
        assert_eq!(xs, out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[], &[2], 3, &[2]);
    test(&[2], &[3], 2, &[1]);
    test(&[1, 2, 3], &[6, 7], 100, &[7, 9, 3]);
    test(&[6, 7], &[1, 2, 3], 100, &[7, 9, 3]);
    test(
        &[100, 101, 0xffff_ffff],
        &[102, 101, 2],
        97,
        &[202, 202, 1, 1],
    );
    test(&[100, 101, 0xffff_ffff], &[102, 101, 2], 96, &[202, 202, 1]);
    test(&[0xffff_ffff], &[2], 33, &[1, 1]);
    test(&[0xffff_ffff], &[2], 32, &[1]);
    test(&[0xffff_ffff, 3], &[2], 34, &[1, 0]);
    test(&[0xffff_ffff, 3], &[2], 35, &[1, 4]);
    test(&[0xffff_ffff, 0xffff_ffff], &[2], 65, &[1, 0, 1]);
    test(&[0xffff_ffff, 0xffff_ffff], &[2], 64, &[1, 0]);
    test(&[2], &[0xffff_ffff, 0xffff_ffff], 65, &[1, 0, 1]);
    test(&[2], &[0xffff_ffff, 0xffff_ffff], 64, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mod_power_of_two_add_greater_in_place_left() {
    let test = |xs_before: &[Limb], ys, pow, xs_after: &[Limb], carry| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_slice_mod_power_of_two_add_greater_in_place_left(&mut xs, ys, pow),
            carry
        );
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], 0, &[], false);
    test(&[], &[], 5, &[], false);
    test(&[2], &[], 3, &[2], false);
    test(&[2], &[3], 2, &[1], false);
    test(&[1, 2, 3], &[6, 7], 100, &[7, 9, 3], false);
    test(
        &[100, 101, 0xffff_ffff],
        &[102, 101, 2],
        97,
        &[202, 202, 1],
        true,
    );
    test(
        &[100, 101, 0xffff_ffff],
        &[102, 101, 2],
        96,
        &[202, 202, 1],
        false,
    );
    test(&[0xffff_ffff], &[2], 33, &[1], true);
    test(&[0xffff_ffff], &[2], 32, &[1], false);
    test(&[0xffff_ffff, 3], &[2], 34, &[1, 0], false);
    test(&[0xffff_ffff, 3], &[2], 35, &[1, 4], false);
    test(&[0xffff_ffff, 0xffff_ffff], &[2], 65, &[1, 0], true);
    test(&[0xffff_ffff, 0xffff_ffff], &[2], 64, &[1, 0], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_mod_power_of_two_add_greater_in_place_left_fail() {
    let mut xs = vec![6, 7];
    limbs_slice_mod_power_of_two_add_greater_in_place_left(&mut xs, &[1, 2, 3], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_add_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                pow,
                right,
                xs_after: &[Limb],
                ys_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_mod_power_of_two_add_in_place_either(&mut xs, &mut ys, pow),
            right
        );
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], 0, false, &[], &[]);
    test(&[], &[], 5, false, &[], &[]);
    test(&[2], &[], 3, false, &[2], &[]);
    test(&[], &[2], 3, true, &[], &[2]);
    test(&[2], &[3], 2, false, &[1], &[3]);
    test(&[1, 2, 3], &[6, 7], 100, false, &[7, 9, 3], &[6, 7]);
    test(&[6, 7], &[1, 2, 3], 100, true, &[6, 7], &[7, 9, 3]);
    test(
        &[100, 101, 0xffff_ffff],
        &[102, 101, 2],
        97,
        false,
        &[202, 202, 1, 1],
        &[102, 101, 2],
    );
    test(
        &[100, 101, 0xffff_ffff],
        &[102, 101, 2],
        96,
        false,
        &[202, 202, 1],
        &[102, 101, 2],
    );
    test(&[0xffff_ffff], &[2], 33, false, &[1, 1], &[2]);
    test(&[0xffff_ffff], &[2], 32, false, &[1], &[2]);
    test(&[0xffff_ffff, 3], &[2], 34, false, &[1, 0], &[2]);
    test(&[0xffff_ffff, 3], &[2], 35, false, &[1, 4], &[2]);
    test(
        &[0xffff_ffff, 0xffff_ffff],
        &[2],
        65,
        false,
        &[1, 0, 1],
        &[2],
    );
    test(&[0xffff_ffff, 0xffff_ffff], &[2], 64, false, &[1, 0], &[2]);
    test(
        &[2],
        &[0xffff_ffff, 0xffff_ffff],
        65,
        true,
        &[2],
        &[1, 0, 1],
    );
    test(&[2], &[0xffff_ffff, 0xffff_ffff], 64, true, &[2], &[1, 0]);
}

#[test]
fn test_mod_power_of_two_add() {
    let test = |u, v, pow, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));
        assert!(Natural::from_str(v)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_add_assign(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_two_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_add_assign(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_add(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_two_add(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_add(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_two_add(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 5, "0");
    test("0", "2", 5, "2");
    test("10", "14", 4, "8");
    test("0", "123", 7, "123");
    test("123", "0", 7, "123");
    test("123", "456", 9, "67");
    test("1267650600228229401496703205375", "3", 100, "2");
    test("3", "1267650600228229401496703205375", 100, "2");
}

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
        //TODO assert_eq!(x + x, x << 1);
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
