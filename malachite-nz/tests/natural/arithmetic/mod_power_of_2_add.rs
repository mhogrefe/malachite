use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2IsReduced,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mod_power_of_2_add::{
    limbs_mod_power_of_2_add, limbs_mod_power_of_2_add_greater,
    limbs_mod_power_of_2_add_in_place_either, limbs_mod_power_of_2_add_limb,
    limbs_slice_mod_power_of_2_add_greater_in_place_left,
    limbs_slice_mod_power_of_2_add_limb_in_place, limbs_vec_mod_power_of_2_add_in_place_left,
    limbs_vec_mod_power_of_2_add_limb_in_place,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_add_limb() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_add_limb(xs, y, pow), out);
    };
    test(&[], 0, 0, &[]);
    test(&[], 0, 5, &[]);
    test(&[], 5, 3, &[5]);
    test(&[123, 456], 789, 41, &[912, 456]);
    test(&[u32::MAX], 2, 33, &[1, 1]);
    test(&[u32::MAX], 2, 32, &[1]);
    test(&[u32::MAX, 3], 2, 34, &[1, 0]);
    test(&[u32::MAX, 3], 2, 35, &[1, 4]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mod_power_of_2_add_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb], carry: bool| {
        let mut xs = xs.to_vec();
        assert_eq!(
            limbs_slice_mod_power_of_2_add_limb_in_place(&mut xs, y, pow),
            carry
        );
        assert_eq!(xs, out);
    };
    test(&[], 0, 0, &[], false);
    test(&[], 0, 5, &[], false);
    test(&[], 5, 3, &[], true);
    test(&[123, 456], 789, 41, &[912, 456], false);
    test(&[u32::MAX], 2, 33, &[1], true);
    test(&[u32::MAX], 2, 32, &[1], false);
    test(&[u32::MAX, 3], 2, 34, &[1, 0], false);
    test(&[u32::MAX, 3], 2, 35, &[1, 4], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_mod_power_of_2_add_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_vec_mod_power_of_2_add_limb_in_place(&mut xs, y, pow);
        assert_eq!(xs, out);
    };
    test(&[123, 456], 789, 41, &[912, 456]);
    test(&[u32::MAX], 2, 33, &[1, 1]);
    test(&[u32::MAX], 2, 32, &[1]);
    test(&[u32::MAX, 3], 2, 34, &[1, 0]);
    test(&[u32::MAX, 3], 2, 35, &[1, 4]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_mod_power_of_2_add_limb_in_place_fail() {
    limbs_vec_mod_power_of_2_add_limb_in_place(&mut vec![], 10, 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_add_greater() {
    let test = |xs, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_add_greater(xs, ys, pow), out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[2], &[3], 2, &[1]);
    test(&[1, 2, 3], &[6, 7], 100, &[7, 9, 3]);
    test(&[100, 101, u32::MAX], &[102, 101, 2], 97, &[202, 202, 1, 1]);
    test(&[100, 101, u32::MAX], &[102, 101, 2], 96, &[202, 202, 1]);
    test(&[u32::MAX], &[2], 33, &[1, 1]);
    test(&[u32::MAX], &[2], 32, &[1]);
    test(&[u32::MAX, 3], &[2], 34, &[1, 0]);
    test(&[u32::MAX, 3], &[2], 35, &[1, 4]);
    test(&[u32::MAX, u32::MAX], &[2], 65, &[1, 0, 1]);
    test(&[u32::MAX, u32::MAX], &[2], 64, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn test_limbs_mod_power_of_2_add_greater_fail() {
    limbs_mod_power_of_2_add_greater(&[6, 7], &[1, 2, 3], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_add_and_limbs_vec_mod_power_of_2_add_in_place_left() {
    let test = |xs_before, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_add(xs_before, ys, pow), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_mod_power_of_2_add_in_place_left(&mut xs, ys, pow);
        assert_eq!(xs, out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[], &[2], 3, &[2]);
    test(&[2], &[3], 2, &[1]);
    test(&[1, 2, 3], &[6, 7], 100, &[7, 9, 3]);
    test(&[6, 7], &[1, 2, 3], 100, &[7, 9, 3]);
    test(&[100, 101, u32::MAX], &[102, 101, 2], 97, &[202, 202, 1, 1]);
    test(&[100, 101, u32::MAX], &[102, 101, 2], 96, &[202, 202, 1]);
    test(&[u32::MAX], &[2], 33, &[1, 1]);
    test(&[u32::MAX], &[2], 32, &[1]);
    test(&[u32::MAX, 3], &[2], 34, &[1, 0]);
    test(&[u32::MAX, 3], &[2], 35, &[1, 4]);
    test(&[u32::MAX, u32::MAX], &[2], 65, &[1, 0, 1]);
    test(&[u32::MAX, u32::MAX], &[2], 64, &[1, 0]);
    test(&[2], &[u32::MAX, u32::MAX], 65, &[1, 0, 1]);
    test(&[2], &[u32::MAX, u32::MAX], 64, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mod_power_of_2_add_greater_in_place_left() {
    let test = |xs_before: &[Limb], ys, pow, xs_after: &[Limb], carry| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut xs, ys, pow),
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
        &[100, 101, u32::MAX],
        &[102, 101, 2],
        97,
        &[202, 202, 1],
        true,
    );
    test(
        &[100, 101, u32::MAX],
        &[102, 101, 2],
        96,
        &[202, 202, 1],
        false,
    );
    test(&[u32::MAX], &[2], 33, &[1], true);
    test(&[u32::MAX], &[2], 32, &[1], false);
    test(&[u32::MAX, 3], &[2], 34, &[1, 0], false);
    test(&[u32::MAX, 3], &[2], 35, &[1, 4], false);
    test(&[u32::MAX, u32::MAX], &[2], 65, &[1, 0], true);
    test(&[u32::MAX, u32::MAX], &[2], 64, &[1, 0], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_mod_power_of_2_add_greater_in_place_left_fail() {
    let mut xs = vec![6, 7];
    limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut xs, &[1, 2, 3], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_add_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                pow,
                right,
                xs_after: &[Limb],
                ys_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_mod_power_of_2_add_in_place_either(&mut xs, &mut ys, pow),
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
        &[100, 101, u32::MAX],
        &[102, 101, 2],
        97,
        false,
        &[202, 202, 1, 1],
        &[102, 101, 2],
    );
    test(
        &[100, 101, u32::MAX],
        &[102, 101, 2],
        96,
        false,
        &[202, 202, 1],
        &[102, 101, 2],
    );
    test(&[u32::MAX], &[2], 33, false, &[1, 1], &[2]);
    test(&[u32::MAX], &[2], 32, false, &[1], &[2]);
    test(&[u32::MAX, 3], &[2], 34, false, &[1, 0], &[2]);
    test(&[u32::MAX, 3], &[2], 35, false, &[1, 4], &[2]);
    test(&[u32::MAX, u32::MAX], &[2], 65, false, &[1, 0, 1], &[2]);
    test(&[u32::MAX, u32::MAX], &[2], 64, false, &[1, 0], &[2]);
    test(&[2], &[u32::MAX, u32::MAX], 65, true, &[2], &[1, 0, 1]);
    test(&[2], &[u32::MAX, u32::MAX], 64, true, &[2], &[1, 0]);
}

#[test]
fn test_mod_power_of_2_add() {
    let test = |u, v, pow, out| {
        assert!(Natural::from_str(u).unwrap().mod_power_of_2_is_reduced(pow));
        assert!(Natural::from_str(v).unwrap().mod_power_of_2_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_2_add_assign(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_2_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_2_add_assign(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_2_add(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n =
            (&Natural::from_str(u).unwrap()).mod_power_of_2_add(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_2_add(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_2_add(&Natural::from_str(v).unwrap(), pow);
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
