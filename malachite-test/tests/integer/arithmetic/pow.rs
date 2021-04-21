use malachite_base::num::arithmetic::traits::{Parity, Pow, PowAssign, PowerOf2, Square};
use malachite_base::num::basic::traits::{Iverson, NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use num::traits::Pow as NumPow;
use rug::ops::Pow as RugPow;

use malachite_test::common::{
    test_properties, test_properties_custom_scale, test_properties_no_special,
};
use malachite_test::inputs::base::{pairs_of_small_signed_and_small_u64_var_2, small_unsigneds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_unsigned, triples_of_integer_integer_and_small_unsigned,
    triples_of_integer_small_unsigned_and_small_unsigned,
};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn pow_properties() {
    test_properties_custom_scale(16, pairs_of_integer_and_small_unsigned, |&(ref x, exp)| {
        let power = x.pow(exp);
        assert!(power.is_valid());

        let power_alt = x.clone().pow(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let mut power_alt = x.clone();
        power_alt.pow_assign(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let power_of_neg = (-x).pow(exp);
        if exp.even() {
            assert_eq!(power_of_neg, power);
        } else {
            assert_eq!(power_of_neg, -&power);
        }

        assert_eq!(bigint_to_integer(&integer_to_bigint(x).pow(exp)), power);
        assert_eq!(
            rug_integer_to_integer(&integer_to_rug_integer(x).pow(u32::exact_from(exp))),
            power
        );
    });

    test_properties_no_special(
        pairs_of_small_signed_and_small_u64_var_2::<SignedLimb>,
        |&(x, y)| {
            assert_eq!(Pow::pow(x, y), Integer::from(x).pow(y));
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref x, exp)| {
        assert_eq!(x.pow(exp), Integer::from(x).pow(exp));
    });

    test_properties(integers, |x| {
        assert_eq!(x.pow(0), 1);
        assert_eq!(x.pow(1), *x);
        assert_eq!(x.pow(2), x.square());
    });

    test_properties_no_special(small_unsigneds, |&exp| {
        assert_eq!(Integer::ZERO.pow(exp), u64::iverson(exp == 0));
        assert_eq!(Integer::ONE.pow(exp), 1);
        assert_eq!(Integer::TWO.pow(exp), Integer::power_of_2(exp));

        assert_eq!(
            Integer::NEGATIVE_ONE.pow(exp),
            if exp.even() { 1 } else { -1 }
        );
    });

    test_properties_custom_scale(
        16,
        triples_of_integer_integer_and_small_unsigned,
        |&(ref x, ref y, exp)| {
            assert_eq!((x * y).pow(exp), x.pow(exp) * y.pow(exp));
        },
    );

    test_properties_custom_scale(
        16,
        triples_of_integer_small_unsigned_and_small_unsigned,
        |&(ref x, e, f)| {
            assert_eq!(x.pow(e + f), x.pow(e) * x.pow(f));
            assert_eq!(x.pow(e * f), x.pow(e).pow(f));
        },
    );
}
