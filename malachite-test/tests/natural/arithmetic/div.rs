use malachite_base::num::arithmetic::traits::DivMod;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::div::{
    limbs_div, limbs_div_barrett, limbs_div_barrett_approx, limbs_div_barrett_approx_scratch_len,
    limbs_div_barrett_scratch_len, limbs_div_divide_and_conquer,
    limbs_div_divide_and_conquer_approx, limbs_div_divisor_of_limb_max_with_carry_in_place,
    limbs_div_divisor_of_limb_max_with_carry_to_out, limbs_div_limb, limbs_div_limb_in_place,
    limbs_div_limb_to_out, limbs_div_schoolbook, limbs_div_schoolbook_approx, limbs_div_to_out,
    limbs_div_to_out_ref_ref, limbs_div_to_out_ref_val, limbs_div_to_out_val_ref,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::natural::arithmetic::div::{
    limbs_div_limb_in_place_alt, limbs_div_limb_to_out_alt,
};

use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3,
    quadruples_of_three_limb_vecs_and_limb_var_1, quadruples_of_three_limb_vecs_and_limb_var_2,
    triples_of_limb_vec_limb_and_limb_var_1, triples_of_limb_vec_var_41,
    triples_of_limb_vec_var_42, triples_of_limb_vec_var_43,
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_1,
    positive_naturals,
};

fn verify_limbs_div(qs_in: &[Limb], ns: &[Limb], ds: &[Limb], q_highest: bool, qs_out: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let expected_q = &n / &d;
    let base_q_len = ns.len() - ds.len();
    let mut qs = qs_out[..base_q_len].to_vec();
    if q_highest {
        qs.push(1);
    }
    let q = Natural::from_owned_limbs_asc(qs);
    assert_eq!(q, expected_q);
    assert_eq!(&qs_in[base_q_len..], &qs_out[base_q_len..]);
    let r = n - q * &d;
    assert!(r < d);
}

fn verify_limbs_div_2(qs_in: &[Limb], ns: &[Limb], ds: &[Limb], qs_out: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let expected_q = &n / &d;
    let base_q_len = ns.len() - ds.len() + 1;
    let q = Natural::from_limbs_asc(&qs_out[..base_q_len]);
    assert_eq!(q, expected_q);
    assert_eq!(&qs_in[base_q_len..], &qs_out[base_q_len..]);
    let r = n - q * &d;
    assert!(r < d);
}

fn verify_limbs_div_approx(
    qs_in: &[Limb],
    ns_in: &[Limb],
    ds: &[Limb],
    q_highest: bool,
    qs_out: &[Limb],
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
    let q_is_one_too_large = q != expected_q;
    if q_is_one_too_large {
        assert_eq!(q, expected_q + Natural::ONE);
        assert_eq!((q - Natural::ONE) * &d + expected_r, n);
    } else {
        assert_eq!(q * &d + expected_r, n);
    }
    assert_eq!(&qs_in[base_q_len..], &qs_out[base_q_len..]);
}

fn verify_limbs_div_approx_2(
    qs_in: &[Limb],
    ns: &[Limb],
    ds: &[Limb],
    q_highest: bool,
    qs_out: &[Limb],
) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let (expected_q, expected_r) = (&n).div_mod(&d);
    let base_q_len = ns.len() - ds.len();
    let mut qs = qs_out[..base_q_len].to_vec();
    if q_highest {
        qs.push(1);
    }
    let q = Natural::from_owned_limbs_asc(qs);
    let q_is_too_large = q != expected_q;
    if q_is_too_large {
        assert!(q > expected_q);
        assert!(q - &expected_q <= 4);
        assert_eq!(expected_q * &d + expected_r, n);
    } else {
        assert_eq!(q * &d + expected_r, n);
    }
    assert_eq!(&qs_in[base_q_len..], &qs_out[base_q_len..]);
}

#[test]
fn limbs_div_limb_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_div_limb(limbs, limb)),
                Natural::from_limbs_asc(limbs) / Natural::from(limb)
            );
        },
    );
}

#[test]
fn limbs_div_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_div_limb_to_out(&mut out, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                Natural::from_limbs_asc(&out[..len]),
                Natural::from_limbs_asc(in_limbs) / Natural::from(limb)
            );
            assert_eq!(&out[len..], &old_out[len..]);

            let mut out_alt = old_out.clone();
            limbs_div_limb_to_out_alt(&mut out_alt, in_limbs, limb);
            assert_eq!(out, out_alt);
        },
    );
}

#[test]
fn limbs_div_limb_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_div_limb_in_place(&mut limbs, limb);
            let out = limbs.clone();
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs),
                Natural::from_limbs_asc(&old_limbs) / Natural::from(limb)
            );

            let mut limbs = old_limbs.clone();
            limbs_div_limb_in_place_alt(&mut limbs, limb);
            assert_eq!(limbs, out);
        },
    );
}

#[test]
fn limbs_div_divisor_of_limb_max_with_carry_to_out_properties() {
    test_properties(
        quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3,
        |&(ref out, ref xs, divisor, carry)| {
            //TODO figure out what to test
            let mut out = out.to_vec();
            limbs_div_divisor_of_limb_max_with_carry_to_out(&mut out, xs, divisor, carry);
        },
    );
}

#[test]
fn limbs_div_divisor_of_limb_max_with_carry_in_place_properties() {
    test_properties(
        triples_of_limb_vec_limb_and_limb_var_1,
        |&(ref xs, divisor, carry)| {
            //TODO figure out what to test
            let mut xs = xs.to_vec();
            limbs_div_divisor_of_limb_max_with_carry_in_place(&mut xs, divisor, carry);
        },
    );
}

#[test]
fn limbs_div_schoolbook_properties() {
    test_properties(
        quadruples_of_three_limb_vecs_and_limb_var_1,
        |(ref qs_in, ref ns_in, ref ds, inverse)| {
            let mut qs = qs_in.clone();
            let mut ns = ns_in.clone();
            let q_highest = limbs_div_schoolbook(&mut qs, &mut ns, ds, *inverse);
            verify_limbs_div(qs_in, ns_in, ds, q_highest, &qs);
        },
    );
}

#[test]
fn limbs_div_divide_and_conquer_properties() {
    test_properties(
        quadruples_of_three_limb_vecs_and_limb_var_2,
        |(ref qs_in, ref ns_in, ref ds, inverse)| {
            let mut qs = qs_in.clone();
            let mut ns = ns_in.clone();
            let q_highest = limbs_div_divide_and_conquer(&mut qs, &mut ns, ds, *inverse);
            verify_limbs_div(qs_in, ns_in, ds, q_highest, &qs);
        },
    );
}

#[test]
fn limbs_div_barrett_properties() {
    test_properties_custom_scale(
        512,
        triples_of_limb_vec_var_42,
        |(ref qs_in, ref ns, ref ds)| {
            let mut qs = qs_in.clone();
            let mut scratch = vec![0; limbs_div_barrett_scratch_len(ns.len(), ds.len())];
            let q_highest = limbs_div_barrett(&mut qs, &ns, ds, &mut scratch);
            verify_limbs_div(qs_in, ns, ds, q_highest, &qs);
        },
    );
}

#[test]
fn limbs_div_schoolbook_approx_properties() {
    test_properties(
        quadruples_of_three_limb_vecs_and_limb_var_1,
        |(ref qs_in, ref ns_in, ref ds, inverse)| {
            let mut qs = qs_in.clone();
            let mut ns = ns_in.clone();
            let q_highest = limbs_div_schoolbook_approx(&mut qs, &mut ns, ds, *inverse);
            verify_limbs_div_approx(qs_in, ns_in, ds, q_highest, &qs);
        },
    );
}

#[test]
fn limbs_div_divide_and_conquer_approx_properties() {
    test_properties_custom_scale(
        128,
        quadruples_of_three_limb_vecs_and_limb_var_2,
        |(ref qs_in, ref ns_in, ref ds, inverse)| {
            let mut qs = qs_in.clone();
            let mut ns = ns_in.clone();
            let q_highest = limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, ds, *inverse);
            verify_limbs_div_approx(qs_in, ns_in, ds, q_highest, &qs);
        },
    );
}

#[test]
fn limbs_div_barrett_approx_properties() {
    test_properties_custom_scale(
        512,
        triples_of_limb_vec_var_41,
        |(ref qs_in, ref ns, ref ds)| {
            let mut qs = qs_in.clone();
            let mut scratch = vec![0; limbs_div_barrett_approx_scratch_len(ns.len(), ds.len())];
            let q_highest = limbs_div_barrett_approx(&mut qs, &ns, ds, &mut scratch);
            verify_limbs_div_approx_2(qs_in, ns, ds, q_highest, &qs);
        },
    );
}

#[test]
fn limbs_div_to_out_properties() {
    test_properties_custom_scale(
        512,
        triples_of_limb_vec_var_43,
        |(ref qs_in, ref ns, ref ds)| {
            let mut qs = qs_in.clone();
            limbs_div_to_out_ref_ref(&mut qs, ns, ds);
            verify_limbs_div_2(qs_in, ns, ds, &qs);
            let qs_out = qs;

            let mut qs = qs_in.clone();
            let mut ns_cloned = ns.clone();
            let mut ds_cloned = ds.clone();
            limbs_div_to_out(&mut qs, &mut ns_cloned, &mut ds_cloned);
            assert_eq!(qs, qs_out);

            let mut qs = qs_in.clone();
            let mut ns_cloned = ns.clone();
            limbs_div_to_out_val_ref(&mut qs, &mut ns_cloned, ds);
            assert_eq!(qs, qs_out);

            let mut qs = qs_in.clone();
            let mut ds_cloned = ds.clone();
            limbs_div_to_out_ref_val(&mut qs, ns, &mut ds_cloned);
            assert_eq!(qs, qs_out);

            let qs = limbs_div(ns, ds);
            let qs: &[Limb] = &qs;
            assert_eq!(&qs_out[..qs.len()], qs);
        },
    );
}

fn div_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    mut_x /= y;
    assert!(mut_x.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    mut_x /= y.clone();
    let q_alt = mut_x;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = x / y;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = x / y.clone();
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = x.clone() / y;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = x.clone() / y.clone();
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = x.div_mod(y).0;
    assert_eq!(q_alt, q);

    let num_q = natural_to_biguint(x) / &natural_to_biguint(y);
    assert_eq!(biguint_to_natural(&num_q), q);

    let rug_q = natural_to_rug_integer(x) / natural_to_rug_integer(y);
    assert_eq!(rug_integer_to_natural(&rug_q), q);

    let r = x - &q * y;
    assert!(r < *y);
    assert_eq!(q * y + r, *x);
}

#[test]
fn div_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural,
        |&(ref x, ref y)| {
            div_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            div_properties_helper(x, y);
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n / Natural::ONE, *n);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(n / n, 1);
        assert_eq!(Natural::ZERO / n, Natural::ZERO);
        if *n > 1 {
            assert_eq!(Natural::ONE / n, Natural::ZERO);
        }
    });
}
