use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoIsReduced, ModPowerOfTwoSub, ModPowerOfTwoSubAssign,
};

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mod_power_of_two_sub::{
    limbs_mod_power_of_two_limb_sub_limbs, limbs_mod_power_of_two_limb_sub_limbs_in_place,
    limbs_mod_power_of_two_sub, limbs_mod_power_of_two_sub_in_place_either,
    limbs_mod_power_of_two_sub_in_place_left, limbs_mod_power_of_two_sub_in_place_right,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

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
