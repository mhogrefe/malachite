use malachite_base::num::{WrappingAddAssign, WrappingSubAssign};
use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::div_exact_limb::limbs_div_exact_3_in_place;
use natural::arithmetic::shl_u::limbs_shl_to_out;
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_in_place_with_overlap, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use platform::Limb;

/// This is mpn_toom_interpolate_5pts in mpn/generic/toom_interpolate_5pts.c.
pub(crate) fn _limbs_mul_toom_interpolate_5_points(
    c: &mut [Limb],
    v_2: &mut [Limb],
    v_neg_1: &mut [Limb],
    k: usize,
    two_r: usize,
    v_neg_1_neg: bool,
    mut v_inf_0: Limb,
) {
    let two_k = k + k;
    let two_k_plus_1 = two_k + 1;
    let four_k_plus_1 = two_k_plus_1 + two_k;
    assert_eq!(v_neg_1.len(), two_k_plus_1);
    assert!(two_r <= two_k);
    let v_2 = &mut v_2[..two_k_plus_1];
    {
        let v_1 = &c[two_k..four_k_plus_1]; // v_1 length: 2 * k + 1

        // (1) v_2 <- v_2 - v_neg_1 < v_2 + |v_neg_1|,            (16 8 4 2 1) - (1 -1 1 -1  1) =
        // thus 0 <= v_2 < 50 * B ^ (2 * k) < 2 ^ 6 * B ^ (2 * k) (15 9 3  3  0)
        //
        if v_neg_1_neg {
            assert!(!limbs_slice_add_same_length_in_place_left(v_2, v_neg_1));
        } else {
            assert!(!limbs_sub_same_length_in_place_left(v_2, v_neg_1));
        }

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0        v_1             hi(v_inf)      |v_neg_1|     v_2-v_neg_1          EMPTY
        limbs_div_exact_3_in_place(v_2); // v_2 <- v_2 / 3
                                         // (5 3 1 1 0)

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1             hi(v_inf)        |v_neg_1|    (v_2-v_neg_1)/3       EMPTY
        //
        // (2) v_neg_1 <- tm1 := (v_1 - v_neg_1) / 2  [(1 1 1 1 1) - (1 -1 1 -1 1)] / 2 =
        // tm1 >= 0                                    (0  1 0  1 0)
        // No carry comes out from {v_1, two_k_plus_1} +/- {v_neg_1, two_k_plus_1},
        // and the division by two is exact.
        // If v_neg_1_neg the sign of v_neg_1 is negative
        if v_neg_1_neg {
            assert!(!limbs_slice_add_same_length_in_place_left(v_neg_1, v_1));
        } else {
            assert!(!limbs_sub_same_length_in_place_right(v_1, v_neg_1));
        }
        assert_eq!(limbs_slice_shr_in_place(v_neg_1, 1), 0);

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1             hi(v_inf)          tm1       (v_2-v_neg_1)/3        EMPTY
        //
        // (3) v_1 <- t1 := v_1 - v0  (1 1 1 1 1) - (0 0 0 0 1) = (1 1 1 1 0)
        // t1 >= 0
    }
    {
        let (c_lo, v_1) = c.split_at_mut(two_k);
        if limbs_sub_same_length_in_place_left(&mut v_1[..two_k], c_lo) {
            v_1[two_k].wrapping_sub_assign(1);
        }
        let v1 = &mut v_1[..two_k_plus_1];
        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1-v0           hi(v_inf)          tm1      (v_2-v_neg_1)/3        EMPTY
        //
        // (4) v_2 <- t2 := ((v_2 - v_neg_1) / 3 - t1) / 2 = (v_2 - v_neg_1 - 3 * t1) / 6
        // t2 >= 0                  [(5 3 1 1 0) - (1 1 1 1 0)]/2 = (2 1 0 0 0)
        //
        assert!(!limbs_sub_same_length_in_place_left(v_2, v1));
        assert_eq!(limbs_slice_shr_in_place(v_2, 1), 0);
        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0      v_1 - v0        hi(v_inf)          tm1    (v_2 - v_neg_1 - 3t1) / 6    EMPTY
        //
        // (5) v_1 <- t1 - tm1           (1 1 1 1 0) - (0 1 0 1 0) = (1 0 1 0 0)
        // result is v_1 >= 0
        //
        assert!(!limbs_sub_same_length_in_place_left(v1, v_neg_1));
    }

    let saved;
    // We do not need to read the value in v_neg_1, so we add it in {c + k, ..}
    {
        let (c_lo, c_hi) = c.split_at_mut(3 * k + 1);
        if limbs_slice_add_same_length_in_place_left(&mut c_lo[k..], v_neg_1) {
            // 2 * n - (3 * k + 1) = 2 * r + k - 1
            // Memory allocated for v_neg_1 is now free, it can be recycled
            assert!(!limbs_slice_add_limb_in_place(
                &mut c_hi[..two_r + k - 1],
                1
            ));
        }
        let v_inf = &mut c_hi[k - 1..two_r + k - 1];
        //let (_, v_inf) = remainder.split_at_mut(k);
        // (6) v_2 <- v_2 - 2 * v_inf, (2 1 0 0 0) - 2 * (1 0 0 0 0) = (0 1 0 0 0)
        // result is v_2 >= 0
        saved = v_inf[0]; // Remember v1's highest byte (will be overwritten).
        v_inf[0] = v_inf_0; // Set the right value for v_inf_0
                            // Overwrite unused v_neg_1
        let mut carry = limbs_shl_to_out(v_neg_1, &mut v_inf[..two_r], 1);
        if limbs_sub_same_length_in_place_left(&mut v_2[..two_r], &v_neg_1[..two_r]) {
            carry += 1;
        }
        assert!(!limbs_sub_limb_in_place(&mut v_2[two_r..], carry));
    }
    //  Current matrix is
    //  [1 0 0 0 0; v_inf
    //   0 1 0 0 0; v_2
    //   1 0 1 0 0; v1
    //   0 1 0 1 0; v_neg_1
    //   0 0 0 0 1] v0
    //  Some values already are in-place (we added v_neg_1 in the correct position)
    //  | v_inf|  v1 |  v0 |
    //       | v_neg_1 |
    //  One still is in a separated area
    // | +v_2 |
    //  We have to compute v1-=v_inf; v_neg_1 -= v_2,
    //    |-v_inf|
    //       | -v_2 |
    //  Carefully reordering operations we can avoid to compute twice the sum
    //  of the high half of v_2 plus the low half of v_inf.
    //
    // Add the high half of t2 in {v_inf}
    if two_r > k + 1 {
        // This is the expected flow
        let (c_lo, c_hi) = c[4 * k..].split_at_mut(k + 1);
        if limbs_slice_add_same_length_in_place_left(c_lo, &v_2[k..]) {
            // 2n-(5k+1) = 2r-k-1
            assert!(!limbs_slice_add_limb_in_place(
                &mut c_hi[..two_r - k - 1],
                1
            ));
        }
    } else {
        // triggered only by very unbalanced cases like (k+k+(k-2))x(k+k+1), should be handled by
        // toom32
        // two_r < k + 1 so k + two_r < two_k, the size of v_2
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut c[4 * k..4 * k + two_r],
            &v_2[k..k + two_r]
        ));
    }
    let carry;
    {
        let (v_1, v_inf) = c[2 * k..].split_at_mut(2 * k);
        // (7) v_1 <- v_1 - v_inf,       (1 0 1 0 0) - (1 0 0 0 0) = (0 0 1 0 0)
        // result is >= 0
        // Side effect: we also subtracted (high half) v_neg_1 -= v_2
        // v_inf is at most two_r long.
        carry = limbs_sub_same_length_in_place_left(&mut v_1[..two_r], &v_inf[..two_r]);
        v_inf_0 = v_inf[0]; // Save again the right value for v_inf_0
        v_inf[0] = saved;
    }
    {
        let (c1, v1) = c[k..].split_at_mut(k);
        let v1 = &mut v1[..two_k_plus_1];
        if carry {
            assert!(!limbs_sub_limb_in_place(&mut v1[two_r..], 1)); // Treat the last bytes.
        }

        // (8) v_neg_1 <- v_neg_1 - v_2 (0 1 0 1 0) - (0 1 0 0 0) = (0 0 0 1 0)
        // Operate only on the low half.
        //
        if limbs_sub_same_length_in_place_left(c1, &v_2[..k]) {
            assert!(!limbs_sub_limb_in_place(v1, 1));
        }
    }
    let (c3, v_inf) = c[3 * k..].split_at_mut(k);
    // Beginning the final phase
    // Most of the recomposition was done
    // add t2 in {c + 3 * k, ...}, but only the low half
    if limbs_slice_add_same_length_in_place_left(c3, &v_2[..k]) {
        v_inf[0].wrapping_add_assign(1);
        assert!(v_inf[0] >= 1); // No carry
    }

    // Add v_inf_0, propagate carry.
    assert!(!limbs_slice_add_limb_in_place(&mut v_inf[..two_r], v_inf_0));
}

/// Interpolation for Toom-3.5, using the evaluation points infinity, 1, -1, 2, -2. More precisely,
/// we want to compute f(2 ^ (GMP_NUMB_BITS * n)) for a polynomial f of degree 5, given the six
/// values
///
/// w5 = f(0),
/// w4 = f(-1),
/// w3 = f(1)
/// w2 = f(-2),
/// w1 = f(2),
/// w0 = limit at infinity of f(x) / x^5,
///
/// The result is stored in {out_limbs, 5 * n + n_high}. At entry, w5 is stored at
/// {out_limbs, 2 * n}, w3 is stored at {out_limbs + 2 * n, 2 * n + 1}, and w0 is stored at
/// {out_limbs + 5 * n, n_high}. The other values are 2 * n + 1 limbs each (with most significant
/// limbs small). f(-1) and f(-2) may be negative; signs are passed in. All intermediate results are
/// positive. Inputs are destroyed.
///
/// Interpolation sequence was taken from the paper: "Integer and Polynomial Multiplication: Towards
/// Optimal Toom-Cook Matrices". Some slight variations were introduced: adaptation to "gmp
/// instruction set", and a final saving of an operation by interlacing interpolation and
/// recomposition phases.
///
/// This is mpn_toom_interpolate_6pts from mpn/generic/mpn_toom_interpolate_6pts.c, but the argument
/// w0n == `n_high` is moved to immediately after `n`.
pub(crate) fn _limbs_mul_toom_interpolate_6_points(
    out_limbs: &mut [Limb],
    n: usize,
    n_high: usize,
    w4_neg: bool,
    w4: &mut [Limb],
    w2_neg: bool,
    w2: &mut [Limb],
    w1: &mut [Limb],
) {
    assert_ne!(n, 0);
    assert!(2 * n >= n_high && n_high > 0);
    let limit = 2 * n + 1;
    {
        let (w5, w3) = out_limbs.split_at_mut(2 * n); // w5 length: 2 * n

        // Interpolate with sequence:
        // w2 = (w1 - w2) >> 2
        // w1 = (w1 - w5) >> 1
        // w1 = (w1 - w2) >> 1
        // w4 = (w3 - w4) >> 1
        // w2 = (w2 - w4) / 3
        // w3 =  w3 - w4 - w5
        // w1 = (w1 - w3) / 3
        //
        // Last steps are mixed with recomposition:
        // w2 = w2 - w0 << 2
        // w4 = w4 - w2
        // w3 = w3 - w1
        // w2 = w2 - w0
        //
        // w2 = (w1 - w2) >> 2
        if w2_neg {
            limbs_slice_add_same_length_in_place_left(&mut w2[..limit], &w1[..limit]);
        } else {
            limbs_sub_same_length_in_place_right(&w1[..limit], &mut w2[..limit]);
        }
        limbs_slice_shr_in_place(&mut w2[..limit], 2);

        // w1 = (w1 - w5) >> 1
        if limbs_sub_same_length_in_place_left(&mut w1[..2 * n], w5) {
            w1[2 * n].wrapping_sub_assign(1);
        }
        limbs_slice_shr_in_place(&mut w1[..limit], 1);

        // w1 = (w1 - w2) >> 1
        limbs_sub_same_length_in_place_left(&mut w1[..limit], &w2[..limit]);
        limbs_slice_shr_in_place(&mut w1[..limit], 1);

        // w4 = (w3 - w4) >> 1
        if w4_neg {
            limbs_slice_add_same_length_in_place_left(&mut w4[..limit], &w3[..limit]);
            limbs_slice_shr_in_place(&mut w4[..limit], 1);
        } else {
            limbs_sub_same_length_in_place_right(&w3[..limit], &mut w4[..limit]);
            limbs_slice_shr_in_place(&mut w4[..limit], 1);
        }

        // w2 = (w2 - w4) / 3
        limbs_sub_same_length_in_place_left(&mut w2[..limit], &w4[..limit]);
        limbs_div_exact_3_in_place(&mut w2[..limit]);

        // w3 = w3 - w4 - w5
        limbs_sub_same_length_in_place_left(&mut w3[..limit], &w4[..limit]);
        if limbs_sub_same_length_in_place_left(&mut w3[..2 * n], w5) {
            w3[2 * n].wrapping_sub_assign(1);
        }

        // w1 = (w1 - w3) / 3
        limbs_sub_same_length_in_place_left(&mut w1[..limit], &w3[..limit]);
        limbs_div_exact_3_in_place(&mut w1[..limit]);
    }
    // [1 0 0 0 0 0;
    //  0 1 0 0 0 0;
    //  1 0 1 0 0 0;
    //  0 1 0 1 0 0;
    //  1 0 1 0 1 0;
    //  0 0 0 0 0 1]
    //
    // out_limbs[] prior to operations:
    //  |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //
    // summation scheme for remaining operations:
    //  |______________5|n_____4|n_____3|n_____2|n______|n______| out_limbs
    //  |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //                 || H w4  | L w4  |
    //         || H w2  | L w2  |
    //     || H w1  | L w1  |
    //             ||-H w1  |-L w1  |
    //          |-H w0  |-L w0 ||-H w2  |-L w2  |
    //
    if limbs_slice_add_same_length_in_place_left(&mut out_limbs[n..=3 * n], &w4[..limit]) {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[3 * n + 1..=4 * n],
            1
        ));
    }

    // w2 -= w0 << 2
    // {w4, 2 * n + 1} is now free and can be overwritten.
    let mut carry = limbs_shl_to_out(w4, &mut out_limbs[5 * n..5 * n + n_high], 2);
    if limbs_sub_same_length_in_place_left(&mut w2[..n_high], &w4[..n_high]) {
        carry += 1;
    }
    assert!(!limbs_sub_limb_in_place(&mut w2[n_high..limit], carry));

    // w4L = w4L - w2L
    if limbs_sub_same_length_in_place_left(&mut out_limbs[n..2 * n], &w2[..n]) {
        assert!(!limbs_sub_limb_in_place(
            &mut out_limbs[2 * n..2 * n + limit],
            1
        ));
    }

    let carry = if limbs_slice_add_same_length_in_place_left(&mut out_limbs[3 * n..4 * n], &w2[..n])
    {
        1
    } else {
        0
    };
    // w3H = w3H + w2L
    let special_carry_1 = out_limbs[4 * n] + carry;
    // w1L + w2H
    let mut carry = w2[2 * n];
    if limbs_add_same_length_to_out(&mut out_limbs[4 * n..], &w1[..n], &w2[n..2 * n]) {
        carry += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(&mut w1[n..limit], carry));
    // w0 = w0 + w1H
    let mut special_carry_2 = 0;
    if n_high > n {
        special_carry_2 = w1[2 * n];
        if limbs_slice_add_same_length_in_place_left(&mut out_limbs[5 * n..6 * n], &w1[n..2 * n]) {
            special_carry_2.wrapping_add_assign(1);
        }
    } else if limbs_slice_add_same_length_in_place_left(
        &mut out_limbs[5 * n..5 * n + n_high],
        &w1[n..n + n_high],
    ) {
        special_carry_2 = 1;
    }

    // summation scheme for the next operation:
    //  |...____5|n_____4|n_____3|n_____2|n______|n______| out_limbs
    //  |...w0___|_w1_w2_|_H w3__|_L w3__|_H w5__|_L w5__|
    //          ...-w0___|-w1_w2 |
    //
    // if (LIKELY(n_high > n)) the two operands below DO overlap!
    let carry =
        _limbs_sub_same_length_in_place_with_overlap(&mut out_limbs[2 * n..5 * n + n_high], 2 * n);

    // embankment is a "dirty trick" to avoid carry/borrow propagation beyond allocated memory
    let embankment;
    {
        let out_high = &mut out_limbs[5 * n + n_high - 1];
        embankment = out_high.wrapping_sub(1);
        *out_high = 1;
    }
    if n_high > n {
        if special_carry_1 > special_carry_2 {
            assert!(!limbs_slice_add_limb_in_place(
                &mut out_limbs[4 * n..5 * n + n_high],
                special_carry_1 - special_carry_2
            ));
        } else {
            assert!(!limbs_sub_limb_in_place(
                &mut out_limbs[4 * n..5 * n + n_high],
                special_carry_2 - special_carry_1
            ));
        }
        if carry {
            assert!(!limbs_sub_limb_in_place(
                &mut out_limbs[3 * n + n_high..5 * n + n_high],
                1
            ));
        }
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[6 * n..5 * n + n_high],
            special_carry_2
        ));
    } else {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[4 * n..5 * n + n_high],
            special_carry_1
        ));
        if carry {
            special_carry_2.wrapping_add_assign(1);
        }
        assert!(!limbs_sub_limb_in_place(
            &mut out_limbs[3 * n + n_high..5 * n + n_high],
            special_carry_2
        ));
    }
    out_limbs[5 * n + n_high - 1].wrapping_add_assign(embankment);
}
