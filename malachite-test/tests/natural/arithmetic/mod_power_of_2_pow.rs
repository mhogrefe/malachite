use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Neg, ModPowerOf2Pow,
    ModPowerOf2PowAssign, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Iverson, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::arithmetic::mod_power_of_2_pow::{
    limbs_mod_power_of_2_pow, limbs_pow_low,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::arithmetic::mod_power_of_2_pow::*;

use malachite_test::common::{
    test_properties, test_properties_custom_scale, test_properties_no_special,
};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_var_28, triples_of_unsigned_unsigned_and_small_u64_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_17,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_u64_var_1, pairs_of_natural_and_unsigned,
    quadruples_of_three_naturals_and_u64_var_2, quadruples_of_three_naturals_and_u64_var_3,
    triples_of_natural_natural_and_u64_var_2,
};

// TODO...
// pairs_of_unsigned_vec_var_28 -> unsigned_vec_pair_gen_var_3
// triples_of_unsigned_unsigned_and_small_u64_var_2
// triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_17
// pairs_of_natural_and_u64_var_1
// pairs_of_natural_and_unsigned
// quadruples_of_three_naturals_and_u64_var_2
// quadruples_of_three_naturals_and_u64_var_3
// triples_of_natural_natural_and_u64_var_2

fn verify_limbs_pow_low(xs: &[Limb], es: &[Limb], out: &[Limb]) {
    let exp = Natural::from_limbs_asc(es);
    let n = xs.len();
    let pow = u64::exact_from(n) << Limb::LOG_WIDTH;
    let x = Natural::from_limbs_asc(xs).mod_power_of_2(pow);
    let expected = x.mod_power_of_2_pow(exp, pow);
    assert!(expected.mod_power_of_2_is_reduced(pow));
    assert_eq!(Natural::from_limbs_asc(&out[..n]), expected);
}

#[test]
fn limbs_pow_low_properties() {
    test_properties_custom_scale(32, pairs_of_unsigned_vec_var_28, |&(ref xs, ref es)| {
        let xs_old = xs;
        let mut xs = xs.clone();
        let mut scratch = vec![0; xs.len()];
        limbs_pow_low(&mut xs, &es, &mut scratch);
        verify_limbs_pow_low(xs_old, es, &xs);
    });
}

fn verify_limbs_mod_power_of_2_pow(xs: &[Limb], es: &[Limb], pow: u64, out: &[Limb]) {
    let exp = Natural::from_limbs_asc(es);
    let x = Natural::from_limbs_asc(xs);
    assert!(x.mod_power_of_2_is_reduced(pow));
    let expected = (&x).mod_power_of_2_pow(&exp, pow);
    assert!(expected.mod_power_of_2_is_reduced(pow));
    assert_eq!(simple_binary_mod_power_of_2_pow(&x, &exp, pow), expected);
    assert_eq!(Natural::from_limbs_asc(out), expected);
}

#[test]
fn limbs_mod_power_of_2_pow_properties() {
    test_properties_custom_scale(
        32,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_17,
        |&(ref xs, ref es, pow)| {
            let xs_old = xs;
            let mut xs = xs.clone();
            limbs_mod_power_of_2_pow(&mut xs, &es, pow);
            verify_limbs_mod_power_of_2_pow(xs_old, es, pow, &xs);
        },
    );
}

#[test]
fn mod_power_of_2_pow_properties() {
    test_properties(
        triples_of_natural_natural_and_u64_var_2,
        |&(ref x, ref exp, pow)| {
            assert!(x.mod_power_of_2_is_reduced(pow));
            let power_val_val = x.clone().mod_power_of_2_pow(exp.clone(), pow);
            let power_val_ref = x.clone().mod_power_of_2_pow(exp, pow);
            let power_ref_val = x.mod_power_of_2_pow(exp.clone(), pow);
            let power = x.mod_power_of_2_pow(exp, pow);
            assert!(power_val_val.is_valid());
            assert!(power_val_ref.is_valid());
            assert!(power_ref_val.is_valid());
            assert!(power.is_valid());
            assert!(power.mod_power_of_2_is_reduced(pow));
            assert_eq!(power_val_val, power);
            assert_eq!(power_val_ref, power);
            assert_eq!(power_ref_val, power);

            let mut mut_x = x.clone();
            mut_x.mod_power_of_2_pow_assign(exp.clone(), pow);
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, power);
            let mut mut_x = x.clone();
            mut_x.mod_power_of_2_pow_assign(exp, pow);
            assert_eq!(mut_x, power);
            assert!(mut_x.is_valid());

            if exp.even() {
                assert_eq!(
                    x.mod_power_of_2_neg(pow).mod_power_of_2_pow(exp, pow),
                    power
                );
            } else {
                assert_eq!(
                    x.mod_power_of_2_neg(pow).mod_power_of_2_pow(exp, pow),
                    power.mod_power_of_2_neg(pow)
                );
            }
        },
    );

    test_properties(pairs_of_natural_and_unsigned, |&(ref exp, pow)| {
        assert_eq!(
            Natural::ZERO.mod_power_of_2_pow(exp, pow),
            Natural::iverson(*exp == 0 && pow != 0),
        );
        if pow != 0 {
            assert_eq!(Natural::ONE.mod_power_of_2_pow(exp, pow), 1);
        }
    });

    test_properties(pairs_of_natural_and_u64_var_1, |&(ref x, pow)| {
        assert_eq!(
            x.mod_power_of_2_pow(Natural::ZERO, pow),
            Natural::iverson(pow != 0)
        );
        assert_eq!(x.mod_power_of_2_pow(Natural::ONE, pow), *x);
        assert_eq!(
            x.mod_power_of_2_pow(Natural::TWO, pow),
            x.mod_power_of_2_mul(x, pow)
        );
    });

    test_properties(
        quadruples_of_three_naturals_and_u64_var_2,
        |&(ref x, ref y, ref exp, pow)| {
            assert_eq!(
                x.mod_power_of_2_mul(y, pow).mod_power_of_2_pow(exp, pow),
                x.mod_power_of_2_pow(exp, pow)
                    .mod_power_of_2_mul(y.mod_power_of_2_pow(exp, pow), pow)
            );
        },
    );

    test_properties(
        quadruples_of_three_naturals_and_u64_var_3,
        |&(ref x, ref e, ref f, pow)| {
            assert_eq!(
                x.mod_power_of_2_pow(e + f, pow),
                x.mod_power_of_2_pow(e, pow)
                    .mod_power_of_2_mul(x.mod_power_of_2_pow(f, pow), pow)
            );
            assert_eq!(
                x.mod_power_of_2_pow(e * f, pow),
                x.mod_power_of_2_pow(e, pow).mod_power_of_2_pow(f, pow)
            );
        },
    );

    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_2::<Limb>,
        |&(x, exp, pow)| {
            assert_eq!(
                x.mod_power_of_2_pow(exp, pow),
                Natural::from(x).mod_power_of_2_pow(Natural::from(exp), pow)
            );
        },
    );
}
