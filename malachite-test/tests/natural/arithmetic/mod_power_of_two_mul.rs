use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModMul, ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoIsReduced, ModPowerOfTwoMul,
    ModPowerOfTwoMulAssign, ModPowerOfTwoNeg, PowerOfTwo,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::mod_power_of_two_mul::{
    limbs_mod_power_of_two_mul, limbs_mod_power_of_two_mul_ref_ref,
    limbs_mod_power_of_two_mul_val_ref,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    triples_of_limb_vec_limb_vec_and_u64_var_16, triples_of_unsigned_unsigned_and_small_u64_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_u64_var_1, quadruples_of_three_naturals_and_u64_var_1,
    triples_of_natural_natural_and_u64_var_1,
};

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

#[test]
fn limbs_mod_power_of_two_mul_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_16,
        |&(ref xs, ref ys, pow)| {
            let product =
                Natural::from_limbs_asc(xs).mod_power_of_two_mul(Natural::from_limbs_asc(ys), pow);
            assert_eq!(
                (Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys)).mod_power_of_two(pow),
                product
            );
            let mut mut_xs = xs.clone();
            let mut mut_ys = ys.clone();
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_mul(
                    &mut mut_xs,
                    &mut mut_ys,
                    pow
                )),
                product,
            );
            let mut mut_xs = xs.clone();
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_mul_val_ref(
                    &mut mut_xs,
                    ys,
                    pow
                )),
                product,
            );
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_mul_ref_ref(xs, ys, pow)),
                product,
            );
        },
    );
}

#[test]
fn mod_power_of_two_mul_properties() {
    test_properties(
        triples_of_natural_natural_and_u64_var_1,
        |&(ref x, ref y, pow)| {
            assert!(x.mod_power_of_two_is_reduced(pow));
            assert!(y.mod_power_of_two_is_reduced(pow));
            let product_val_val = x.clone().mod_power_of_two_mul(y.clone(), pow);
            let product_val_ref = x.clone().mod_power_of_two_mul(y, pow);
            let product_ref_val = x.mod_power_of_two_mul(y.clone(), pow);
            let product = x.mod_power_of_two_mul(y, pow);
            assert!(product_val_val.is_valid());
            assert!(product_val_ref.is_valid());
            assert!(product_ref_val.is_valid());
            assert!(product.is_valid());
            assert!(product.mod_power_of_two_is_reduced(pow));
            assert_eq!(product_val_val, product);
            assert_eq!(product_val_ref, product);
            assert_eq!(product_ref_val, product);

            assert_eq!((x * y).mod_power_of_two(pow), product);
            assert_eq!(x.mod_mul(y, Natural::power_of_two(pow)), product);

            let mut mut_x = x.clone();
            mut_x.mod_power_of_two_mul_assign(y.clone(), pow);
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, product);
            let mut mut_x = x.clone();
            mut_x.mod_power_of_two_mul_assign(y, pow);
            assert_eq!(mut_x, product);
            assert!(mut_x.is_valid());

            assert_eq!(y.mod_power_of_two_mul(x, pow), product);
            assert_eq!(
                x.mod_power_of_two_mul(y.mod_power_of_two_neg(pow), pow),
                (&product).mod_power_of_two_neg(pow)
            );
            assert_eq!(
                x.mod_power_of_two_neg(pow).mod_power_of_two_mul(y, pow),
                product.mod_power_of_two_neg(pow)
            );
        },
    );

    test_properties(pairs_of_natural_and_u64_var_1, |&(ref x, pow)| {
        assert_eq!(x.mod_power_of_two_mul(Natural::ZERO, pow), 0);
        assert_eq!(Natural::ZERO.mod_power_of_two_mul(x, pow), 0);
        assert_eq!(x.mod_power_of_two_mul(Natural::ONE, pow), *x);
        assert_eq!(Natural::ONE.mod_power_of_two_mul(x, pow), *x);
        //TODO assert_eq!(x * x, x.square());
    });

    test_properties(
        quadruples_of_three_naturals_and_u64_var_1,
        |&(ref x, ref y, ref z, pow)| {
            assert_eq!(
                x.mod_power_of_two_mul(y, pow).mod_power_of_two_mul(z, pow),
                x.mod_power_of_two_mul(y.mod_power_of_two_mul(z, pow), pow)
            );
            assert_eq!(
                x.mod_power_of_two_mul(y.mod_power_of_two_add(z, pow), pow),
                x.mod_power_of_two_mul(y, pow)
                    .mod_power_of_two_add(x.mod_power_of_two_mul(z, pow), pow)
            );
            assert_eq!(
                x.mod_power_of_two_add(y, pow).mod_power_of_two_mul(z, pow),
                x.mod_power_of_two_mul(z, pow)
                    .mod_power_of_two_add(y.mod_power_of_two_mul(z, pow), pow)
            );
        },
    );

    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_1::<Limb>,
        |&(x, y, pow)| {
            assert_eq!(
                x.mod_power_of_two_mul(y, pow),
                Natural::from(x).mod_power_of_two_mul(Natural::from(y), pow)
            );
        },
    );
}
