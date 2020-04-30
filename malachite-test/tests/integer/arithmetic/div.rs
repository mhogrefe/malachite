use malachite_base::num::arithmetic::traits::DivRem;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;

use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer,
    pairs_of_integer_and_nonzero_integer_var_1,
};

fn div_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    mut_x /= y;
    assert!(mut_x.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    mut_x /= y.clone();
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x / y;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x / y.clone();
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.clone() / y;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.clone() / y.clone();
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.div_rem(y).0;
    assert_eq!(quotient_alt, quotient);

    let num_quotient = integer_to_bigint(x) / &integer_to_bigint(y);
    assert_eq!(bigint_to_integer(&num_quotient), quotient);

    let rug_quotient = integer_to_rug_integer(x) / integer_to_rug_integer(y);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);

    let remainder = x - &quotient * y;
    assert!(remainder.lt_abs(y));
    assert!(remainder == Integer::ZERO || (remainder > Integer::ZERO) == (*x > Integer::ZERO));
    assert_eq!(&quotient * y + &remainder, *x);
    assert_eq!((-x) / y, -&quotient);
    assert_eq!(x / (-y), -quotient);
}

#[test]
fn div_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer,
        |&(ref x, ref y)| {
            div_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            div_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x / Integer::ONE, *x);
        assert_eq!(x / Integer::NEGATIVE_ONE, -x);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(Integer::ZERO / x, 0);
        if *x > Integer::ONE {
            assert_eq!(Integer::ONE / x, 0);
        }
        assert_eq!(x / Integer::ONE, *x);
        assert_eq!(x / Integer::NEGATIVE_ONE, -x);
        assert_eq!(x / x, 1);
        assert_eq!(x / -x, -1);
    });
}
