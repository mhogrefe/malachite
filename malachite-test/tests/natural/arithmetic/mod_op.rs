use malachite_base::num::arithmetic::traits::{
    DivMod, Mod, ModAssign, ModIsReduced, NegMod, NegModAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::JoinHalves;
use malachite_nz::natural::arithmetic::div_mod::limbs_div_mod_barrett_scratch_len;
use malachite_nz::natural::arithmetic::mod_op::{
    _limbs_mod_barrett, _limbs_mod_divide_and_conquer, _limbs_mod_limb_alt_1,
    _limbs_mod_limb_alt_2, _limbs_mod_limb_any_leading_zeros_1,
    _limbs_mod_limb_any_leading_zeros_2, _limbs_mod_limb_at_least_1_leading_zero,
    _limbs_mod_limb_at_least_2_leading_zeros, _limbs_mod_limb_small_normalized,
    _limbs_mod_limb_small_unnormalized, _limbs_mod_schoolbook, limbs_mod,
    limbs_mod_by_two_limb_normalized, limbs_mod_limb, limbs_mod_three_limb_by_two_limb,
    limbs_mod_to_out,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::natural::arithmetic::mod_op::_limbs_mod_limb_alt_3;
use malachite_nz_test_util::natural::arithmetic::mod_op::rug_neg_mod;
use num::Integer;
use rug::ops::RemRounding;

use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_var_9, pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1,
    pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2,
    pairs_of_nonempty_unsigned_vec_and_unsigned_var_1, pairs_of_unsigned_and_positive_unsigned,
    pairs_of_unsigned_vec_and_positive_unsigned_var_1, pairs_of_unsigned_vec_var_10,
    quadruples_of_limb_vec_var_1, quadruples_of_three_limb_vecs_and_limb_var_2,
    sextuples_of_limbs_var_1, triples_of_limb_vec_var_45, triples_of_two_limb_vecs_and_limb_var_1,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_1,
    positive_naturals, triples_of_natural_natural_and_positive_natural,
};

fn verify_limbs_mod_1(ns_in: &[Limb], ds: &[Limb], ns_out: &[Limb]) {
    let n = Natural::from_limbs_asc(ns_in);
    let d = Natural::from_limbs_asc(ds);
    let expected_r = n % &d;
    let r = Natural::from_limbs_asc(&ns_out[..ds.len()]);
    assert_eq!(r, expected_r);
    assert!(r < d);
}

fn verify_limbs_mod_2(rs_in: &[Limb], ns: &[Limb], ds: &[Limb], rs_out: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let d_len = ds.len();
    let expected_r = n % &d;
    let r = Natural::from_limbs_asc(&rs_out[..d_len]);
    assert_eq!(r, expected_r);
    assert_eq!(&rs_in[d_len..], &rs_out[d_len..]);
    assert!(r < d);
}

fn verify_limbs_mod_3(ns: &[Limb], ds: &[Limb], rs: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let expected_r = n % &d;
    let r = Natural::from_limbs_asc(rs);
    assert_eq!(r, expected_r);
    assert!(r < d);
    assert_eq!(rs.len(), ds.len());
}

fn verify_limbs_mod_three_limb_by_two_limb(
    n_2: Limb,
    n_1: Limb,
    n_0: Limb,
    d_1: Limb,
    d_0: Limb,
    r: DoubleLimb,
) {
    let n = Natural::from_owned_limbs_asc(vec![n_0, n_1, n_2]);
    let d = Natural::from(DoubleLimb::join_halves(d_1, d_0));
    let r = Natural::from(r);
    assert_eq!(n % &d, r);
    assert!(r < d);
}

fn verify_limbs_mod_by_two_limb_normalized(ns: &[Limb], ds: &[Limb], r_0: Limb, r_1: Limb) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let expected_r = n % &d;
    let r = Natural::from_owned_limbs_asc(vec![r_0, r_1]);
    assert_eq!(r, expected_r);
    assert!(r < d);
}

#[test]
fn limbs_mod_limb_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, divisor)| {
            let remainder = limbs_mod_limb(limbs, divisor);
            assert_eq!(
                Natural::from_limbs_asc(limbs) % Natural::from(divisor),
                remainder
            );
            assert_eq!(
                _limbs_mod_limb_any_leading_zeros_1(limbs, divisor),
                remainder
            );
            assert_eq!(
                _limbs_mod_limb_any_leading_zeros_2(limbs, divisor),
                remainder
            );
            assert_eq!(_limbs_mod_limb_alt_1(limbs, divisor), remainder);
            assert_eq!(_limbs_mod_limb_alt_2(limbs, divisor), remainder);
            assert_eq!(_limbs_mod_limb_alt_3(limbs, divisor), remainder);
        },
    );
}

#[test]
fn _limbs_mod_limb_small_normalized_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned_var_1,
        |&(ref limbs, divisor)| {
            let remainder = _limbs_mod_limb_small_normalized(limbs, divisor);
            assert_eq!(
                remainder,
                Natural::from_limbs_asc(limbs) % Natural::from(divisor)
            );
            if limbs.len() == 1 {
                assert_eq!(remainder, limbs[0] % divisor);
            } else {
                assert_eq!(remainder, limbs_mod_limb(limbs, divisor));
            }
        },
    );
}

#[test]
fn _limbs_mod_limb_small_unnormalized_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, divisor)| {
            let remainder = _limbs_mod_limb_small_unnormalized(limbs, divisor);
            assert_eq!(
                remainder,
                _limbs_mod_limb_at_least_1_leading_zero(limbs, divisor)
            );
            assert_eq!(
                remainder,
                Natural::from_limbs_asc(limbs) % Natural::from(divisor)
            );
            if limbs.len() == 1 {
                assert_eq!(remainder, limbs[0] % divisor);
            } else {
                assert_eq!(remainder, limbs_mod_limb(limbs, divisor));
            }
        },
    );
}

#[test]
fn _limbs_mod_limb_at_least_2_leading_zeros_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2,
        |&(ref limbs, divisor)| {
            let remainder = _limbs_mod_limb_at_least_2_leading_zeros(limbs, divisor);
            assert_eq!(
                remainder,
                Natural::from_limbs_asc(limbs) % Natural::from(divisor)
            );
            if limbs.len() == 1 {
                assert_eq!(remainder, limbs[0] % divisor);
            } else {
                assert_eq!(remainder, limbs_mod_limb(limbs, divisor));
            }
        },
    );
}

#[test]
fn limbs_mod_three_limb_by_two_limb_properties() {
    test_properties(
        sextuples_of_limbs_var_1,
        |&(n_2, n_1, n_0, d_1, d_0, inverse)| {
            let r = limbs_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, inverse);
            verify_limbs_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, r);
        },
    );
}

#[test]
fn limbs_mod_by_two_limb_normalized_properties() {
    test_properties(pairs_of_unsigned_vec_var_10, |(ns, ds)| {
        let (r_0, r_1) = limbs_mod_by_two_limb_normalized(&ns, &ds);
        verify_limbs_mod_by_two_limb_normalized(&ns, &ds, r_0, r_1);
    });
}

#[test]
fn limbs_mod_schoolbook_properties() {
    test_properties(
        triples_of_two_limb_vecs_and_limb_var_1,
        |(ref ns_in, ref ds, inverse)| {
            let mut ns = ns_in.clone();
            _limbs_mod_schoolbook(&mut ns, ds, *inverse);
            verify_limbs_mod_1(ns_in, ds, &ns);
        },
    );
}

#[test]
fn limbs_mod_divide_and_conquer_properties() {
    test_properties_custom_scale(
        128,
        quadruples_of_three_limb_vecs_and_limb_var_2,
        |(ref qs_in, ref ns_in, ref ds, inverse)| {
            let mut qs = qs_in.clone();
            let mut ns = ns_in.clone();
            _limbs_mod_divide_and_conquer(&mut qs, &mut ns, ds, *inverse);
            verify_limbs_mod_1(ns_in, ds, &ns);
        },
    );
}

#[test]
fn limbs_mod_barrett_properties() {
    test_properties_custom_scale(
        512,
        quadruples_of_limb_vec_var_1,
        |(ref qs_in, ref rs_in, ref ns, ref ds)| {
            let mut qs = qs_in.clone();
            let mut rs = rs_in.clone();
            let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
            _limbs_mod_barrett(&mut qs, &mut rs, ns, ds, &mut scratch);
            verify_limbs_mod_2(rs_in, ns, ds, &rs);
        },
    );
}

#[test]
fn limbs_mod_properties() {
    test_properties_custom_scale(512, pairs_of_limb_vec_var_9, |(ref ns, ref ds)| {
        verify_limbs_mod_3(ns, ds, &limbs_mod(ns, ds));
    });
}

#[test]
fn limbs_mod_to_out_properties() {
    test_properties_custom_scale(
        512,
        triples_of_limb_vec_var_45,
        |(ref rs_in, ref ns, ref ds)| {
            let mut rs = rs_in.clone();
            limbs_mod_to_out(&mut rs, ns, ds);
            verify_limbs_mod_2(rs_in, ns, ds, &rs);
        },
    );
}

fn mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    mut_x.mod_assign(y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;
    assert!(remainder.mod_is_reduced(y));

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

    let mut remainder_alt = x.clone();
    remainder_alt %= y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let mut remainder_alt = x.clone();
    remainder_alt %= y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

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

    let remainder_alt = x.div_mod(y).1;
    assert_eq!(remainder_alt, remainder);

    let num_remainder = natural_to_biguint(x).mod_floor(&natural_to_biguint(y));
    assert_eq!(biguint_to_natural(&num_remainder), remainder);

    let num_remainder = natural_to_biguint(x) % &natural_to_biguint(y);
    assert_eq!(biguint_to_natural(&num_remainder), remainder);

    let rug_remainder = natural_to_rug_integer(x).rem_floor(natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    let rug_remainder = natural_to_rug_integer(x) % natural_to_rug_integer(y);
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    assert!(remainder < *y);
    assert_eq!(&remainder % y, remainder);
}

#[test]
fn mod_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural,
        |&(ref x, ref y)| {
            mod_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            mod_properties_helper(x, y);
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n % Natural::ONE, 0);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(n % n, 0);
        assert_eq!(Natural::ZERO % n, 0);
        if *n > 1 {
            assert_eq!(Natural::ONE % n, 1);
        }
    });

    test_properties(
        triples_of_natural_natural_and_positive_natural,
        |&(ref x, ref y, ref z)| {
            assert_eq!((x + y) % z, (x % z + y % z) % z);
            assert_eq!(x * y % z, (x % z) * (y % z) % z);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb, Limb>,
        |&(x, y)| {
            assert_eq!(Natural::from(x) % Natural::from(y), x % y);
        },
    );
}

fn neg_mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    mut_x.neg_mod_assign(y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;
    assert!(remainder.mod_is_reduced(y));

    let mut mut_x = x.clone();
    mut_x.neg_mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.neg_mod(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.neg_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().neg_mod(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().neg_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.neg_mod(y);
    assert_eq!(remainder_alt, remainder);

    let rug_remainder = rug_neg_mod(natural_to_rug_integer(x), natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    assert!(remainder < *y);
}

#[test]
fn neg_mod_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        neg_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            neg_mod_properties_helper(x, y);
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.neg_mod(Natural::ONE), 0);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(n.neg_mod(n), 0);
        assert_eq!(Natural::ZERO.neg_mod(n), 0);
        assert_eq!(Natural::ONE.neg_mod(n), n - Natural::ONE);
    });

    test_properties(
        triples_of_natural_natural_and_positive_natural,
        |&(ref x, ref y, ref z)| {
            assert_eq!((x + y).neg_mod(z), (x % z + y % z).neg_mod(z));
            assert_eq!((x * y).neg_mod(z), ((x % z) * (y % z)).neg_mod(z));
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb, Limb>,
        |&(x, y)| {
            assert_eq!(Natural::from(x).neg_mod(Natural::from(y)), x.neg_mod(y));
        },
    );
}
