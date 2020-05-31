use malachite_base::num::arithmetic::traits::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, DivRem, Mod, ModAssign,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use num::Integer as NumInteger;
use rug::ops::RemRounding;

use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer,
    pairs_of_integer_and_nonzero_integer_var_1,
};

fn mod_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    mut_x.mod_assign(y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.mod_op(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.mod_op(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().mod_op(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().mod_op(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(x.div_mod(y).1, remainder);

    let num_remainder = integer_to_bigint(x).mod_floor(&integer_to_bigint(y));
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let rug_remainder = integer_to_rug_integer(x).rem_floor(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == 0 || (remainder > 0) == (*y > 0));

    assert_eq!((-x).mod_op(y), -x.ceiling_mod(y));
    assert_eq!(x.mod_op(-y), x.ceiling_mod(y));
}

#[test]
fn mod_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer,
        |&(ref x, ref y)| {
            mod_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            mod_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x.mod_op(Integer::ONE), 0);
        assert_eq!(x.mod_op(Integer::NEGATIVE_ONE), 0);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x.mod_op(Integer::ONE), 0);
        assert_eq!(x.mod_op(Integer::NEGATIVE_ONE), 0);
        assert_eq!(x.mod_op(x), 0);
        assert_eq!(x.mod_op(-x), 0);
        assert_eq!(Integer::ZERO.mod_op(x), 0);
        if *x > 1 {
            assert_eq!(Integer::ONE.mod_op(x), 1);
            assert_eq!(Integer::NEGATIVE_ONE.mod_op(x), x - Integer::ONE);
        }
    });
}

fn rem_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    mut_x %= y;
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x %= y.clone();
    assert!(mut_x.is_valid());
    assert_eq!(mut_x, remainder);

    let remainder_alt = x % y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x % y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone() % y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone() % y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(x.div_rem(y).1, remainder);

    let num_remainder = integer_to_bigint(x) % &integer_to_bigint(y);
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let rug_remainder = integer_to_rug_integer(x) % integer_to_rug_integer(y);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == 0 || (remainder > 0) == (*x > 0));

    assert_eq!((-x) % y, -&remainder);
    assert_eq!(x % (-y), remainder);
}

#[test]
fn rem_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer,
        |&(ref x, ref y)| {
            rem_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            rem_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x % Integer::ONE, 0);
        assert_eq!(x % Integer::NEGATIVE_ONE, 0);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x % Integer::ONE, 0);
        assert_eq!(x % Integer::NEGATIVE_ONE, 0);
        assert_eq!(x % x, 0);
        assert_eq!(x % -x, 0);
        assert_eq!(Integer::ZERO % x, 0);
        if *x > 1 {
            assert_eq!(Integer::ONE % x, 1);
            assert_eq!(Integer::NEGATIVE_ONE % x, -1);
        }
    });
}

fn ceiling_mod_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    mut_x.ceiling_mod_assign(y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.ceiling_mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.ceiling_mod(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.ceiling_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().ceiling_mod(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().ceiling_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(x.ceiling_div_mod(y).1, remainder);

    let rug_remainder = integer_to_rug_integer(x).rem_ceil(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == 0 || (remainder >= 0) != (*y > 0));

    assert_eq!((-x).ceiling_mod(y), -x.mod_op(y));
    assert_eq!(x.ceiling_mod(-y), x.mod_op(y));
}

#[test]
fn ceiling_mod_properties() {
    test_properties(pairs_of_integer_and_nonzero_integer, |&(ref x, ref y)| {
        ceiling_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            ceiling_mod_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x.ceiling_mod(Integer::ONE), 0);
        assert_eq!(x.ceiling_mod(Integer::NEGATIVE_ONE), 0);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x.ceiling_mod(Integer::ONE), 0);
        assert_eq!(x.ceiling_mod(Integer::NEGATIVE_ONE), 0);
        assert_eq!(x.ceiling_mod(x), 0);
        assert_eq!(x.ceiling_mod(-x), 0);
        assert_eq!(Integer::ZERO.ceiling_mod(x), 0);
    });
}
