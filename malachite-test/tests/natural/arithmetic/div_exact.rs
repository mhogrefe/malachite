use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, DivMod, DivRound, EqModPowerOfTwo, ModPowerOfTwo,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_div_exact_limb_in_place_no_special_3, _limbs_div_exact_limb_no_special_3,
    _limbs_div_exact_limb_to_out_no_special_3, _limbs_modular_div, _limbs_modular_div_barrett,
    _limbs_modular_div_barrett_scratch_len, _limbs_modular_div_divide_and_conquer,
    _limbs_modular_div_mod_barrett, _limbs_modular_div_mod_barrett_scratch_len,
    _limbs_modular_div_mod_divide_and_conquer, _limbs_modular_div_mod_schoolbook,
    _limbs_modular_div_ref, _limbs_modular_div_ref_scratch_len, _limbs_modular_div_schoolbook,
    _limbs_modular_div_scratch_len, limbs_div_exact, limbs_div_exact_3, limbs_div_exact_3_in_place,
    limbs_div_exact_3_to_out, limbs_div_exact_limb, limbs_div_exact_limb_in_place,
    limbs_div_exact_limb_to_out, limbs_div_exact_to_out, limbs_div_exact_to_out_ref_ref,
    limbs_div_exact_to_out_ref_val, limbs_div_exact_to_out_val_ref, limbs_modular_invert,
    limbs_modular_invert_limb, limbs_modular_invert_scratch_len,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_nz_test_util::natural::arithmetic::div_exact::{
    limbs_div_exact_3_in_place_alt, limbs_div_exact_3_to_out_alt,
};

use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::base::{
    odd_limbs, pairs_of_limb_vec_and_positive_limb_var_2, pairs_of_limb_vec_var_8,
    pairs_of_unsigned_vec_var_12, pairs_of_unsigneds_var_7, quadruples_of_limb_vec_var_4,
    quadruples_of_three_limb_vecs_and_limb_var_3, quadruples_of_three_limb_vecs_and_limb_var_4,
    quadruples_of_three_limb_vecs_and_limb_var_5, quadruples_of_three_limb_vecs_and_limb_var_6,
    triples_of_limb_vec_limb_vec_and_positive_limb_var_2, triples_of_limb_vec_var_50,
    triples_of_limb_vec_var_51, triples_of_limb_vec_var_53, vecs_of_limb_var_4,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_natural_var_1, positive_naturals,
};

fn verify_limbs_modular_invert(ds: &[Limb], is: &[Limb]) {
    let d = Natural::from_limbs_asc(ds);
    let i = Natural::from_limbs_asc(is);
    let pow = u64::wrapping_from(ds.len()) << Limb::LOG_WIDTH;
    assert!((d * i).eq_mod_power_of_two(&Natural::ONE, pow));
}

fn verify_limbs_modular_div_mod(ns: &[Limb], ds: &[Limb], borrow: bool, qs: &[Limb], rs: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let q = Natural::from_limbs_asc(qs);
    let r = Natural::from_limbs_asc(rs);
    let n_len = ns.len();
    let d_len = ds.len();
    let q_len = n_len - d_len;
    let qd = q * d;
    assert_eq!(n < qd, borrow);
    assert!(qd.eq_mod_power_of_two(&n, u64::wrapping_from(q_len) << Limb::LOG_WIDTH));
    let expected_r = (Integer::from(n) - Integer::from(qd))
        .mod_power_of_two(u64::wrapping_from(n_len) << Limb::LOG_WIDTH)
        >> (u64::wrapping_from(q_len) << Limb::LOG_WIDTH);
    assert_eq!(expected_r, r);
}

fn verify_limbs_modular_div(ns: &[Limb], ds: &[Limb], qs: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let q = Natural::from_limbs_asc(qs);
    assert_eq!(
        (q * d).mod_power_of_two(u64::wrapping_from(ns.len()) << Limb::LOG_WIDTH),
        n
    );
}

fn verify_limbs_div_exact(ns: &[Limb], ds: &[Limb], qs: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let expected_q = Natural::from_limbs_asc(qs);
    let (q, r) = n.div_mod(d);
    assert_eq!(q, expected_q);
    assert_eq!(r, 0);
}

#[test]
fn limbs_modular_invert_limb_properties() {
    test_properties(odd_limbs, |&limb| {
        let inverse = limbs_modular_invert_limb(limb);
        assert_eq!(limb.wrapping_mul(inverse), 1);
        assert_eq!(limbs_modular_invert_limb(inverse), limb);
    });
}

#[test]
fn limbs_div_exact_limb_properties() {
    test_properties(
        pairs_of_limb_vec_and_positive_limb_var_2,
        |&(ref limbs, limb)| {
            let expected_result = Natural::from_limbs_asc(limbs).div_exact(Natural::from(limb));
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_div_exact_limb(limbs, limb)),
                expected_result
            );
            assert_eq!(
                Natural::from_owned_limbs_asc(_limbs_div_exact_limb_no_special_3(limbs, limb)),
                expected_result
            );
        },
    );
}

#[test]
fn limbs_div_exact_limb_to_out_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_positive_limb_var_2,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_div_exact_limb_to_out(&mut out, in_limbs, limb);
            let len = in_limbs.len();
            let expected_result = Natural::from_limbs_asc(in_limbs).div_exact(Natural::from(limb));
            assert_eq!(Natural::from_limbs_asc(&out[..len]), expected_result);
            assert_eq!(&out[len..], &old_out[len..]);

            let mut out = old_out.to_vec();
            _limbs_div_exact_limb_to_out_no_special_3(&mut out, in_limbs, limb);
            let len = in_limbs.len();
            let expected_result = Natural::from_limbs_asc(in_limbs).div_exact(Natural::from(limb));
            assert_eq!(Natural::from_limbs_asc(&out[..len]), expected_result);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_div_exact_limb_in_place_properties() {
    test_properties(
        pairs_of_limb_vec_and_positive_limb_var_2,
        |&(ref limbs, limb)| {
            let old_limbs = limbs;
            let mut limbs = old_limbs.to_vec();
            limbs_div_exact_limb_in_place(&mut limbs, limb);
            let expected_result =
                Natural::from_limbs_asc(&old_limbs).div_exact(Natural::from(limb));
            assert_eq!(Natural::from_owned_limbs_asc(limbs), expected_result);

            let mut limbs = old_limbs.to_vec();
            _limbs_div_exact_limb_in_place_no_special_3(&mut limbs, limb);
            let expected_result =
                Natural::from_limbs_asc(&old_limbs).div_exact(Natural::from(limb));
            assert_eq!(Natural::from_owned_limbs_asc(limbs), expected_result);
        },
    );
}

#[test]
fn limbs_div_exact_3_properties() {
    test_properties(vecs_of_limb_var_4, |ref limbs| {
        let q_limbs = Natural::from_owned_limbs_asc(limbs_div_exact_3(limbs));
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_div_exact_limb(limbs, 3)),
            q_limbs,
        );
        assert_eq!(
            Natural::from_limbs_asc(limbs).div_exact(Natural::from(3u32)),
            q_limbs
        );
    });
}

#[test]
fn limbs_div_exact_3_to_out_properties() {
    test_properties(pairs_of_limb_vec_var_8, |&(ref out, ref in_limbs)| {
        let mut out = out.to_vec();
        let old_out = out.clone();
        limbs_div_exact_3_to_out(&mut out, in_limbs);
        let len = in_limbs.len();
        assert_eq!(
            Natural::from_limbs_asc(&out[..len]),
            Natural::from_limbs_asc(in_limbs).div_exact(Natural::from(3u32))
        );
        assert_eq!(&out[len..], &old_out[len..]);

        let mut out_alt = old_out.clone();
        limbs_div_exact_limb_to_out(&mut out_alt, in_limbs, 3);
        assert_eq!(out_alt, out);

        let mut out_alt = old_out.clone();
        limbs_div_exact_3_to_out_alt(&mut out_alt, in_limbs);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn limbs_div_exact_3_in_place_properties() {
    test_properties(vecs_of_limb_var_4, |ref limbs| {
        let old_limbs = limbs;
        let mut limbs = old_limbs.to_vec();
        limbs_div_exact_3_in_place(&mut limbs);
        assert_eq!(
            Natural::from_limbs_asc(&limbs),
            Natural::from_limbs_asc(&old_limbs).div_exact(Natural::from(3u32))
        );

        let mut limbs_alt = old_limbs.to_vec();
        limbs_div_exact_limb_in_place(&mut limbs_alt, 3);
        assert_eq!(limbs_alt, limbs);

        let mut limbs_alt = old_limbs.to_vec();
        limbs_div_exact_3_in_place_alt(&mut limbs_alt);
        assert_eq!(limbs_alt, limbs);
    });
}

#[test]
fn limbs_modular_invert_properties() {
    test_properties_custom_scale(512, pairs_of_unsigned_vec_var_12, |&(ref is, ref ds)| {
        let mut is = is.to_vec();
        let n = ds.len();
        let mut scratch = vec![0; limbs_modular_invert_scratch_len(n)];
        limbs_modular_invert(&mut is, ds, &mut scratch);
        verify_limbs_modular_invert(ds, &is[..n]);
    });
}

#[test]
fn limbs_modular_div_mod_schoolbook_properties() {
    test_properties(
        quadruples_of_three_limb_vecs_and_limb_var_4,
        |&(ref qs, ref ns, ref ds, inverse)| {
            let ns_old = ns;
            let mut qs = qs.to_vec();
            let mut ns = ns.to_vec();
            let borrow = _limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, ds, inverse);
            let q_len = ns.len() - ds.len();
            verify_limbs_modular_div_mod(ns_old, ds, borrow, &qs[..q_len], &ns[q_len..]);
        },
    );
}

#[test]
fn limbs_modular_div_mod_divide_and_conquer_properties() {
    test_properties_custom_scale(
        128,
        quadruples_of_three_limb_vecs_and_limb_var_5,
        |&(ref qs, ref ns, ref ds, inverse)| {
            let ns_old = ns;
            let mut qs = qs.to_vec();
            let mut ns = ns.to_vec();
            let borrow = _limbs_modular_div_mod_divide_and_conquer(&mut qs, &mut ns, ds, inverse);
            let q_len = ns.len() - ds.len();
            verify_limbs_modular_div_mod(ns_old, ds, borrow, &qs[..q_len], &ns[q_len..]);
        },
    );
}

#[test]
fn limbs_modular_div_mod_barrett_properties() {
    test_properties_custom_scale(
        512,
        quadruples_of_limb_vec_var_4,
        |&(ref qs, ref rs, ref ns, ref ds)| {
            let mut qs = qs.to_vec();
            let mut rs = rs.to_vec();
            let mut scratch =
                vec![0; _limbs_modular_div_mod_barrett_scratch_len(ns.len(), ds.len())];
            let borrow = _limbs_modular_div_mod_barrett(&mut qs, &mut rs, ns, ds, &mut scratch);
            let q_len = ns.len() - ds.len();
            verify_limbs_modular_div_mod(ns, ds, borrow, &qs[..q_len], &rs[..ds.len()]);
        },
    );
}

#[test]
fn limbs_modular_div_schoolbook_properties() {
    test_properties(
        quadruples_of_three_limb_vecs_and_limb_var_3,
        |&(ref qs, ref ns, ref ds, inverse)| {
            let ns_old = ns;
            let mut qs = qs.to_vec();
            let mut ns = ns.to_vec();
            _limbs_modular_div_schoolbook(&mut qs, &mut ns, ds, inverse);
            verify_limbs_modular_div(ns_old, ds, &qs);
        },
    );
}

#[test]
fn limbs_modular_div_divide_and_conquer_properties() {
    test_properties_custom_scale(
        512,
        quadruples_of_three_limb_vecs_and_limb_var_6,
        |&(ref qs, ref ns, ref ds, inverse)| {
            let ns_old = ns;
            let mut qs = qs.to_vec();
            let mut ns = ns.to_vec();
            _limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, ds, inverse);
            verify_limbs_modular_div(ns_old, ds, &qs);
        },
    );
}

#[test]
fn limbs_modular_div_barrett_properties() {
    test_properties_custom_scale(
        512,
        triples_of_limb_vec_var_50,
        |&(ref qs, ref ns, ref ds)| {
            let mut qs = qs.to_vec();
            let mut scratch = vec![0; _limbs_modular_div_barrett_scratch_len(ns.len(), ds.len())];
            _limbs_modular_div_barrett(&mut qs, ns, ds, &mut scratch);
            verify_limbs_modular_div(ns, ds, &qs[..ns.len()]);
        },
    );
}

#[test]
fn limbs_modular_div_properties() {
    test_properties_custom_scale(
        512,
        triples_of_limb_vec_var_51,
        |&(ref qs, ref ns, ref ds)| {
            let qs_old = qs;
            let mut qs = qs_old.to_vec();
            let mut mut_ns = ns.to_vec();
            let mut scratch = vec![0; _limbs_modular_div_scratch_len(ns.len(), ds.len())];
            _limbs_modular_div(&mut qs, &mut mut_ns, ds, &mut scratch);
            let result = qs;

            let mut qs = qs_old.to_vec();
            let mut scratch = vec![0; _limbs_modular_div_ref_scratch_len(ns.len(), ds.len())];
            _limbs_modular_div_ref(&mut qs, ns, ds, &mut scratch);
            assert_eq!(qs, result);

            verify_limbs_modular_div(ns, ds, &qs[..ns.len()]);
        },
    );
}

#[test]
fn limbs_div_exact_properties() {
    test_properties(triples_of_limb_vec_var_53, |&(ref qs, ref ns, ref ds)| {
        let qs_old = qs;
        let mut qs = qs_old.to_vec();
        let mut mut_ns = ns.to_vec();
        let mut mut_ds = ds.to_vec();
        limbs_div_exact_to_out(&mut qs, &mut mut_ns, &mut mut_ds);
        let result = qs;

        let mut qs = qs_old.to_vec();
        let mut mut_ns = ns.to_vec();
        limbs_div_exact_to_out_val_ref(&mut qs, &mut mut_ns, ds);
        assert_eq!(qs, result);

        let mut qs = qs_old.to_vec();
        let mut mut_ds = ds.to_vec();
        limbs_div_exact_to_out_ref_val(&mut qs, ns, &mut mut_ds);
        assert_eq!(qs, result);

        let mut qs = qs_old.to_vec();
        limbs_div_exact_to_out_ref_ref(&mut qs, ns, ds);
        assert_eq!(qs, result);

        let q_len = ns.len() - ds.len() + 1;
        let qs = limbs_div_exact(ns, ds);
        assert_eq!(qs, &result[..q_len]);

        verify_limbs_div_exact(ns, ds, &qs[..q_len]);
    });
}

#[test]
fn div_exact_properties() {
    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            let mut mut_x = x.clone();
            mut_x.div_exact_assign(y);
            assert!(mut_x.is_valid());
            let q = mut_x;

            let mut mut_x = x.clone();
            mut_x.div_exact_assign(y.clone());
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, q);

            let q_alt = x.div_exact(y);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.div_exact(y.clone());
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.clone().div_exact(y);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.clone().div_exact(y.clone());
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.div_round(y, RoundingMode::Exact);
            assert_eq!(q_alt, q);

            assert_eq!(
                rug_integer_to_natural(
                    &natural_to_rug_integer(x).div_exact(&natural_to_rug_integer(y))
                ),
                q
            );

            assert_eq!(q * y, *x);
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.div_exact(Natural::ONE), *n);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(Natural::ZERO.div_exact(n), 0);
        assert_eq!(n.div_exact(n), 1);
    });

    test_properties(pairs_of_unsigneds_var_7::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x).div_exact(Natural::from(y)), x.div_exact(y));
    });
}
