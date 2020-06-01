use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem, DivRound,
    NegMod, PowerOfTwo,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_limb_in_place_mod_alt, _limbs_div_limb_to_out_mod_alt, _limbs_div_mod_barrett,
    _limbs_div_mod_barrett_scratch_len, _limbs_div_mod_divide_and_conquer,
    _limbs_div_mod_schoolbook, _limbs_invert_approx, _limbs_invert_basecase_approx,
    _limbs_invert_newton_approx, limbs_div_limb_in_place_mod, limbs_div_limb_mod,
    limbs_div_limb_to_out_mod, limbs_div_mod, limbs_div_mod_by_two_limb_normalized,
    limbs_div_mod_three_limb_by_two_limb, limbs_div_mod_to_out, limbs_invert_limb,
    limbs_two_limb_inverse_helper,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::natural::arithmetic::div_mod::rug_ceiling_div_neg_mod;
use malachite_nz_test_util::natural::arithmetic::div_mod::{
    limbs_div_limb_in_place_mod_naive, limbs_div_limb_to_out_mod_naive,
};
use num::Integer;

use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_var_9, pairs_of_unsigned_and_positive_unsigned,
    pairs_of_unsigned_vec_and_positive_unsigned_var_1, pairs_of_unsigneds_var_2,
    quadruples_of_limb_vec_var_1, quadruples_of_limb_vec_var_2,
    quadruples_of_three_limb_vecs_and_limb_var_1, quadruples_of_three_limb_vecs_and_limb_var_2,
    sextuples_of_limbs_var_1, triples_of_limb_vec_var_38, triples_of_limb_vec_var_39,
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_var_37, unsigneds_var_1,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_1,
    positive_naturals,
};

fn verify_limbs_two_limb_inverse_helper(hi: Limb, lo: Limb, result: Limb) {
    let b = Natural::power_of_two(Limb::WIDTH);
    let b_cubed_minus_1 = Natural::low_mask(Limb::WIDTH * 3);
    let x = Natural::from(DoubleLimb::join_halves(hi, lo));
    let expected_result = &b_cubed_minus_1 / &x - &b;
    assert_eq!(result, expected_result);
    assert!(b_cubed_minus_1 - (Natural::from(result) + b) * &x < x);
}

fn verify_limbs_div_mod_three_limb_by_two_limb(
    n_2: Limb,
    n_1: Limb,
    n_0: Limb,
    d_1: Limb,
    d_0: Limb,
    q: Limb,
    r: DoubleLimb,
) {
    let n = Natural::from_owned_limbs_asc(vec![n_0, n_1, n_2]);
    let d = Natural::from(DoubleLimb::join_halves(d_1, d_0));
    let r = Natural::from(r);
    assert_eq!((&n).div_mod(&d), (Natural::from(q), r.clone()));
    assert!(r < d);
    assert_eq!(Natural::from(q) * d + r, n);
}

fn verify_limbs_div_mod_by_two_limb_normalized(
    qs_in: &[Limb],
    ns_in: &[Limb],
    ds: &[Limb],
    q_highest: bool,
    qs_out: &[Limb],
    ns_out: &[Limb],
) {
    let n = Natural::from_limbs_asc(ns_in);
    let d = Natural::from_limbs_asc(ds);
    let (expected_q, expected_r) = (&n).div_mod(&d);
    let base_q_len = ns_in.len() - 2;
    let mut qs = qs_out[..base_q_len].to_vec();
    if q_highest {
        qs.push(1);
    }
    let q = Natural::from_owned_limbs_asc(qs);
    let r = Natural::from_limbs_asc(&ns_out[..2]);
    assert_eq!(q, expected_q);
    assert_eq!(r, expected_r);
    assert_eq!(&qs_in[base_q_len..], &qs_out[base_q_len..]);
    assert_eq!(&ns_in[2..], &ns_out[2..]);

    assert!(r < d);
    assert_eq!(q * d + r, n);
}

fn verify_limbs_div_mod_1(
    qs_in: &[Limb],
    ns_in: &[Limb],
    ds: &[Limb],
    q_highest: bool,
    qs_out: &[Limb],
    ns_out: &[Limb],
) {
    let n = Natural::from_limbs_asc(ns_in);
    let d = Natural::from_limbs_asc(ds);
    let (expected_q, expected_r) = (&n).div_mod(&d);
    let base_q_len = ns_in.len() - ds.len();
    let mut qs = qs_out[..base_q_len].to_vec();
    if q_highest {
        qs.push(1);
    }
    let q = Natural::from_owned_limbs_asc(qs);
    let r = Natural::from_limbs_asc(&ns_out[..ds.len()]);
    assert_eq!(q, expected_q);
    assert_eq!(r, expected_r);
    assert_eq!(&qs_in[base_q_len..], &qs_out[base_q_len..]);
    assert!(r < d);
    assert_eq!(q * d + r, n);
}

fn verify_limbs_invert_approx(
    is_in: &[Limb],
    ds: &[Limb],
    result_definitely_exact: bool,
    is_out: &[Limb],
) {
    let d = Natural::from_limbs_asc(ds);
    let n = ds.len();
    let bits = u64::exact_from(n << Limb::LOG_WIDTH);
    let product = Natural::power_of_two(bits << 1);
    //TODO compare to limbs_invert
    let mut expected_i = (&product - Natural::ONE) / &d;
    let offset = Natural::power_of_two(bits);
    expected_i -= &offset;
    let i = Natural::from_limbs_asc(&is_out[..n]);
    let x = (&i + &offset) * &d;
    let result_exact = i == expected_i;
    if result_definitely_exact {
        assert!(result_exact);
    }
    let y = if result_exact {
        assert_eq!(i, expected_i);
        (i + offset + Natural::ONE) * d
    } else {
        assert_eq!(&i + Natural::ONE, expected_i);
        (i + offset + Natural::TWO) * d
    };
    assert!(x < product);
    assert!(product <= y);
    assert_eq!(&is_in[n..], &is_out[n..]);
}

fn verify_limbs_div_mod_2(
    qs_in: &[Limb],
    rs_in: &[Limb],
    ns: &[Limb],
    ds: &[Limb],
    q_highest: bool,
    qs_out: &[Limb],
    rs_out: &[Limb],
) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let d_len = ds.len();
    let (expected_q, expected_r) = (&n).div_mod(&d);
    let base_q_len = ns.len() - d_len;
    let mut qs = qs_out[..base_q_len].to_vec();
    if q_highest {
        qs.push(1);
    }
    let q = Natural::from_owned_limbs_asc(qs);
    let r = Natural::from_limbs_asc(&rs_out[..d_len]);
    assert_eq!(q, expected_q);
    assert_eq!(r, expected_r);
    assert_eq!(&qs_in[base_q_len..], &qs_out[base_q_len..]);
    assert_eq!(&rs_in[d_len..], &rs_out[d_len..]);
    assert!(r < d);
    assert_eq!(q * d + r, n);
}

fn verify_limbs_div_mod_3(
    qs_in: &[Limb],
    rs_in: &[Limb],
    ns: &[Limb],
    ds: &[Limb],
    qs_out: &[Limb],
    rs_out: &[Limb],
) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let d_len = ds.len();
    let (expected_q, expected_r) = (&n).div_mod(&d);
    let base_q_len = ns.len() - d_len + 1;
    let qs = qs_out[..base_q_len].to_vec();
    let q = Natural::from_owned_limbs_asc(qs);
    let r = Natural::from_limbs_asc(&rs_out[..d_len]);
    assert_eq!(q, expected_q);
    assert_eq!(r, expected_r);
    assert_eq!(&qs_in[base_q_len..], &qs_out[base_q_len..]);
    assert_eq!(&rs_in[d_len..], &rs_out[d_len..]);
    assert!(r < d);
    assert_eq!(q * d + r, n);
}

fn verify_limbs_div_mod_4(ns: &[Limb], ds: &[Limb], qs: &[Limb], rs: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let (expected_q, expected_r) = (&n).div_mod(&d);
    let q = Natural::from_limbs_asc(qs);
    let r = Natural::from_limbs_asc(rs);
    assert_eq!(q, expected_q);
    assert_eq!(r, expected_r);
    assert!(r < d);
    assert_eq!(q * d + r, n);
    let d_len = ds.len();
    assert_eq!(qs.len(), ns.len() - d_len + 1);
    assert_eq!(rs.len(), d_len);
}

#[test]
fn limbs_invert_limb_properties() {
    test_properties(unsigneds_var_1, |&limb| {
        limbs_invert_limb(limb);
    });
}

#[test]
fn limbs_div_limb_mod_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            let (q_limbs, r) = limbs_div_limb_mod(limbs, limb);
            let (q, r_alt) = Natural::from_limbs_asc(limbs).div_mod(Natural::from(limb));
            assert_eq!(Natural::from_owned_limbs_asc(q_limbs), q);
            assert_eq!(r, r_alt);
        },
    );
}

#[test]
fn limbs_div_limb_to_out_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let r = limbs_div_limb_to_out_mod(&mut out, in_limbs, limb);
            let (q, r_alt) = Natural::from_limbs_asc(in_limbs).div_mod(Natural::from(limb));
            assert_eq!(r, r_alt);
            let len = in_limbs.len();
            assert_eq!(Natural::from_limbs_asc(&out[..len]), q);
            assert_eq!(&out[len..], &old_out[len..]);
            let final_out = out.clone();

            let mut out = old_out.to_vec();
            assert_eq!(_limbs_div_limb_to_out_mod_alt(&mut out, in_limbs, limb), r);
            assert_eq!(out, final_out);

            let mut out = old_out.to_vec();
            assert_eq!(limbs_div_limb_to_out_mod_naive(&mut out, in_limbs, limb), r);
            assert_eq!(out, final_out);
        },
    );
}

#[test]
fn limbs_div_limb_in_place_mod_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            let r = limbs_div_limb_in_place_mod(&mut limbs, limb);
            let (q, r_alt) = Natural::from_limbs_asc(&old_limbs).div_mod(Natural::from(limb));
            assert_eq!(Natural::from_owned_limbs_asc(limbs), q);
            assert_eq!(r, r_alt);

            let mut limbs = old_limbs.clone();
            let r_alt = _limbs_div_limb_in_place_mod_alt(&mut limbs, limb);
            let q_alt = Natural::from_owned_limbs_asc(limbs);
            assert_eq!(q, q_alt);
            assert_eq!(r, r_alt);

            let mut limbs = old_limbs.clone();
            let r_alt = limbs_div_limb_in_place_mod_naive(&mut limbs, limb);
            let q_alt = Natural::from_owned_limbs_asc(limbs);
            assert_eq!(q, q_alt);
            assert_eq!(r, r_alt);
        },
    );
}

#[test]
fn limbs_two_limb_inverse_helper_properties() {
    test_properties(pairs_of_unsigneds_var_2, |&(hi, lo)| {
        let result = limbs_two_limb_inverse_helper(hi, lo);
        verify_limbs_two_limb_inverse_helper(hi, lo, result);
    });
}

#[test]
fn limbs_div_mod_three_limb_by_two_limb_properties() {
    test_properties(
        sextuples_of_limbs_var_1,
        |&(n_2, n_1, n_0, d_1, d_0, inverse)| {
            let (q, r) = limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, inverse);
            verify_limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, q, r);
        },
    );
}

#[test]
fn limbs_div_mod_by_two_limb_normalized_properties() {
    test_properties(triples_of_unsigned_vec_var_37, |(qs_in, ns_in, ds)| {
        let mut qs = qs_in.clone();
        let mut ns = ns_in.clone();
        let q_highest = limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, &ds);
        verify_limbs_div_mod_by_two_limb_normalized(&qs_in, &ns_in, &ds, q_highest, &qs, &ns);
    });
}

#[test]
fn limbs_div_mod_schoolbook_properties() {
    test_properties(
        quadruples_of_three_limb_vecs_and_limb_var_1,
        |(ref qs_in, ref ns_in, ref ds, inverse)| {
            let mut qs = qs_in.clone();
            let mut ns = ns_in.clone();
            let q_highest = _limbs_div_mod_schoolbook(&mut qs, &mut ns, ds, *inverse);
            verify_limbs_div_mod_1(qs_in, ns_in, ds, q_highest, &qs, &ns);
        },
    );
}

#[test]
fn limbs_div_mod_divide_and_conquer_properties() {
    test_properties_custom_scale(
        128,
        quadruples_of_three_limb_vecs_and_limb_var_2,
        |(ref qs_in, ref ns_in, ref ds, inverse)| {
            let mut qs = qs_in.clone();
            let mut ns = ns_in.clone();
            let q_highest = _limbs_div_mod_divide_and_conquer(&mut qs, &mut ns, ds, *inverse);
            verify_limbs_div_mod_1(qs_in, ns_in, ds, q_highest, &qs, &ns);
        },
    );
}

#[test]
fn limbs_invert_basecase_approx_properties() {
    test_properties_custom_scale(
        128,
        triples_of_limb_vec_var_38,
        |(ref is_in, ref ds, ref scratch_in)| {
            let mut is = is_in.clone();
            let mut scratch = scratch_in.clone();
            let result_definitely_exact = _limbs_invert_basecase_approx(&mut is, ds, &mut scratch);
            verify_limbs_invert_approx(is_in, ds, result_definitely_exact, &is);
        },
    );
}

#[test]
fn limbs_invert_newton_approx_properties() {
    test_properties_custom_scale(
        512,
        triples_of_limb_vec_var_39,
        |(ref is_in, ref ds, ref scratch_in)| {
            let mut is = is_in.clone();
            let mut scratch = scratch_in.clone();
            let result_definitely_exact = _limbs_invert_newton_approx(&mut is, ds, &mut scratch);
            verify_limbs_invert_approx(is_in, ds, result_definitely_exact, &is);
        },
    );
}

#[test]
fn limbs_invert_approx_properties() {
    test_properties_custom_scale(
        512,
        triples_of_limb_vec_var_38,
        |(ref is_in, ref ds, ref scratch_in)| {
            let mut is = is_in.clone();
            let mut scratch = scratch_in.clone();
            let result_definitely_exact = _limbs_invert_approx(&mut is, ds, &mut scratch);
            verify_limbs_invert_approx(is_in, ds, result_definitely_exact, &is);
        },
    );
}

#[test]
fn limbs_div_mod_barrett_properties() {
    test_properties_custom_scale(
        512,
        quadruples_of_limb_vec_var_1,
        |(ref qs_in, ref rs_in, ref ns, ref ds)| {
            let mut qs = qs_in.clone();
            let mut rs = rs_in.clone();
            let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
            let q_highest = _limbs_div_mod_barrett(&mut qs, &mut rs, ns, ds, &mut scratch);
            verify_limbs_div_mod_2(qs_in, rs_in, ns, ds, q_highest, &qs, &rs);
        },
    );
}

#[test]
fn limbs_div_mod_properties() {
    test_properties_custom_scale(512, pairs_of_limb_vec_var_9, |(ref ns, ref ds)| {
        let (qs, rs) = limbs_div_mod(ns, ds);
        verify_limbs_div_mod_4(ns, ds, &qs, &rs);
    });
}

#[test]
fn limbs_div_mod_to_out_properties() {
    test_properties_custom_scale(
        512,
        quadruples_of_limb_vec_var_2,
        |(ref qs_in, ref rs_in, ref ns, ref ds)| {
            let mut qs = qs_in.clone();
            let mut rs = rs_in.clone();
            limbs_div_mod_to_out(&mut qs, &mut rs, ns, ds);
            verify_limbs_div_mod_3(qs_in, rs_in, ns, ds, &qs, &rs);
        },
    );
}

fn div_mod_and_div_rem_properties_helper(x: &Natural, y: &Natural) {
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

    let mut q_alt = x.clone();
    let r_alt = q_alt.div_assign_rem(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

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

    let (num_q, num_r) = natural_to_biguint(x).div_mod_floor(&natural_to_biguint(y));
    assert_eq!(biguint_to_natural(&num_q), q);
    assert_eq!(biguint_to_natural(&num_r), r);

    let (num_q, num_r) = natural_to_biguint(x).div_rem(&natural_to_biguint(y));
    assert_eq!(biguint_to_natural(&num_q), q);
    assert_eq!(biguint_to_natural(&num_r), r);

    let (rug_q, rug_r) = natural_to_rug_integer(x).div_rem_floor(natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_q), q);
    assert_eq!(rug_integer_to_natural(&rug_r), r);

    let (rug_q, rug_r) = natural_to_rug_integer(x).div_rem(natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_q), q);
    assert_eq!(rug_integer_to_natural(&rug_r), r);

    assert!(r < *y);
    assert_eq!(q * y + r, *x);
}

#[test]
fn div_mod_and_div_rem_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural,
        |&(ref x, ref y)| {
            div_mod_and_div_rem_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            div_mod_and_div_rem_properties_helper(x, y);
        },
    );

    test_properties(naturals, |x| {
        assert_eq!(x.div_mod(Natural::ONE), (x.clone(), Natural::ZERO));
    });

    test_properties(positive_naturals, |x| {
        assert_eq!(x.div_mod(x), (Natural::ONE, Natural::ZERO));
        assert_eq!(Natural::ZERO.div_mod(x), (Natural::ZERO, Natural::ZERO));
        if *x > 1 {
            assert_eq!(Natural::ONE.div_mod(x), (Natural::ZERO, Natural::ONE));
        }
    });

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb, Limb>,
        |&(x, y)| {
            let (q, r) = x.div_mod(y);
            assert_eq!(
                Natural::from(x).div_mod(Natural::from(y)),
                (Natural::from(q), Natural::from(r))
            );
        },
    );
}

fn ceiling_div_neg_mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    let r = mut_x.ceiling_div_assign_neg_mod(y);
    assert!(mut_x.is_valid());
    assert!(r.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    let r_alt = mut_x.ceiling_div_assign_neg_mod(y.clone());
    let q_alt = mut_x;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.ceiling_div_neg_mod(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.ceiling_div_neg_mod(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().ceiling_div_neg_mod(y);
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().ceiling_div_neg_mod(y.clone());
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);
    assert!(r_alt.is_valid());
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = (x.div_round(y, RoundingMode::Ceiling), x.neg_mod(y));
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);

    let (rug_q, rug_r) =
        rug_ceiling_div_neg_mod(natural_to_rug_integer(x), natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_q), q);
    assert_eq!(rug_integer_to_natural(&rug_r), r);

    assert!(r < *y);
    assert_eq!(q * y - r, *x);
}

#[test]
fn ceiling_div_neg_mod_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        ceiling_div_neg_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            ceiling_div_neg_mod_properties_helper(x, y);
        },
    );

    test_properties(naturals, |x| {
        assert_eq!(
            x.ceiling_div_neg_mod(Natural::ONE),
            (x.clone(), Natural::ZERO)
        );
    });

    test_properties(positive_naturals, |x| {
        assert_eq!(x.ceiling_div_neg_mod(x), (Natural::ONE, Natural::ZERO));
        assert_eq!(
            Natural::ZERO.ceiling_div_neg_mod(x),
            (Natural::ZERO, Natural::ZERO)
        );
        if *x > 1 {
            assert_eq!(
                Natural::ONE.ceiling_div_neg_mod(x),
                (Natural::ONE, x - Natural::ONE)
            );
        }
    });

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb, Limb>,
        |&(x, y)| {
            let (q, r) = x.ceiling_div_neg_mod(y);
            assert_eq!(
                Natural::from(x).ceiling_div_neg_mod(Natural::from(y)),
                (Natural::from(q), Natural::from(r))
            );
        },
    );
}
