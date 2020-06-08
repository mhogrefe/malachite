use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use num::Integer as NumInteger;

use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::base::pairs_of_signeds_var_2;
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer,
    pairs_of_integer_and_nonzero_integer_var_1,
};

fn div_mod_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    let r = mut_x.div_assign_mod(y);
    assert!(mut_x.is_valid());
    assert!(r.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    let r_alt = mut_x.div_assign_mod(y.clone());
    let q_alt = mut_x;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.div_mod(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.div_mod(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().div_mod(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().div_mod(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = (x.div_round(y, RoundingMode::Floor), x.mod_op(y));
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);

    let (num_q, num_r) = integer_to_bigint(x).div_mod_floor(&integer_to_bigint(y));
    assert_eq!(bigint_to_integer(&num_q), q);
    assert_eq!(bigint_to_integer(&num_r), r);

    let (rug_q, rug_r) = integer_to_rug_integer(x).div_rem_floor(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_q), q);
    assert_eq!(rug_integer_to_integer(&rug_r), r);

    assert!(r.lt_abs(y));
    assert!(r == 0 || (r > 0) == (*y > 0));
    assert_eq!(q * y + r, *x);

    let (neg_q, neg_r) = (-x).div_mod(y);
    assert_eq!(x.ceiling_div_mod(y), (-neg_q, -neg_r));

    let (neg_q, r) = x.div_mod(-y);
    assert_eq!(x.ceiling_div_mod(y), (-neg_q, r));
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

    test_properties(pairs_of_signeds_var_2::<SignedLimb>, |&(x, y)| {
        let (q, r) = x.div_mod(y);
        assert_eq!(
            Integer::from(x).div_mod(Integer::from(y)),
            (Integer::from(q), Integer::from(r))
        );
    });
}

fn div_rem_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    let r = mut_x.div_assign_rem(y);
    assert!(mut_x.is_valid());
    assert!(r.is_valid());
    let q = mut_x;

    let mut q_alt = x.clone();
    let r_alt = q_alt.div_assign_rem(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.div_rem(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.div_rem(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().div_rem(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().div_rem(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = (x / y, x % y);
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);

    let (num_q, num_r) = integer_to_bigint(x).div_rem(&integer_to_bigint(y));
    assert_eq!(bigint_to_integer(&num_q), q);
    assert_eq!(bigint_to_integer(&num_r), r);

    let (rug_q, rug_r) = integer_to_rug_integer(x).div_rem(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_q), q);
    assert_eq!(rug_integer_to_integer(&rug_r), r);

    assert!(r.lt_abs(y));
    assert!(r == 0 || (r > 0) == (*x > 0));
    assert_eq!(&q * y + &r, *x);

    assert_eq!((-x).div_rem(y), (-&q, -&r));
    assert_eq!(x.div_rem(-y), (-q, r));
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

    test_properties(pairs_of_signeds_var_2::<SignedLimb>, |&(x, y)| {
        let (q, r) = x.div_rem(y);
        assert_eq!(
            Integer::from(x).div_rem(Integer::from(y)),
            (Integer::from(q), Integer::from(r))
        );
    });
}

fn ceiling_div_mod_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    let r = mut_x.ceiling_div_assign_mod(y);
    assert!(mut_x.is_valid());
    assert!(r.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    let r_alt = mut_x.ceiling_div_assign_mod(y.clone());
    let q_alt = mut_x;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.ceiling_div_mod(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.ceiling_div_mod(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().ceiling_div_mod(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().ceiling_div_mod(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = (x.div_round(y, RoundingMode::Ceiling), x.ceiling_mod(y));
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);

    let (rug_q, rug_r) = integer_to_rug_integer(x).div_rem_ceil(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_q), q);
    assert_eq!(rug_integer_to_integer(&rug_r), r);

    assert!(r.lt_abs(y));
    assert!(r == 0 || (r > 0) != (*y > 0));
    assert_eq!(q * y + r, *x);

    let (neg_q, neg_r) = (-x).ceiling_div_mod(y);
    assert_eq!(x.div_mod(y), (-neg_q, -neg_r));

    let (neg_q, r) = x.ceiling_div_mod(-y);
    assert_eq!(x.div_mod(y), (-neg_q, r));
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

    test_properties(pairs_of_signeds_var_2::<SignedLimb>, |&(x, y)| {
        let (q, r) = x.ceiling_div_mod(y);
        assert_eq!(
            Integer::from(x).ceiling_div_mod(Integer::from(y)),
            (Integer::from(q), Integer::from(r))
        );
    });
}
