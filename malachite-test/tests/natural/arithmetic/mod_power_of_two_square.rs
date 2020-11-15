use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoIsReduced, ModPowerOfTwoMul, ModPowerOfTwoNeg, ModPowerOfTwoSquare,
    ModPowerOfTwoSquareAssign, ModSquare, PowerOfTwo, Square,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::arithmetic::mod_power_of_two_square::{
    _limbs_square_low_basecase, _limbs_square_low_divide_and_conquer,
    _limbs_square_low_scratch_len, limbs_mod_power_of_two_square,
    limbs_mod_power_of_two_square_ref, limbs_square_low,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::arithmetic::mod_power_of_two_square::*;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_2, pairs_of_unsigned_vec_and_small_unsigned_var_4,
    pairs_of_unsigned_vec_var_26, pairs_of_unsigned_vec_var_27, pairs_of_unsigned_vec_var_4,
    small_unsigneds,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_u64_var_1, triples_of_natural_natural_and_u64_var_1,
};

fn verify_limbs_square_low(out_before: &[Limb], xs: &[Limb], out_after: &[Limb]) {
    let len = xs.len();
    let x = Natural::from_limbs_asc(xs);
    let pow = u64::exact_from(len) << Limb::LOG_WIDTH;
    let expected_square = (&x).square().mod_power_of_two(pow);
    assert_eq!(x.mod_power_of_two_square(pow), expected_square);
    let square = Natural::from_limbs_asc(&out_after[..len]);
    assert_eq!(square, expected_square);
    assert_eq!(&out_before[len..], &out_after[len..]);
}

#[test]
fn limbs_square_low_basecase_properties() {
    test_properties(pairs_of_unsigned_vec_var_26, |&(ref out, ref xs)| {
        let out_old = out;
        let mut out = out_old.clone();
        _limbs_square_low_basecase(&mut out, xs);
        verify_limbs_square_low(out_old, xs, &out);
        let expected_out = out;

        let mut out = out_old.clone();
        _limbs_square_low_basecase_unrestricted(&mut out, xs);
        assert_eq!(out, expected_out);
    });
}

#[test]
fn limbs_square_low_divide_and_conquer_properties() {
    test_properties(pairs_of_unsigned_vec_var_27, |&(ref out, ref xs)| {
        let out_old = out;
        let mut out = out_old.clone();
        let mut scratch = vec![0; _limbs_square_low_scratch_len(xs.len())];
        _limbs_square_low_divide_and_conquer(&mut out, xs, &mut scratch);
        verify_limbs_square_low(out_old, xs, &out);
    });
}

#[test]
fn limbs_square_low_properties() {
    test_properties(pairs_of_unsigned_vec_var_4, |&(ref out, ref xs)| {
        let out_old = out;
        let mut out = out_old.clone();
        limbs_square_low(&mut out, xs);
        verify_limbs_square_low(out_old, xs, &out);
    });
}

#[test]
fn limbs_mod_power_of_two_square_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_4,
        |&(ref xs, pow)| {
            let xs_old = xs;
            let mut xs = xs_old.clone();
            let out = limbs_mod_power_of_two_square(&mut xs, pow);
            assert_eq!(limbs_mod_power_of_two_square_ref(xs_old, pow), out);

            let expected_square = Natural::from_limbs_asc(xs_old).mod_power_of_two_square(pow);
            assert_eq!(Natural::from_owned_limbs_asc(out), expected_square);
        },
    );
}

#[test]
fn mod_power_of_two_square_properties() {
    test_properties(pairs_of_natural_and_u64_var_1, |&(ref n, pow)| {
        assert!(n.mod_power_of_two_is_reduced(pow));
        let square = n.mod_power_of_two_square(pow);
        assert!(square.is_valid());
        assert!(square.mod_power_of_two_is_reduced(pow));

        let square_alt = n.clone().mod_power_of_two_square(pow);
        assert!(square_alt.is_valid());
        assert_eq!(square_alt, square);

        let mut n_alt = n.clone();
        n_alt.mod_power_of_two_square_assign(pow);
        assert!(square_alt.is_valid());
        assert_eq!(square_alt, square);

        assert_eq!(square, n.square().mod_power_of_two(pow));
        assert_eq!(square, n.mod_square(Natural::power_of_two(pow)));
        assert_eq!(
            n.mod_power_of_two_neg(pow).mod_power_of_two_square(pow),
            square
        );
    });

    test_properties_no_special(small_unsigneds, |&pow| {
        assert_eq!(Natural::ZERO.mod_power_of_two_square(pow), 0);
        if pow != 0 {
            assert_eq!(Natural::ONE.mod_power_of_two_square(pow), 1);
        }
    });

    test_properties(
        triples_of_natural_natural_and_u64_var_1,
        |&(ref x, ref y, pow)| {
            assert_eq!(
                x.mod_power_of_two_mul(y, pow).mod_power_of_two_square(pow),
                x.mod_power_of_two_square(pow)
                    .mod_power_of_two_mul(y.mod_power_of_two_square(pow), pow)
            );
        },
    );

    test_properties_no_special(
        pairs_of_unsigned_and_small_u64_var_2::<Limb>,
        |&(n, pow)| {
            assert_eq!(
                n.mod_power_of_two_square(pow),
                Natural::from(n).mod_power_of_two_square(pow)
            );
        },
    );
}
