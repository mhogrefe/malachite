use malachite_base::num::arithmetic::traits::{Pow, PowAssign, PowerOfTwo, Square};
use malachite_base::num::basic::traits::{Iverson, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_set_zero;
use malachite_nz::natural::arithmetic::pow::{
    _limb_pow_alt_estimated_out_len, _limb_pow_alt_estimated_scratch_len,
    _limbs_pow_alt_estimated_out_len, _limbs_pow_alt_estimated_scratch_len, limb_pow_alt,
    limb_pow_to_out_alt, limbs_pow_alt, limbs_pow_to_out_alt,
};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::natural::arithmetic::pow::{
    natural_pow_naive, natural_pow_simple_binary,
};
use num::traits::Pow as NumPow;
use rug::ops::Pow as RugPow;

use malachite_test::common::{
    test_properties, test_properties_custom_scale, test_properties_no_special,
};
use malachite_test::inputs::base::{
    pairs_of_small_unsigneds_var_2, pairs_of_unsigned_and_small_unsigned_var_2,
    pairs_of_unsigned_vec_and_small_unsigned_var_3, small_unsigneds,
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_2,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned, triples_of_natural_natural_and_small_unsigned,
    triples_of_natural_small_unsigned_and_small_unsigned,
};

#[test]
fn limb_pow_properties() {
    test_properties(pairs_of_unsigned_and_small_unsigned_var_2, |&(x, exp)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limb_pow_alt(x, exp)),
            Natural::from(x).pow(exp)
        );
    });
}

#[test]
fn limb_pow_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_3,
        |&(ref out, x, exp)| {
            let mut out = out.to_vec();
            let mut old_out = out.clone();
            let mut scratch = vec![0; _limb_pow_alt_estimated_scratch_len(x, exp)];
            let estimated_out_len = _limb_pow_alt_estimated_out_len(x, exp);
            let out_len = limb_pow_to_out_alt(&mut out, x, exp, &mut scratch);
            slice_set_zero(&mut old_out[..estimated_out_len]);
            let n = Natural::from(x).pow(exp);
            let mut limbs = n.into_limbs_asc();
            limbs.resize(out_len, 0);
            assert_eq!(limbs, &out[..out_len]);
            assert_eq!(&out[estimated_out_len..], &old_out[estimated_out_len..]);
        },
    );
}

#[test]
fn limbs_pow_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_3,
        |&(ref xs, exp)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_pow_alt(xs, exp)),
                Natural::from_limbs_asc(xs).pow(exp)
            );
        },
    );
}

#[test]
fn limbs_pow_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_2,
        |&(ref out, ref xs, exp)| {
            let mut out = out.to_vec();
            let mut old_out = out.clone();
            let mut scratch = vec![0; _limbs_pow_alt_estimated_scratch_len(xs, exp)];
            let estimated_out_len = _limbs_pow_alt_estimated_out_len(xs, exp);
            let out_len = limbs_pow_to_out_alt(&mut out, xs, exp, &mut scratch);
            slice_set_zero(&mut old_out[..estimated_out_len]);
            let n = Natural::from_limbs_asc(xs).pow(exp);
            let mut limbs = n.into_limbs_asc();
            limbs.resize(out_len, 0);
            assert_eq!(limbs, &out[..out_len]);
            assert_eq!(&out[estimated_out_len..], &old_out[estimated_out_len..]);
        },
    );
}

#[test]
fn pow_properties() {
    test_properties_custom_scale(16, pairs_of_natural_and_small_unsigned, |&(ref x, exp)| {
        let power = x.pow(exp);
        assert!(power.is_valid());

        let power_alt = x.clone().pow(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let mut power_alt = x.clone();
        power_alt.pow_assign(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        assert_eq!(biguint_to_natural(&natural_to_biguint(x).pow(exp)), power);
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).pow(u32::exact_from(exp))),
            power
        );

        assert_eq!(power, natural_pow_naive(x, exp));
        assert_eq!(power, natural_pow_simple_binary(x, exp));
    });

    test_properties_no_special(pairs_of_small_unsigneds_var_2::<u64>, |&(x, y)| {
        assert_eq!(Pow::pow(x, y), Natural::from(x).pow(y));
    });

    test_properties(naturals, |x| {
        assert_eq!(x.pow(0), 1);
        assert_eq!(x.pow(1), *x);
        assert_eq!(x.pow(2), x.square());
    });

    test_properties_no_special(small_unsigneds, |&exp| {
        assert_eq!(Natural::ZERO.pow(exp), u64::iverson(exp == 0));
        assert_eq!(Natural::ONE.pow(exp), 1);
        assert_eq!(Natural::TWO.pow(exp), Natural::power_of_two(exp));
    });

    test_properties_custom_scale(
        16,
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, exp)| {
            assert_eq!((x * y).pow(exp), x.pow(exp) * y.pow(exp));
        },
    );

    test_properties_custom_scale(
        16,
        triples_of_natural_small_unsigned_and_small_unsigned,
        |&(ref x, e, f)| {
            assert_eq!(x.pow(e + f), x.pow(e) * x.pow(f));
            assert_eq!(x.pow(e * f), x.pow(e).pow(f));
        },
    );
}
