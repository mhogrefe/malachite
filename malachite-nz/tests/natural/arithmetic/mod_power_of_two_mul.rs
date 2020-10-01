use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoIsReduced, ModPowerOfTwoMul, ModPowerOfTwoMulAssign,
};

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mod_power_of_two_mul::{
    limbs_mod_power_of_two_mul, limbs_mod_power_of_two_mul_ref_ref,
    limbs_mod_power_of_two_mul_val_ref,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_mul() {
    let test = |xs, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_two_mul_ref_ref(xs, ys, pow), out);

        let mut mut_xs = xs.to_vec();
        assert_eq!(
            limbs_mod_power_of_two_mul_val_ref(&mut mut_xs, ys, pow),
            out
        );

        let mut mut_xs = xs.to_vec();
        let mut mut_ys = ys.to_vec();
        assert_eq!(
            limbs_mod_power_of_two_mul(&mut mut_xs, &mut mut_ys, pow),
            out
        );

        let product = Natural::from_limbs_asc(out);
        assert_eq!(
            Natural::from_limbs_asc(xs).mod_power_of_two_mul(Natural::from_limbs_asc(ys), pow),
            product
        );
        assert_eq!(
            (Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys)).mod_power_of_two(pow),
            product
        );
    };
    // max_len <= xs_len + ys_len + 1
    // xs_len >= limit && ys_len >= limit
    // xs_len == max_len
    // ys_len == max_len
    test(&[1], &[1], 1, &[1]);
    test(&[1], &[1], 5, &[1]);
    // xs_len < max_len
    // ys_len < max_len
    test(&[1], &[1], 33, &[1, 0]);
    test(&[2], &[1], 3, &[2]);
    test(&[1], &[2], 3, &[2]);
    test(&[2], &[3], 2, &[2]);
    // xs_len < limit || ys_len < limit
    test(&[1, 2, 3], &[6, 7], 100, &[6, 19, 32, 5]);
    test(&[6, 7], &[1, 2, 3], 100, &[6, 19, 32, 5]);
    // max_len > xs_len + ys_len + 1
    test(&[3255925883], &[3653042335], 131, &[2997571685, 2769295845]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_two_mul_fail_1() {
    limbs_mod_power_of_two_mul(&mut vec![1], &mut vec![], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_two_mul_fail_2() {
    limbs_mod_power_of_two_mul(&mut vec![], &mut vec![1], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_two_mul_val_ref_fail_1() {
    limbs_mod_power_of_two_mul_val_ref(&mut vec![1], &[], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_two_mul_val_ref_fail_2() {
    limbs_mod_power_of_two_mul_val_ref(&mut vec![], &[1], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_two_mul_ref_ref_fail_1() {
    limbs_mod_power_of_two_mul_ref_ref(&[1], &[], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_two_mul_ref_ref_fail_2() {
    limbs_mod_power_of_two_mul_ref_ref(&[], &[1], 2);
}

#[test]
fn test_mod_power_of_two_mul() {
    let test = |u, v, pow, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));
        assert!(Natural::from_str(v)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_mul_assign(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_two_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_mul_assign(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_mul(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_two_mul(Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_mul(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_two_mul(&Natural::from_str(v).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("1", "1", 5, "1");
    test("1", "1", 33, "1");
    test("1", "2", 5, "2");
    test("3", "2", 5, "6");
    test("10", "14", 4, "12");
    test("123", "456", 9, "280");
    test("123456789", "987654321", 60, "121932631112635269");
}
