use malachite_base::num::{
    NotAssign, Parity, PrimitiveInteger, WrappingAddAssign, WrappingSubAssign,
};
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
    n_high: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_ne!(n_high, 0);
    assert!(n_high <= n);
    assert_eq!(v_1.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);

    split_into_chunks!(poly, n, n_high_alt, [poly_0, poly_1, poly_2], poly_3);
    assert_eq!(n_high_alt, n_high);

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
    high_n: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_ne!(high_n, 0);
    assert!(high_n <= n);
    split_into_chunks!(poly, n, n_high_alt, [poly_0, poly_1, poly_2], poly_3);
    assert_eq!(n_high_alt, high_n);
    assert_eq!(v_2.len(), n + 1);
    {
        let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
        assert_eq!(scratch_init.len(), n);
        // scratch <- (poly_0 + 4 * poly_2) +/- (2 * poly_1 + 8 * poly_3)
        v_2[n] = limbs_shl_to_out(scratch_init, poly_2, 2);
        if limbs_add_same_length_to_out(v_2, scratch_init, poly_0) {
            v_2[n] += 1;
        }
        if high_n < n {
            scratch_init[high_n] = limbs_shl_to_out(scratch_init, poly_3, 2);
            *scratch_last = if _limbs_add_to_out_special(scratch_init, high_n + 1, poly_1) {
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
pub(crate) fn _limbs_mul_toom_evaluate_poly_in_1_and_neg_1(
    xp1: &mut [Limb],
    xm1: &mut [Limb],
    k: usize,
    xp: &[Limb],
    n: usize,
    hn: usize,
    tp: &mut [Limb], //TODO remove
    xs: &[Limb],
    ys: &[Limb],
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
        if *xs.last().unwrap() != 0 && *ys.last().unwrap() != 0 {
            panic!(
                "i < k in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1: {:?} {:?}",
                xs, ys
            );
        }
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

// Evaluates a polynomial of degree 2 < `degree` < GMP_NUMB_BITS, in the points +2 and -2, where
// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
//
// This is mpn_toom_eval_pm2 from mpn/generic/toom_eval_pm2.c.
// TODO continue cleaning
pub(crate) fn _limbs_mul_toom_evaluate_poly_in_2_and_neg_2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    mut degree: u32,
    poly: &[Limb],
    n: usize,
    n_high: usize,
    scratch: &mut [Limb],
) -> bool {
    assert!(degree > 2);
    assert!(degree < Limb::WIDTH);
    assert_ne!(n_high, 0);
    assert!(n_high <= n);

    // The degree `degree` is also the number of full-size coefficients, so that last coefficient,
    // of size `n_high`, starts at `poly[degree * n..]`.
    let degree_u = degree as usize;
    let mut cy = 0;
    shl_2_and_add_with_carry_to_out(
        v_2,
        &poly[degree_u * n..degree_u * n + n_high],
        &poly[(degree_u - 2) * n..(degree_u - 2) * n + n_high],
        &mut cy,
    );
    if n_high != n {
        cy = if limbs_add_limb_to_out(
            &mut v_2[n_high..],
            &poly[(degree_u - 2) * n + n_high..(degree_u - 1) * n],
            cy,
        ) {
            1
        } else {
            0
        };
    }
    let mut i = degree_u - 4;
    loop {
        shl_2_and_add_with_carry_in_place_left(&mut v_2[..n], &poly[i * n..(i + 1) * n], &mut cy);
        if i < 2 {
            break;
        }
        i -= 2;
    }
    v_2[n] = cy;
    degree.wrapping_sub_assign(1);
    let degree_u = degree as usize;

    cy = 0;
    shl_2_and_add_with_carry_to_out(
        scratch,
        &poly[degree_u * n..(degree_u + 1) * n],
        &poly[(degree_u - 2) * n..(degree_u - 1) * n],
        &mut cy,
    );
    if degree_u >= 4 {
        let mut i = degree_u - 4;
        loop {
            shl_2_and_add_with_carry_in_place_left(
                &mut scratch[..n],
                &poly[i * n..(i + 1) * n],
                &mut cy,
            );
            if i < 2 {
                break;
            }
            i -= 2;
        }
    }
    scratch[n] = cy;

    let limit = n + 1;
    if (degree & 1) != 0 {
        assert_eq!(limbs_slice_shl_in_place(&mut scratch[..limit], 1), 0);
    } else {
        assert_eq!(limbs_slice_shl_in_place(&mut v_2[..limit], 1), 0);
    }

    let mut neg = limbs_cmp_same_length(&v_2[..limit], &scratch[..limit]) == Ordering::Less;

    if neg {
        limbs_sub_same_length_to_out(v_neg_2, &scratch[..limit], &v_2[..limit]);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, &v_2[..limit], &scratch[..limit]);
    }

    limbs_slice_add_same_length_in_place_left(&mut v_2[..limit], &scratch[..limit]);

    assert!(v_2[n] < (1 << (degree + 2)) - 1);
    assert!(v_neg_2[n] < Limb::from(((1 << (degree + 3)) - 1 - (1 ^ degree & 1)) / 3));

    if degree.even() {
        neg.not_assign();
    }
    neg
}

/// Evaluates a polynomial of degree `degree` > 2, in the points 2 ^ shift and -2 ^ shift, where
/// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
///
/// This is mpn_toom_eval_pm2exp from mpn/generic/toom_eval_pm2exp.c.
pub(crate) fn _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(
    v_2_pow: &mut [Limb],
    v_neg_2_pow: &mut [Limb],
    degree: u32,
    poly: &[Limb],
    n: usize,
    n_high: usize,
    shift: u32,
    scratch: &mut [Limb],
) -> bool {
    assert!(degree >= 3);
    assert!(shift * degree < Limb::WIDTH);
    assert_ne!(n_high, 0);
    assert!(n_high <= n);
    assert_eq!(scratch.len(), n + 1);
    assert_eq!(v_2_pow.len(), n + 1);
    let degree_u = degree as usize;
    let mut coefficients = Vec::with_capacity(degree_u + 1);
    let mut lower_index;
    let mut upper_index = 0;
    for _ in 0..degree {
        lower_index = upper_index;
        upper_index += n;
        coefficients.push(&poly[lower_index..upper_index]);
    }
    coefficients.push(&poly[upper_index..]);
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
        while i < degree_u {
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
        while i < degree_u {
            *scratch_last += limbs_shl_to_out(v_neg_2_pow, coefficients[i], (i as u32) * shift);
            if limbs_slice_add_same_length_in_place_left(scratch_init, &v_neg_2_pow[..n]) {
                scratch_last.wrapping_add_assign(1);
            }
            i += 2;
        }
    }

    v_neg_2_pow[n_high] = limbs_shl_to_out(v_neg_2_pow, coefficients[degree_u], degree * shift);
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
