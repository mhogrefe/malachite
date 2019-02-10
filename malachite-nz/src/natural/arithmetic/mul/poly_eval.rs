use malachite_base::num::{NotAssign, Parity, PrimitiveInteger, WrappingAddAssign};
use natural::arithmetic::add::{
    _limbs_add_to_out_special, limbs_add_same_length_to_out, limbs_add_to_out,
    limbs_slice_add_greater_in_place_left, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::limbs_add_limb_to_out;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::sub::limbs_sub_same_length_to_out;
use natural::comparison::ord::limbs_cmp_same_length;
use platform::Limb;
use std::cmp::Ordering;

/// Evaluate a degree-3 polynomial in +1 and -1, where each coefficient has width `n` limbs, except
/// the last, which has width `n_high` limbs.
///
/// This is mpn_toom_eval_dgr3_pm1 in mpn/generic/toom_eval_dgr3_pm1.c.
pub(crate) fn _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
    v_1: &mut [Limb],
    v_neg_1: &mut [Limb],
    poly: &[Limb],
    n: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_eq!(v_1.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);

    split_into_chunks!(poly, n, n_high, [poly_0, poly_1, poly_2], poly_3);
    assert!(n_high <= n);

    v_1[n] = if limbs_add_same_length_to_out(v_1, poly_0, poly_2) {
        1
    } else {
        0
    };
    scratch[n] = if limbs_add_to_out(scratch, poly_1, poly_3) {
        1
    } else {
        0
    };
    let v_neg_1_neg = limbs_cmp_same_length(v_1, scratch) == Ordering::Less;
    if v_neg_1_neg {
        limbs_sub_same_length_to_out(v_neg_1, scratch, v_1);
    } else {
        limbs_sub_same_length_to_out(v_neg_1, v_1, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_1, scratch);
    assert!(v_1[n] <= 3);
    assert!(v_neg_1[n] <= 1);
    v_neg_1_neg
}

/// Evaluate a degree-3 polynomial in +2 and -2, where each coefficient has width `n` limbs, except
/// the last, which has width `n_high` limbs.
///
/// Needs n + 1 limbs of temporary storage.
/// This is mpn_toom_eval_dgr3_pm2 from mpn/generic/toom_eval_dg3_pm2.c.
pub(crate) fn _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    poly: &[Limb],
    n: usize,
    scratch: &mut [Limb],
) -> bool {
    split_into_chunks!(poly, n, n_high, [poly_0, poly_1, poly_2], poly_3);
    assert!(n_high <= n);
    assert_eq!(v_2.len(), n + 1);
    {
        let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
        assert_eq!(scratch_init.len(), n);
        // scratch <- (poly_0 + 4 * poly_2) +/- (2 * poly_1 + 8 * poly_3)
        v_2[n] = limbs_shl_to_out(scratch_init, poly_2, 2);
        if limbs_add_same_length_to_out(v_2, scratch_init, poly_0) {
            v_2[n] += 1;
        }
        if n_high < n {
            scratch_init[n_high] = limbs_shl_to_out(scratch_init, poly_3, 2);
            *scratch_last = if _limbs_add_to_out_special(scratch_init, n_high + 1, poly_1) {
                1
            } else {
                0
            };
        } else {
            *scratch_last = limbs_shl_to_out(scratch_init, poly_3, 2);
            if limbs_slice_add_same_length_in_place_left(scratch_init, poly_1) {
                *scratch_last += 1;
            }
        }
    }
    limbs_slice_shl_in_place(scratch, 1);
    let v_neg_2_neg = limbs_cmp_same_length(v_2, scratch) == Ordering::Less;
    if v_neg_2_neg {
        limbs_sub_same_length_to_out(v_neg_2, scratch, v_2);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, v_2, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_2, scratch);
    assert!(v_2[n] < 15);
    assert!(v_neg_2[n] < 10);
    v_neg_2_neg
}

// mpn_toom_eval_pm1 -- Evaluate a polynomial in +1 and -1
// Evaluates a polynomial of degree k > 3, in the points +1 and -1.
//
// This is mpn_toom_eval_pm1 from mpn/generic/toom_eval_pm1.c.
// TODO clean
pub(crate) fn _limbs_mul_toom_evaluate_poly_in_1_and_neg_1(
    xp1: &mut [Limb],
    xm1: &mut [Limb],
    k: usize,
    xp: &[Limb],
    n: usize,
    hn: usize,
    tp: &mut [Limb],
) -> bool {
    assert!(k > 3);
    assert_ne!(hn, 0);
    assert!(hn <= n);
    assert_eq!(tp.len(), n + 1);

    // The degree k is also the number of full-size coefficients, so
    // that last coefficient, of size hn, starts at xp + k*n.
    xp1[n] = if limbs_add_same_length_to_out(xp1, &xp[..n], &xp[2 * n..3 * n]) {
        1
    } else {
        0
    };

    let mut i = 4;
    while i < k {
        assert!(!limbs_slice_add_greater_in_place_left(
            &mut xp1[..n + 1],
            &xp[i * n..(i + 1) * n]
        ));
        i += 2;
    }

    tp[n] = if limbs_add_same_length_to_out(tp, &xp[n..2 * n], &xp[3 * n..4 * n]) {
        1
    } else {
        0
    };
    let mut i = 5;
    while i < k {
        assert!(!limbs_slice_add_greater_in_place_left(
            &mut tp[..n + 1],
            &xp[i * n..(i + 1) * n]
        ));
        i += 2;
    }

    if k & 1 != 0 {
        assert!(!limbs_slice_add_greater_in_place_left(
            &mut tp[..n + 1],
            &xp[k * n..k * n + hn]
        ));
    } else {
        assert!(!limbs_slice_add_greater_in_place_left(
            &mut xp1[..n + 1],
            &xp[k * n..k * n + hn]
        ));
    }

    let neg = limbs_cmp_same_length(&xp1[..n + 1], &tp[..n + 1]) == Ordering::Less;
    if neg {
        limbs_sub_same_length_to_out(xm1, &tp[..n + 1], &xp1[..n + 1]);
    } else {
        limbs_sub_same_length_to_out(xm1, &xp1[..n + 1], &tp[..n + 1]);
    }

    limbs_slice_add_same_length_in_place_left(&mut xp1[..n + 1], &tp[..n + 1]);
    let k = k as Limb;
    assert!(xp1[n] <= k);
    assert!(xm1[n] <= (k >> 1) + 1);
    neg
}

/// Given a `Natural` whose highest limb is `carry` and remaining limbs are `xs`, multiplies the
/// `Natural` by 4 and adds the `Natural` whose limbs are `ys`. The highest limb of the result is
/// written back to `carry` and the remaining limbs are written to `out`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// This is DO_addlsh2 from mpn/generic/toom_eval_pm2.c, with d == `out`, a == `xs`, and b ==
/// `ys`.
fn shl_2_and_add_with_carry_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb], carry: &mut Limb) {
    *carry <<= 2;
    *carry += limbs_shl_to_out(out, xs, 2);
    if limbs_slice_add_same_length_in_place_left(&mut out[..ys.len()], ys) {
        *carry += 1;
    }
}

/// Given a `Natural` whose highest limb is `carry` and remaining limbs are `xs`, multiplies the
/// `Natural` by 4 and adds the `Natural` whose limbs are `ys`. The highest limb of the result is
/// written back to `carry` and the remaining limbs are written to `xs`. `xs` and `ys` must have the
/// same length.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ys.len()`
///
/// This is DO_addlsh2 from mpn/generic/toom_eval_pm2.c, with d == b == `ys` and a == `xs`.
fn shl_2_and_add_with_carry_in_place_left(xs: &mut [Limb], ys: &[Limb], carry: &mut Limb) {
    *carry <<= 2;
    *carry += limbs_slice_shl_in_place(xs, 2);
    if limbs_slice_add_same_length_in_place_left(xs, ys) {
        *carry += 1;
    }
}

// Evaluates a polynomial of degree 3 < `degree` < `Limb::WIDTH`, in the points +2 and -2, where
// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
//
// This is mpn_toom_eval_pm2 from mpn/generic/toom_eval_pm2.c.
pub(crate) fn _limbs_mul_toom_evaluate_poly_in_2_and_neg_2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    degree: usize,
    poly: &[Limb],
    n: usize,
    scratch: &mut [Limb],
) -> bool {
    assert!(degree >= 3);
    assert!(degree < Limb::WIDTH as usize);
    assert_eq!(v_2.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);

    // The degree `degree` is also the number of full-size coefficients, so that last coefficient,
    // of size `n_high`, starts at `poly[degree * n..]`.
    let coefficients: Vec<&[Limb]> = poly.chunks(n).collect();
    assert_eq!(coefficients.len(), degree + 1);
    let n_high = coefficients[degree].len();
    {
        let (v_2_last, v_2_init) = v_2.split_last_mut().unwrap();
        let mut carry = 0;
        shl_2_and_add_with_carry_to_out(
            v_2_init,
            coefficients[degree],
            &coefficients[degree - 2][..n_high],
            &mut carry,
        );
        if n_high != n {
            carry = if limbs_add_limb_to_out(
                &mut v_2_init[n_high..],
                &coefficients[degree - 2][n_high..],
                carry,
            ) {
                1
            } else {
                0
            };
        }
        if degree >= 4 {
            let mut i = degree - 4;
            loop {
                shl_2_and_add_with_carry_in_place_left(v_2_init, coefficients[i], &mut carry);
                if i < 2 {
                    break;
                }
                i -= 2;
            }
        }
        *v_2_last = carry;
    }
    {
        let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
        let mut carry = 0;
        shl_2_and_add_with_carry_to_out(
            scratch_init,
            coefficients[degree - 1],
            coefficients[degree - 3],
            &mut carry,
        );
        if degree >= 5 {
            let mut i = degree - 5;
            loop {
                shl_2_and_add_with_carry_in_place_left(scratch_init, coefficients[i], &mut carry);
                if i < 2 {
                    break;
                }
                i -= 2;
            }
        }
        *scratch_last = carry;
    }
    if degree.even() {
        assert_eq!(limbs_slice_shl_in_place(scratch, 1), 0);
    } else {
        assert_eq!(limbs_slice_shl_in_place(v_2, 1), 0);
    }
    let mut v_neg_2_neg = limbs_cmp_same_length(v_2, scratch) == Ordering::Less;
    if v_neg_2_neg {
        limbs_sub_same_length_to_out(v_neg_2, scratch, v_2);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, v_2, scratch);
    }
    if degree.odd() {
        v_neg_2_neg.not_assign();
    }
    limbs_slice_add_same_length_in_place_left(v_2, scratch);
    let mut shift = 1 << (degree + 1);
    if shift != 0 {
        assert!(v_2[n] < shift - 1);
    }
    shift <<= 1;
    if shift != 0 {
        assert!(v_neg_2[n] < shift / 3);
    }
    v_neg_2_neg
}

/// Evaluates a polynomial of degree `degree` > 2, in the points 2 ^ `shift` and -2 ^ `shift`, where
/// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
///
/// This is mpn_toom_eval_pm2exp from mpn/generic/toom_eval_pm2exp.c.
pub(crate) fn _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(
    v_2_pow: &mut [Limb],
    v_neg_2_pow: &mut [Limb],
    degree: usize,
    poly: &[Limb],
    n: usize,
    shift: u32,
    scratch: &mut [Limb],
) -> bool {
    assert!(degree >= 3);
    assert!(shift * (degree as u32) < Limb::WIDTH);
    assert_eq!(scratch.len(), n + 1);
    assert_eq!(v_2_pow.len(), n + 1);
    let coefficients: Vec<&[Limb]> = poly.chunks(n).collect();
    assert_eq!(coefficients.len(), degree + 1);
    let n_high = coefficients[degree].len();
    {
        let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
        let (v_2_pow_last, v_2_pow_init) = v_2_pow.split_last_mut().unwrap();
        // The degree `degree` is also the number of full-size coefficients, so that the last
        // coefficient, of size `n_high`, starts at `poly + degree * n`.
        *v_2_pow_last = limbs_shl_to_out(scratch_init, coefficients[2], shift << 1);
        if limbs_add_same_length_to_out(v_2_pow_init, coefficients[0], scratch_init) {
            v_2_pow_last.wrapping_add_assign(1);
        }
        let mut i = 4;
        while i < degree {
            v_2_pow_last.wrapping_add_assign(limbs_shl_to_out(
                scratch_init,
                coefficients[i],
                (i as u32) * shift,
            ));
            if limbs_slice_add_same_length_in_place_left(v_2_pow_init, scratch_init) {
                v_2_pow_last.wrapping_add_assign(1);
            }
            i += 2;
        }

        *scratch_last = limbs_shl_to_out(scratch_init, coefficients[1], shift);
        let mut i = 3;
        while i < degree {
            *scratch_last += limbs_shl_to_out(v_neg_2_pow, coefficients[i], (i as u32) * shift);
            if limbs_slice_add_same_length_in_place_left(scratch_init, &v_neg_2_pow[..n]) {
                scratch_last.wrapping_add_assign(1);
            }
            i += 2;
        }
    }

    v_neg_2_pow[n_high] =
        limbs_shl_to_out(v_neg_2_pow, coefficients[degree], degree as u32 * shift);
    if degree.even() {
        limbs_slice_add_greater_in_place_left(v_2_pow, &v_neg_2_pow[..n_high + 1]);
    } else {
        limbs_slice_add_greater_in_place_left(scratch, &v_neg_2_pow[..n_high + 1]);
    }
    let v_neg_2_pow_neg = limbs_cmp_same_length(v_2_pow, scratch) == Ordering::Less;
    if v_neg_2_pow_neg {
        limbs_sub_same_length_to_out(v_neg_2_pow, scratch, v_2_pow);
    } else {
        limbs_sub_same_length_to_out(v_neg_2_pow, v_2_pow, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_2_pow, scratch);
    v_neg_2_pow_neg
}

// This is DO_mpn_addlsh_n from mpn/generic/toom_eval_pm2rexp.c.
pub(crate) fn do_mpn_addlsh_n(dst: &mut [Limb], src: &[Limb], s: usize, ws: &mut [Limb]) -> Limb {
    let n = src.len();
    let cy = limbs_shl_to_out(ws, src, s as u32);
    cy + if limbs_slice_add_same_length_in_place_left(&mut dst[..n], &mut ws[..n]) {
        1
    } else {
        0
    }
}

/// Evaluate a polynomial in +2^-k and -2^-k
/// Evaluates a polynomial of degree k >= 3.
/// This is mpn_toom_eval_pm2rexp from mpn/generic/toom_eval_pm2rexp.c.
pub fn _limbs_mul_toom_evaluate_poly_in_2_neg_pow_and_neg_2_neg_pow(
    rp: &mut [Limb],
    rm: &mut [Limb],
    q: usize,
    ap: &[Limb],
    n: usize,
    t: usize,
    s: usize,
    ws: &mut [Limb],
) -> bool {
    // {ap,q*n+t} -> {rp,n+1} {rm,n+1} , with {ws, n+1}
    assert!(n >= t);
    assert_ne!(s, 0); // or _eval_pm1 should be used
    assert!(q > 1);
    assert!(s * q < Limb::WIDTH as usize);
    rp[n] = limbs_shl_to_out(rp, &ap[..n], (s * q) as u32);
    ws[n] = limbs_shl_to_out(ws, &ap[n..2 * n], (s * (q - 1)) as u32);
    if (q & 1) != 0 {
        assert!(!limbs_slice_add_greater_in_place_left(
            &mut ws[..n + 1],
            &ap[n * q..n * q + t]
        ));
        let carry = do_mpn_addlsh_n(rp, &ap[n * (q - 1)..n * q], s, rm);
        rp[n].wrapping_add_assign(carry);
    } else {
        assert!(!limbs_slice_add_greater_in_place_left(
            &mut rp[..n + 1],
            &ap[n * q..n * q + t]
        ));
    }
    let mut i = 2;
    while i < q - 1 {
        let carry = do_mpn_addlsh_n(rp, &ap[n * i..n * (i + 1)], s * (q - i), rm);
        rp[n].wrapping_add_assign(carry);
        i += 1;
        let carry = do_mpn_addlsh_n(ws, &ap[n * i..n * (i + 1)], s * (q - i), rm);
        ws[n].wrapping_add_assign(carry);
        i += 1;
    }

    let neg = limbs_cmp_same_length(&rp[..n + 1], &ws[..n + 1]) == Ordering::Less;
    if neg {
        limbs_sub_same_length_to_out(rm, &ws[..n + 1], &rp[..n + 1]);
    } else {
        limbs_sub_same_length_to_out(rm, &rp[..n + 1], &ws[..n + 1]);
    }
    assert!(!limbs_slice_add_same_length_in_place_left(
        &mut rp[..n + 1],
        &ws[..n + 1]
    ));
    neg
}
