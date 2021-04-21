use malachite_base::num::arithmetic::traits::{Pow, PowAssign, PowerOf2, Square};
use malachite_base::num::basic::traits::{Iverson, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::arithmetic::pow::limbs_pow;
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
    pairs_of_small_unsigneds_var_2, pairs_of_unsigned_vec_and_small_unsigned_var_3, small_unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned, triples_of_natural_natural_and_small_unsigned,
    triples_of_natural_small_unsigned_and_small_unsigned,
};

#[test]
fn limbs_pow_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_3,
        |&(ref xs, exp)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_pow(xs, exp)),
                Natural::from_limbs_asc(xs).pow(exp)
            );
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
        assert_eq!(power, x.pow_ref_alt(exp));
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
        assert_eq!(Natural::TWO.pow(exp), Natural::power_of_2(exp));
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
