use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use num::Integer as NumInteger;

use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer,
    pairs_of_integer_and_nonzero_integer_var_1,
};

fn div_mod_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    let remainder = mut_x.div_assign_mod(y);
    assert!(mut_x.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    let remainder_alt = mut_x.div_assign_mod(y.clone());
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (x.div_round(y, RoundingMode::Floor), x.mod_op(y));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (num_quotient, num_remainder) = integer_to_bigint(x).div_mod_floor(&integer_to_bigint(y));
    assert_eq!(bigint_to_integer(&num_quotient), quotient);
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let (rug_quotient, rug_remainder) =
        integer_to_rug_integer(x).div_rem_floor(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == 0 || (remainder > 0) == (*y > 0));
    assert_eq!(quotient * y + remainder, *x);

    let (neg_quotient, neg_remainder) = (-x).div_mod(y);
    assert_eq!(x.ceiling_div_mod(y), (-neg_quotient, -neg_remainder));

    let (neg_quotient, remainder) = x.div_mod(-y);
    assert_eq!(x.ceiling_div_mod(y), (-neg_quotient, remainder));
}

#[test]
fn div_mod_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer,
        |&(ref x, ref y)| {
            div_mod_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            div_mod_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        let (q, r) = x.div_mod(Integer::ONE);
        assert_eq!(q, *x);
        assert_eq!(r, 0);

        let (q, r) = x.div_mod(Integer::NEGATIVE_ONE);
        assert_eq!(q, -x);
        assert_eq!(r, 0);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x.div_mod(Integer::ONE), (x.clone(), Integer::ZERO));
        assert_eq!(x.div_mod(Integer::NEGATIVE_ONE), (-x, Integer::ZERO));
        assert_eq!(x.div_mod(x), (Integer::ONE, Integer::ZERO));
        assert_eq!(x.div_mod(-x), (Integer::NEGATIVE_ONE, Integer::ZERO));
        assert_eq!(Integer::ZERO.div_mod(x), (Integer::ZERO, Integer::ZERO));
        if *x > 1 {
            assert_eq!(Integer::ONE.div_mod(x), (Integer::ZERO, Integer::ONE));
            assert_eq!(
                Integer::NEGATIVE_ONE.div_mod(x),
                (Integer::NEGATIVE_ONE, x - Integer::ONE)
            );
        }
    });
}

fn div_rem_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    let remainder = mut_x.div_assign_rem(y);
    assert!(mut_x.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_x;

    let mut quotient_alt = x.clone();
    let remainder_alt = quotient_alt.div_assign_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_rem(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_rem(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (x / y, x % y);
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (num_quotient, num_remainder) = integer_to_bigint(x).div_rem(&integer_to_bigint(y));
    assert_eq!(bigint_to_integer(&num_quotient), quotient);
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let (rug_quotient, rug_remainder) =
        integer_to_rug_integer(x).div_rem(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == 0 || (remainder > 0) == (*x > 0));
    assert_eq!(&quotient * y + &remainder, *x);

    assert_eq!((-x).div_rem(y), (-&quotient, -&remainder));
    assert_eq!(x.div_rem(-y), (-quotient, remainder));
}

#[test]
fn div_rem_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer,
        |&(ref x, ref y)| {
            div_rem_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            div_rem_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        let (q, r) = x.div_rem(Integer::ONE);
        assert_eq!(q, *x);
        assert_eq!(r, 0);

        let (q, r) = x.div_rem(Integer::NEGATIVE_ONE);
        assert_eq!(q, -x);
        assert_eq!(r, 0);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x.div_rem(Integer::ONE), (x.clone(), Integer::ZERO));
        assert_eq!(x.div_rem(Integer::NEGATIVE_ONE), (-x, Integer::ZERO));
        assert_eq!(x.div_rem(x), (Integer::ONE, Integer::ZERO));
        assert_eq!(x.div_rem(-x), (Integer::NEGATIVE_ONE, Integer::ZERO));
        assert_eq!(Integer::ZERO.div_rem(x), (Integer::ZERO, Integer::ZERO));
        if *x > 1 {
            assert_eq!(Integer::ONE.div_rem(x), (Integer::ZERO, Integer::ONE));
            assert_eq!(
                Integer::NEGATIVE_ONE.div_rem(x),
                (Integer::ZERO, Integer::NEGATIVE_ONE)
            );
        }
    });
}

fn ceiling_div_mod_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    let remainder = mut_x.ceiling_div_assign_mod(y);
    assert!(mut_x.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    let remainder_alt = mut_x.ceiling_div_assign_mod(y.clone());
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.ceiling_div_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.ceiling_div_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().ceiling_div_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().ceiling_div_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (x.div_round(y, RoundingMode::Ceiling), x.ceiling_mod(y));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (rug_quotient, rug_remainder) =
        integer_to_rug_integer(x).div_rem_ceil(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == 0 || (remainder >= 0) != (*y > 0));
    assert_eq!(quotient * y + remainder, *x);

    let (neg_quotient, neg_remainder) = (-x).ceiling_div_mod(y);
    assert_eq!(x.div_mod(y), (-neg_quotient, -neg_remainder));

    let (neg_quotient, remainder) = x.ceiling_div_mod(-y);
    assert_eq!(x.div_mod(y), (-neg_quotient, remainder));
}

#[test]
fn ceiling_div_mod_properties() {
    test_properties(pairs_of_integer_and_nonzero_integer, |&(ref x, ref y)| {
        ceiling_div_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            ceiling_div_mod_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        let (q, r) = x.ceiling_div_mod(Integer::ONE);
        assert_eq!(q, *x);
        assert_eq!(r, 0);

        let (q, r) = x.ceiling_div_mod(Integer::NEGATIVE_ONE);
        assert_eq!(q, -x);
        assert_eq!(r, 0);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x.ceiling_div_mod(Integer::ONE), (x.clone(), Integer::ZERO));
        assert_eq!(
            x.ceiling_div_mod(Integer::NEGATIVE_ONE),
            (-x, Integer::ZERO)
        );
        assert_eq!(x.ceiling_div_mod(x), (Integer::ONE, Integer::ZERO));
        assert_eq!(
            x.ceiling_div_mod(-x),
            (Integer::NEGATIVE_ONE, Integer::ZERO)
        );
        assert_eq!(
            Integer::ZERO.ceiling_div_mod(x),
            (Integer::ZERO, Integer::ZERO)
        );
    });
}
