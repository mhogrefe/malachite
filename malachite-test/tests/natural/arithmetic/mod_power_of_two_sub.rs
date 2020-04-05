use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoIsReduced, ModPowerOfTwoNeg, ModPowerOfTwoSub,
    ModPowerOfTwoSubAssign,
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

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_limb_sub_limbs_and_limbs_mod_power_of_two_limb_sub_limbs_in_place() {
    let test = |x: Limb, ys: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_two_limb_sub_limbs(x, ys, pow), out);

        let mut ys = ys.to_vec();
        limbs_mod_power_of_two_limb_sub_limbs_in_place(x, &mut ys, pow);
        assert_eq!(ys, out);
    };
    test(3, &[2], 4, &[1]);
    test(3, &[10], 4, &[9]);
    test(3, &[1, 2, 3], 70, &[2, 4294967294, 60]);
    test(
        3,
        &[1, 2, 3],
        200,
        &[
            2, 4294967294, 4294967292, 4294967295, 4294967295, 4294967295, 255,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_two_limb_sub_limbs_fail() {
    limbs_mod_power_of_two_limb_sub_limbs(3, &mut vec![10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_two_limb_sub_limbs_in_place_fail() {
    let mut ys = vec![10];
    limbs_mod_power_of_two_limb_sub_limbs_in_place(3, &mut ys, 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_sub() {
    let test = |xs_before, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_two_sub(xs_before, ys, pow), out);

        let mut xs = xs_before.to_vec();
        limbs_mod_power_of_two_sub_in_place_left(&mut xs, ys, pow);
        assert_eq!(xs, out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[], &[2], 3, &[6]);
    test(&[2], &[3], 2, &[3]);
    test(&[1, 2, 3], &[6, 7], 100, &[4294967291, 4294967290, 2]);
    test(&[6, 7], &[1, 2, 3], 100, &[5, 5, 4294967293, 15]);
    test(&[6, 7], &[1, 2], 100, &[5, 5]);
    test(
        &[1, 2],
        &[6, 7],
        100,
        &[4294967291, 4294967290, 4294967295, 15],
    );
    test(&[6, 7], &[2, 3, 0], 100, &[4, 4, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_sub_in_place_right() {
    let test = |xs, ys_before: &[Limb], pow, out: &[Limb]| {
        let mut ys = ys_before.to_vec();
        limbs_mod_power_of_two_sub_in_place_right(xs, &mut ys, pow);
        assert_eq!(ys, out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[], &[2], 3, &[6]);
    test(&[2], &[3], 2, &[3]);
    test(&[1, 2, 3], &[6, 7], 100, &[4294967291, 4294967290, 2]);
    test(&[6, 7], &[1, 2, 3], 100, &[5, 5, 4294967293, 15]);
    test(&[6, 7], &[1, 2], 100, &[5, 5]);
    test(
        &[1, 2],
        &[6, 7],
        100,
        &[4294967291, 4294967290, 4294967295, 15],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_sub_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                pow,
                right,
                xs_after: &[Limb],
                ys_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_mod_power_of_two_sub_in_place_either(&mut xs, &mut ys, pow),
            right
        );
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], 0, false, &[], &[]);
    test(&[], &[], 5, false, &[], &[]);
    test(&[2], &[], 3, false, &[2], &[]);
    test(&[], &[2], 3, true, &[], &[6]);
    test(&[2], &[3], 2, false, &[3], &[3]);
    test(
        &[1, 2, 3],
        &[6, 7],
        100,
        false,
        &[4294967291, 4294967290, 2],
        &[6, 7],
    );
    test(
        &[6, 7],
        &[1, 2, 3],
        100,
        true,
        &[6, 7],
        &[5, 5, 4294967293, 15],
    );
    test(&[6, 7], &[1, 2], 100, false, &[5, 5], &[1, 2]);
    test(
        &[1, 2],
        &[6, 7],
        100,
        false,
        &[4294967291, 4294967290, 4294967295, 15],
        &[6, 7],
    );
}

#[test]
fn test_mod_power_of_two_sub() {
    let test = |u, v, pow, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));
        assert!(Natural::from_str(v)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_sub_assign(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_two_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_sub_assign(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_sub(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_two_sub(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_sub(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_two_sub(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 5, "0");
    test("0", "27", 5, "5");
    test("10", "2", 4, "8");
    test("2", "10", 4, "8");
    test("0", "5", 7, "123");
    test("123", "0", 7, "123");
    test("123", "56", 9, "67");
    test("56", "123", 9, "445");
    test("3", "1267650600228229401496703205375", 100, "4");
    test(
        "10970645355953595821",
        "19870830162202579837",
        65,
        "27993303341170119216",
    );
    test(
        "14424295573283161220",
        "2247489031103704789",
        66,
        "12176806542179456431",
    );
    test(
        "2247489031103704789",
        "14424295573283161220",
        66,
        "61610169752658750033",
    );
    test(
        "340279770772528537691305857201098194975",
        "5708990430541473157891818604560539975629668416",
        165,
        "46762343404498631680132551366007801946215309901791",
    )
}

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
