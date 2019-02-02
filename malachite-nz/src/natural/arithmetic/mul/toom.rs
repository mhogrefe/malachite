use malachite_base::limbs::{limbs_set_zero, limbs_test_zero};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{
    NotAssign, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned, WrappingAddAssign,
    WrappingSubAssign,
};
use natural::arithmetic::add::{
    _limbs_add_same_length_with_carry_in_in_place_left, limbs_add_same_length_to_out,
    limbs_add_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::arithmetic::add_mul_limb::mpn_addmul_1;
use natural::arithmetic::mul::poly_eval::{
    _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1,
    _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2,
    _limbs_mul_toom_evaluate_poly_in_1_and_neg_1, _limbs_mul_toom_evaluate_poly_in_2_and_neg_2,
    _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow,
};
use natural::arithmetic::mul::poly_interpolate::{
    _limbs_mul_toom_interpolate_5_points, _limbs_mul_toom_interpolate_6_points,
    _limbs_mul_toom_interpolate_7_points, _limbs_mul_toom_interpolate_8_points,
};
use natural::arithmetic::mul::{
    _limbs_mul_greater_to_out_basecase, limbs_mul_greater_to_out, limbs_mul_same_length_to_out,
    limbs_mul_to_out,
};
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_sub_in_place_left,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::comparison::ord::limbs_cmp_same_length;
use platform::{Limb, SignedLimb};
use std::cmp::Ordering;

//TODO tune all
pub const MUL_TOOM22_THRESHOLD: usize = 30;
pub const MUL_TOOM33_THRESHOLD: usize = 100;
pub const MUL_TOOM44_THRESHOLD: usize = 300;
pub const MUL_TOOM6H_THRESHOLD: usize = 350;
pub const MUL_TOOM8H_THRESHOLD: usize = 450;
pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 100;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 110;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 100;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 110;
pub const MUL_TOOM33_THRESHOLD_LIMIT: usize = MUL_TOOM33_THRESHOLD;

/// Helper function for high degree Toom-Cook algorithms.
///
/// Gets {`xs`, `n`} and (`y_sign` ? -1 : 1) * {`ys`, `n`}. Computes at once:
///   {`xs`, `n`} <- ({`xs`, `n`} + {`ys`, `n`}) / 2 ^ {`x_shift` + 1}
///   {`ys`, `n`} <- ({`xs`, `n`} - {`ys`, `n`}) / 2 ^ {`y_shift` + 1}
/// Finally recompose them obtaining:
///   {`xs`, `n` + `offset`} <- {`xs`, `n`} + {`ys`, `n`} * 2 ^ {`offset` * `Limb::WIDTH`}
///
/// This is mpn_toom_couple_handling from mpn/generic/toom_couple_handling.c. The argument `n` is
/// excluded as it is just the length of ys.
fn _limbs_toom_couple_handling(
    xs: &mut [Limb],
    ys: &mut [Limb],
    y_sign: bool,
    offset: usize,
    x_shift: u32,
    y_shift: u32,
) {
    let n = ys.len();
    assert!(xs.len() >= n + offset);
    let (xs_lo, xs_hi) = xs.split_at_mut(n);
    if y_sign {
        limbs_sub_same_length_in_place_right(xs_lo, ys);
    } else {
        limbs_slice_add_same_length_in_place_left(ys, xs_lo);
    }
    limbs_slice_shr_in_place(ys, 1);
    limbs_sub_same_length_in_place_left(xs_lo, ys);
    if x_shift != 0 {
        limbs_slice_shr_in_place(xs_lo, x_shift);
    }
    if y_shift != 0 {
        limbs_slice_shr_in_place(ys, y_shift);
    }
    if limbs_slice_add_same_length_in_place_left(&mut xs_lo[offset..], &ys[..n - offset]) {
        assert!(!limbs_add_limb_to_out(xs_hi, &ys[n - offset..], 1));
    } else {
        xs_hi[..offset].copy_from_slice(&ys[n - offset..]);
    }
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_22`.
///
/// Scratch need is 2 * (xs.len() + k); k is the recursion depth. k is the smallest k such that
///   ceil(xs.len() / 2 ^ k) < MUL_TOOM22_THRESHOLD,
/// which implies that
///   k = bitsize of floor((xs.len() - 1) / (MUL_TOOM22_THRESHOLD - 1))
///     = 1 + floor(log_2(floor((xs.len() - 1) / (MUL_TOOM22_THRESHOLD - 1))))
///
/// The actual scratch size returned is a quicker-to-compute upper bound.
///
/// This is mpn_toom22_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_22_scratch_size(xs_len: usize) -> usize {
    2 * (xs_len + Limb::WIDTH as usize)
}

// TODO make these compiler flags?
pub const TUNE_PROGRAM_BUILD: bool = true;
pub const WANT_FAT_BINARY: bool = false;

pub const TOOM22_MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD >= 2 * MUL_TOOM22_THRESHOLD;

/// A helper function for `_limbs_mul_greater_to_out_toom_22`.
///
/// This is TOOM22_MUL_N_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_same_length_to_out_toom_22_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    assert_eq!(xs.len(), ys.len());
    if !TOOM22_MAYBE_MUL_TOOM22 || xs.len() < MUL_TOOM22_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else {
        _limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    }
}

/// A helper function for `_limbs_mul_greater_to_out_toom_22`.
///
/// Normally, this calls `_limbs_mul_greater_to_out_basecase` or
/// `_limbs_mul_greater_to_out_toom_22`. But when the fraction
/// MUL_TOOM33_THRESHOLD / MUL_TOOM22_THRESHOLD is large, an initially small relative unbalance will
/// become a larger and larger relative unbalance with each recursion (the difference s - t will be
/// invariant over recursive calls). Therefore, we need to call `_limbs_mul_greater_to_out_toom_32`.
///
/// This is TOOM22_MUL_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_greater_to_out_toom_22_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if !TOOM22_MAYBE_MUL_TOOM22 || ys_len < MUL_TOOM22_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if 4 * xs_len < 5 * ys_len {
        _limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if _limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len, ys_len) {
        _limbs_mul_greater_to_out_toom_32(out, xs, ys, scratch);
    } else {
        limbs_mul_greater_to_out(out, xs, ys);
    }
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_22` are valid.
pub fn _limbs_mul_greater_to_out_toom_22_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let s = xs_len >> 1;
    let n = xs_len - s;
    let t = ys_len.saturating_sub(n);
    s > 0 && (s == n || s == n - 1) && 0 < t && t <= s
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_22_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_22_input_sizes_valid`. The gist is that `xs` must
/// be less than twice as long as `ys`.
///
/// This uses the Toom-22, aka Toom-2, aka Karatsuba algorithm.
///
/// Evaluate in: -1, 0, +inf
///
///  <--s--><--n--->
///   ______________
///  |_xs1_|__xs0__|
///   |ys1_|__ys0__|
///   <-t--><--n--->
///
///  v0   = xs0         * ys0         # X(0)   * Y(0)
///  vm1  = (xs0 - xs1) * (ys0 - ys1) # X(-1)  * Y(-1)
///  vinf = xs1         * ys1         # X(inf) * Y(inf)
///
/// Time: TODO (should be something like O(n<sup>log<sub>2</sub>3</sup>))
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom22_mul from mpn/generic/toom22_mul.c.
pub fn _limbs_mul_greater_to_out_toom_22(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let s = xs_len >> 1;
    let n = xs_len - s;
    assert!(ys_len >= n);
    let t = ys_len - n;

    assert!(s > 0 && (s == n || s == n - 1));
    assert!(0 < t && t <= s);

    let (xs_0, xs_1) = xs.split_at(n); // xs_0: length n, xs_1: length s
    let (ys_0, ys_1) = ys.split_at(n); // ys_0: length n, ys_1: length t

    let mut v_neg_1_neg = false;
    {
        let (asm1, bsm1) = out.split_at_mut(n); // asm1: length n

        // Compute asm1.
        if s == n {
            if limbs_cmp_same_length(xs_0, xs_1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs_1, xs_0);
                v_neg_1_neg = true;
            } else {
                limbs_sub_same_length_to_out(asm1, xs_0, xs_1);
            }
        } else {
            // n - s == 1
            if xs_0[s] == 0 && limbs_cmp_same_length(&xs_0[..s], xs_1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs_1, &xs_0[..s]);
                asm1[s] = 0;
                v_neg_1_neg = true;
            } else {
                asm1[s] = xs_0[s];
                if limbs_sub_same_length_to_out(asm1, &xs_0[..s], xs_1) {
                    asm1[s].wrapping_sub_assign(1);
                }
            }
        }

        // Compute bsm1.
        if t == n {
            if limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less {
                limbs_sub_same_length_to_out(bsm1, ys_1, ys_0);
                v_neg_1_neg.not_assign();
            } else {
                limbs_sub_same_length_to_out(bsm1, ys_0, ys_1);
            }
        } else if limbs_test_zero(&ys_0[t..])
            && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less
        {
            limbs_sub_same_length_to_out(bsm1, ys_1, &ys_0[..t]);
            limbs_set_zero(&mut bsm1[t..n]);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_to_out(bsm1, ys_0, ys_1);
        }

        let (v_neg_1, scratch_out) = scratch.split_at_mut(2 * n); // v_neg_1: length 2 * n
        _limbs_mul_same_length_to_out_toom_22_recursive(v_neg_1, asm1, &bsm1[..n], scratch_out);
    }
    let (v_neg_1, scratch_out) = scratch.split_at_mut(2 * n); // v_neg_1: length 2 * n
    let mut carry = 0;
    let mut carry2;
    {
        let (v_0, v_pos_inf) = out.split_at_mut(2 * n); // v_0: length 2 * n
        if s > t {
            _limbs_mul_greater_to_out_toom_22_recursive(v_pos_inf, xs_1, ys_1, scratch_out);
        } else {
            _limbs_mul_same_length_to_out_toom_22_recursive(
                v_pos_inf,
                xs_1,
                &ys_1[..s],
                scratch_out,
            );
        }

        // v_0, 2 * n limbs
        _limbs_mul_same_length_to_out_toom_22_recursive(v_0, xs_0, ys_0, scratch_out);

        // H(v_0) + L(v_pos_inf)
        if limbs_slice_add_same_length_in_place_left(&mut v_pos_inf[..n], &v_0[n..]) {
            carry += 1;
        }

        // L(v_0) + H(v_0)
        carry2 = carry;
        let (v_0_lo, v_0_hi) = v_0.split_at_mut(n); // v_0_lo: length n, vo_hi: length n
        if limbs_add_same_length_to_out(v_0_hi, &v_pos_inf[..n], v_0_lo) {
            carry2 += 1;
        }

        // L(v_pos_inf) + H(v_pos_inf)
        let (v_pos_inf_lo, v_pos_inf_hi) = v_pos_inf.split_at_mut(n); // v_pos_inf_lo: length n

        // s + t - n == either ys_len - (xs_len >> 1) or ys_len - (xs_len >> 1) - 2.
        // n == xs_len - (xs_len >> 1) and xs_len >= ys_len.
        // So n >= s + t - n.
        if limbs_slice_add_greater_in_place_left(v_pos_inf_lo, &v_pos_inf_hi[..s + t - n]) {
            carry += 1;
        }
    }

    if v_neg_1_neg {
        if limbs_slice_add_same_length_in_place_left(&mut out[n..3 * n], v_neg_1) {
            carry += 1;
        }
    } else if limbs_sub_same_length_in_place_left(&mut out[n..3 * n], v_neg_1) {
        carry.wrapping_sub_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut out[2 * n..2 * n + s + t],
        carry2
    ));
    if carry <= 2 {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out[3 * n..2 * n + s + t],
            carry
        ));
    } else {
        assert!(!limbs_sub_limb_in_place(&mut out[3 * n..2 * n + s + t], 1));
    }
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_32`.
///
/// This is mpn_toom32_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_32_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };
    2 * n + 1
}

/// A helper function for `_limbs_mul_greater_to_out_toom_22`.
///
/// This is TOOM32_MUL_N_REC from mpn/generic/toom32_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_32_recursive(p: &mut [Limb], a: &[Limb], b: &[Limb]) {
    limbs_mul_same_length_to_out(p, a, b);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_32` are valid.
pub fn _limbs_mul_greater_to_out_toom_32_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };
    if ys_len + 2 > xs_len || xs_len + 6 > 3 * ys_len {
        return false;
    }
    let s = xs_len.saturating_sub(2 * n);
    let t = ys_len.saturating_sub(n);
    0 < s && s <= n && 0 < t && t <= n && s + t >= n
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_32_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_32_input_sizes_valid`. The gist is that `xs` must
/// be less than 3 times as long as `ys`.
///
/// This uses the Toom-32 aka Toom-2.5 algorithm.
///
/// Evaluate in: -1, 0, +1, +inf
///
/// <-s-><--n--><--n-->
///  ___________________
/// |xs2_|__xs1_|__xs0_|
///        |ys1_|__ys0_|
///        <-t--><--n-->
///
/// v0   =  xs0              * ys0         # X(0)   * Y(0)
/// v1   = (xs0 + xs1 + xs2) * (ys0 + ys1) # X(1)   * Y(1)    xh  <= 2  yh <= 1
/// vm1  = (xs0 - xs1 + xs2) * (ys0 - ys1) # X(-1)  * Y(-1)  |xh| <= 1  yh = 0
/// vinf =               xs2 * ys1         # X(inf) * Y(inf)
///
/// Time: TODO (should be something like O(n<sup>k</sup>), where k = 2log(2)/(log(5)-log(2))?)
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom32_mul from mpn/generic/toom32_mul.c.
#[allow(unreachable_code)] //TODO remove
#[allow(clippy::cyclomatic_complexity)]
pub fn _limbs_mul_greater_to_out_toom_32(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };

    // Required, to ensure that s + t >= n.
    assert!(ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len);

    split_into_chunks!(xs, n, s, [xs_0, xs_1], xs_2);
    split_into_chunks!(ys, n, t, [ys_0], ys_1);

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    assert!(s + t >= n);

    let mut hi: SignedLimb;
    let mut v_neg_1_neg;
    {
        // Product area of size xs_len + ys_len = 3 * n + s + t >= 4 * n + 2.
        // out_lo: length 2 * n
        let (out_lo, out_hi) = out.split_at_mut(2 * n);
        let (am1, bm1) = out_hi.split_at_mut(n); // am1: length n
        {
            let (ap1, bp1) = out_lo.split_at_mut(n); // ap1: length n, bp1: length n

            // Compute ap1 = xs0 + xs1 + a3, am1 = xs0 - xs1 + a3
            let mut ap1_hi = if limbs_add_to_out(ap1, xs_0, xs_2) {
                1
            } else {
                0
            };
            if ap1_hi == 0 && limbs_cmp_same_length(ap1, xs_1) == Ordering::Less {
                assert!(!limbs_sub_same_length_to_out(am1, xs_1, ap1));
                hi = 0;
                v_neg_1_neg = true;
            } else {
                hi = ap1_hi;
                if limbs_sub_same_length_to_out(am1, ap1, xs_1) {
                    hi -= 1;
                }
                v_neg_1_neg = false;
            }
            if limbs_slice_add_same_length_in_place_left(ap1, xs_1) {
                ap1_hi += 1;
            }

            let bp1_hi;
            // Compute bp1 = ys0 + ys1 and bm1 = ys0 - ys1.
            if t == n {
                bp1_hi = limbs_add_same_length_to_out(bp1, ys_0, ys_1);
                if limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys_1, ys_0));
                    v_neg_1_neg.not_assign();
                } else {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys_0, ys_1));
                }
            } else {
                bp1_hi = limbs_add_to_out(bp1, ys_0, ys_1);
                if limbs_test_zero(&ys_0[t..])
                    && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less
                {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys_1, &ys_0[..t]));
                    limbs_set_zero(&mut bm1[t..n]);
                    v_neg_1_neg.not_assign();
                } else {
                    assert!(!limbs_sub_to_out(bm1, ys_0, ys_1));
                }
            }

            _limbs_mul_same_length_to_out_toom_32_recursive(scratch, ap1, bp1);
            let mut carry = 0;
            if ap1_hi == 1 {
                if limbs_slice_add_same_length_in_place_left(&mut scratch[n..2 * n], &bp1[..n]) {
                    carry = 1;
                }
                if bp1_hi {
                    carry += 1;
                }
            } else if ap1_hi == 2 {
                carry = mpn_addmul_1(&mut scratch[n..], bp1, 2);
                if bp1_hi {
                    carry += 2;
                }
            }
            if bp1_hi && limbs_slice_add_same_length_in_place_left(&mut scratch[n..2 * n], ap1) {
                carry += 1;
            }
            scratch[2 * n] = carry;
        }
        _limbs_mul_same_length_to_out_toom_32_recursive(out_lo, am1, &bm1[..n]);
        if hi != 0 {
            hi = 0;
            if limbs_slice_add_same_length_in_place_left(&mut out_lo[n..], &bm1[..n]) {
                hi = 1;
            }
        }
    }
    out[2 * n] = hi.to_unsigned_bitwise();

    // v1 <-- (v1 + vm1) / 2 = x0 + x2
    {
        let scratch = &mut scratch[..2 * n + 1];
        let out = &out[..2 * n + 1];
        if v_neg_1_neg {
            limbs_sub_same_length_in_place_left(scratch, out);
            assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
        } else {
            limbs_slice_add_same_length_in_place_left(scratch, &out);
            assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
        }
    }

    // We get x1 + x3 = (x0 + x2) - (x0 - x1 + x2 - x3), and hence
    //
    // y = x1 + x3 + (x0 + x2) * B
    //   = (x0 + x2) * B + (x0 + x2) - vm1.
    //
    // y is 3 * n + 1 limbs, y = y0 + y1 B + y2 B^2. We store them as follows: y0 at scratch, y1 at
    // out + 2 * n, and y2 at scratch + n (already in place, except for carry propagation).
    //
    // We thus add
    //
    //    B^3  B^2   B    1
    //     |    |    |    |
    //    +-----+----+
    //  + |  x0 + x2 |
    //    +----+-----+----+
    //  +      |  x0 + x2 |
    //         +----------+
    //  -      |  vm1     |
    //  --+----++----+----+-
    //    | y2  | y1 | y0 |
    //    +-----+----+----+
    //
    // Since we store y0 at the same location as the low half of x0 + x2, we need to do the middle
    // sum first.
    hi = out[2 * n].to_signed_bitwise();
    let mut scratch_high = scratch[2 * n];
    if limbs_add_same_length_to_out(&mut out[2 * n..], &scratch[..n], &scratch[n..2 * n]) {
        scratch_high += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut scratch[n..2 * n + 1],
        scratch_high
    ));

    if v_neg_1_neg {
        let carry = limbs_slice_add_same_length_in_place_left(&mut scratch[..n], &out[..n]);
        let (out_lo, out_hi) = out.split_at_mut(2 * n);
        // out_lo: length 2 * n
        if _limbs_add_same_length_with_carry_in_in_place_left(&mut out_hi[..n], &out_lo[n..], carry)
        {
            hi += 1;
        }
        assert!(!limbs_slice_add_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi.to_unsigned_bitwise()
        ));
    } else {
        let carry = limbs_sub_same_length_in_place_left(&mut scratch[..n], &out[..n]);
        let (out_lo, out_hi) = out.split_at_mut(2 * n);
        // out_lo: length 2 * n
        if _limbs_sub_same_length_with_borrow_in_in_place_left(
            &mut out_hi[..n],
            &out_lo[n..],
            carry,
        ) {
            hi += 1;
        }
        assert!(!limbs_sub_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi.to_unsigned_bitwise()
        ));
    }

    _limbs_mul_same_length_to_out_toom_32_recursive(out, xs_0, ys_0);
    // s + t limbs. Use mpn_mul for now, to handle unbalanced operands
    limbs_mul_to_out(&mut out[3 * n..], xs_2, ys_1);

    // Remaining interpolation.
    //
    //    y * B + x0 + x3 B^3 - x0 B^2 - x3 B
    //    = (x1 + x3) B + (x0 + x2) B^2 + x0 + x3 B^3 - x0 B^2 - x3 B
    //    = y0 B + y1 B^2 + y3 B^3 + Lx0 + H x0 B
    //      + L x3 B^3 + H x3 B^4 - Lx0 B^2 - H x0 B^3 - L x3 B - H x3 B^2
    //    = L x0 + (y0 + H x0 - L x3) B + (y1 - L x0 - H x3) B^2
    //      + (y2 - (H x0 - L x3)) B^3 + H x3 B^4
    //
    //     B^4       B^3       B^2        B         1
    //|         |         |         |         |         |
    //  +-------+                   +---------+---------+
    //  |  Hx3  |                   | Hx0-Lx3 |    Lx0  |
    //  +------+----------+---------+---------+---------+
    //     |    y2    |  y1     |   y0    |
    //     ++---------+---------+---------+
    //     -| Hx0-Lx3 | - Lx0   |
    //      +---------+---------+
    //             | - Hx3  |
    //             +--------+
    //
    // We must take into account the carry from Hx0 - Lx3.
    {
        split_into_chunks_mut!(out, n, [out_0, out_1, out_2], out_3);
        let carry = limbs_sub_same_length_in_place_left(out_1, &out_3[..n]);
        hi = scratch[2 * n].to_signed_bitwise();
        if carry {
            hi.wrapping_add_assign(1);
        }

        let borrow = _limbs_sub_same_length_with_borrow_in_in_place_left(out_2, out_0, carry);
        if _limbs_sub_same_length_with_borrow_in_to_out(out_3, &scratch[n..2 * n], out_1, borrow) {
            hi -= 1;
        }
    }

    if limbs_slice_add_greater_in_place_left(&mut out[n..4 * n], &scratch[..n]) {
        hi += 1;
    }

    if s + t > n {
        let (out_lo, out_hi) = out.split_at_mut(4 * n);
        // out_lo: length 4 * n
        let out_hi = &mut out_hi[..s + t - n];
        if limbs_sub_in_place_left(&mut out_lo[2 * n..], out_hi) {
            hi -= 1;
        }

        if hi < 0 {
            //TODO remove once this is seen
            panic!("hi < 0 second time: {:?} {:?}", xs, ys);
            assert!(!limbs_sub_limb_in_place(
                out_hi,
                Limb::checked_from(-hi).unwrap()
            ));
        } else {
            assert!(!limbs_slice_add_limb_in_place(
                out_hi,
                Limb::checked_from(hi).unwrap()
            ));
        }
    } else {
        assert_eq!(hi, 0);
    }
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_33`.
///
/// Scratch need is 5 * xs_len / 2 + 10 * k, where k is the recursion depth. We use 3 * xs_len + C,
/// so that we can use a smaller constant.
///
/// This is mpn_toom33_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_33_scratch_size(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}

pub const TOOM33_MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD < 3 * MUL_TOOM22_THRESHOLD;
pub const TOOM33_MAYBE_MUL_TOOM33: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM44_THRESHOLD >= 3 * MUL_TOOM33_THRESHOLD;

/// A helper function for `_limbs_mul_greater_to_out_toom_33`.
///
/// This is TOOM33_MUL_N_REC from mpn/generic/toom33_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_33_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(xs.len(), n);
    if TOOM33_MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if !TOOM33_MAYBE_MUL_TOOM33 || n < MUL_TOOM33_THRESHOLD {
        _limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else {
        _limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
    }
}

const SMALLER_RECURSION: bool = true;

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_33` are valid.
pub fn _limbs_mul_greater_to_out_toom_33_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = (xs_len + 2) / 3;
    let s = xs_len.saturating_sub(2 * n);
    let t = ys_len.saturating_sub(2 * n);
    0 < s && s <= n && 0 < t && t <= n
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_33_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_33_input_sizes_valid`. The gist is that 2 times
/// `xs.len()` must be less than 3 times `ys.len()`.
///
/// This uses the Toom-33 aka Toom-3 algorithm.
///
/// Evaluate in: -1, 0, +1, +2, +inf
///
/// <--s--><--n--><--n-->
///  ____________________
/// |_xs2_|__xs1_|__xs0_|
///  |ys2_|__ys1_|__ys0_|
///  <-t--><--n--><--n-->
///
/// v0   =  xs0           *  b0                                 # X(0)   * Y(0)
/// v1   = (xs0 +   * xs1 +  a2)    * (ys0 +  ys1+ ys2)         # X(1)   * Y(1)    xh  <= 2, yh <= 2
/// vm1  = (xs0 -   * xs1 +  a2)    * (ys0 -  ys1+ ys2)         # X(-1)  * Y(-1)  |xh| <= 1, yh <= 1
/// v2   = (xs0 + 2 * xs1 + 4 * a2) * (ys0 + 2 * ys1 + 4 * ys2) # X(2)   * Y(2)    xh  <= 6, yh <= 6
/// vinf =            xs2           *  ys2                      # X(inf) * Y(inf)
///
/// Time: TODO (should be something like O(n<sup>log(5)/log(3)</sup>))
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom33_mul from mpn/generic/toom33_mul.c.
#[allow(clippy::cyclomatic_complexity)]
pub fn _limbs_mul_greater_to_out_toom_33(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = (xs_len + 2) / 3;
    split_into_chunks!(xs, n, s, [xs_0, xs_1], xs_2);
    split_into_chunks!(ys, n, t, [ys_0, ys_1], ys_2);

    assert!(ys_len >= 2 * n);
    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    let mut v_neg_1_neg = false;
    {
        split_into_chunks_mut!(out, n + 1, [bs1, as2], bs2);
        {
            // we need 4n+4 <= 4n+s+t
            let (gp, remainder) = scratch.split_at_mut(2 * n + 2);
            let gp = &mut gp[..n]; // gp length: n
            split_into_chunks_mut!(remainder, n + 1, [asm1, bsm1], as1);

            // Compute as1 and asm1.
            let mut carry = if limbs_add_to_out(gp, xs_0, xs_2) {
                1
            } else {
                0
            };
            as1[n] = carry;
            if limbs_add_same_length_to_out(as1, gp, xs_1) {
                as1[n].wrapping_add_assign(1);
            }
            if carry == 0 && limbs_cmp_same_length(gp, xs_1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs_1, gp);
                asm1[n] = 0;
                v_neg_1_neg = true;
            } else {
                if limbs_sub_same_length_to_out(asm1, gp, xs_1) {
                    carry.wrapping_sub_assign(1);
                }
                asm1[n] = carry;
            }

            // Compute as2.
            let mut carry = if limbs_add_same_length_to_out(as2, xs_2, &as1[..s]) {
                1
            } else {
                0
            };
            if s != n {
                carry = if limbs_add_limb_to_out(&mut as2[s..], &as1[s..n], carry) {
                    1
                } else {
                    0
                }
            }
            carry.wrapping_add_assign(as1[n]);
            carry = 2 * carry + limbs_slice_shl_in_place(&mut as2[..n], 1);
            if limbs_sub_same_length_in_place_left(&mut as2[..n], xs_0) {
                carry.wrapping_sub_assign(1);
            }
            as2[n] = carry;
            // Compute bs1 and bsm1.
            let mut carry = if limbs_add_to_out(gp, ys_0, ys_2) {
                1
            } else {
                0
            };
            bs1[n] = carry;
            if limbs_add_same_length_to_out(bs1, gp, ys_1) {
                bs1[n] += 1;
            }
            if carry == 0 && limbs_cmp_same_length(gp, ys_1) == Ordering::Less {
                limbs_sub_same_length_to_out(bsm1, ys_1, gp);
                bsm1[n] = 0;
                v_neg_1_neg.not_assign();
            } else {
                if limbs_sub_same_length_to_out(bsm1, gp, ys_1) {
                    carry.wrapping_sub_assign(1);
                }
                bsm1[n] = carry;
            }

            // Compute bs2.
            let mut carry = 0;
            if limbs_add_same_length_to_out(bs2, &bs1[..t], ys_2) {
                carry = 1;
            }
            if t != n {
                carry = if limbs_add_limb_to_out(&mut bs2[t..], &bs1[t..n], carry) {
                    1
                } else {
                    0
                };
            }
            carry.wrapping_add_assign(bs1[n]);
            carry = 2 * carry + limbs_slice_shl_in_place(&mut bs2[..n], 1);
            if limbs_sub_same_length_in_place_left(&mut bs2[..n], ys_0) {
                carry.wrapping_sub_assign(1);
            }
            bs2[n] = carry;

            assert!(as1[n] <= 2);
            assert!(bs1[n] <= 2);
            assert!(asm1[n] <= 1);
            assert!(bsm1[n] <= 1);
            assert!(as2[n] <= 6);
            assert!(bs2[n] <= 6);
        }
        {
            let (v_neg_1, remainder) = scratch.split_at_mut(2 * n + 2); // v_neg_1 length: 2 * n + 2
            let (asm1, remainder) = remainder.split_at_mut(n + 1); // asm1 length: n + 1
            let (bsm1, scratch_out) = remainder.split_at_mut(2 * n + 2); // bsm1 length: 2 * n + 2
            if SMALLER_RECURSION {
                _limbs_mul_same_length_to_out_toom_33_recursive(
                    v_neg_1,
                    &asm1[..n],
                    &bsm1[..n],
                    scratch_out,
                );
                let v_neg_1 = &mut v_neg_1[n..2 * n + 1];
                let (v_neg_1_last, v_neg_1_init) = v_neg_1.split_last_mut().unwrap();
                let mut carry = 0;
                if asm1[n] != 0 {
                    carry = bsm1[n];
                    if limbs_slice_add_same_length_in_place_left(v_neg_1_init, &bsm1[..n]) {
                        carry += 1;
                    }
                }
                if bsm1[n] != 0
                    && limbs_slice_add_same_length_in_place_left(v_neg_1_init, &asm1[..n])
                {
                    carry += 1;
                }
                *v_neg_1_last = carry;
            } else {
                // this branch not tested
                _limbs_mul_same_length_to_out_toom_33_recursive(
                    v_neg_1,
                    asm1,
                    &bsm1[..n + 1],
                    scratch_out,
                );
            }
        }
        // v_2 length: 3 * n + 4
        let (v_2, scratch_out) = scratch[2 * n + 1..].split_at_mut(3 * n + 4);
        // v_2, 2n+1 limbs
        _limbs_mul_same_length_to_out_toom_33_recursive(v_2, as2, &bs2[..n + 1], scratch_out);
    }
    let v_inf0;
    {
        let v_inf = &mut out[4 * n..];
        // v_inf, s + t limbs
        if s > t {
            limbs_mul_greater_to_out(v_inf, xs_2, ys_2);
        } else {
            _limbs_mul_same_length_to_out_toom_33_recursive(
                v_inf,
                xs_2,
                &ys_2[..s],
                &mut scratch[5 * n + 5..],
            );
        }
        v_inf0 = v_inf[0]; // v1 overlaps with this
    }

    {
        let (bs1, v_1) = out.split_at_mut(2 * n); // bs1 length: 2 * n
        let (as1, scratch_out) = scratch[4 * n + 4..].split_at_mut(n + 1); // as1 length: n + 1
        if SMALLER_RECURSION {
            let (as1_last, as1_init) = as1.split_last_mut().unwrap();

            // v_1, 2 * n + 1 limbs
            _limbs_mul_same_length_to_out_toom_33_recursive(v_1, as1_init, &bs1[..n], scratch_out);
            let mut carry = 0;
            if *as1_last == 1 {
                carry = bs1[n];
                if limbs_slice_add_same_length_in_place_left(&mut v_1[n..2 * n], &bs1[..n]) {
                    carry += 1;
                }
            } else if *as1_last != 0 {
                carry = 2 * bs1[n] + mpn_addmul_1(&mut v_1[n..], &bs1[..n], 2);
            }
            if bs1[n] == 1 {
                if limbs_slice_add_same_length_in_place_left(&mut v_1[n..2 * n], as1_init) {
                    carry += 1;
                }
            } else if bs1[n] != 0 {
                carry += mpn_addmul_1(&mut v_1[n..], as1_init, 2);
            }
            v_1[2 * n] = carry;
        } else {
            // this branch not tested
            let carry = v_1[2 * n + 1];
            _limbs_mul_same_length_to_out_toom_33_recursive(v_1, as1, &bs1[..n + 1], scratch_out);
            v_1[2 * n + 1] = carry;
        }
    }
    // v_0, 2 * n limbs
    _limbs_mul_same_length_to_out_toom_33_recursive(
        out,
        &xs[..n],
        &ys[..n],
        &mut scratch[5 * n + 5..],
    );

    let (v_neg_1, v_2) = scratch.split_at_mut(2 * n + 1); // v_neg_1 length: 2 * n + 1
    _limbs_mul_toom_interpolate_5_points(out, v_2, v_neg_1, n, s + t, v_neg_1_neg, v_inf0);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_42` are valid.
pub fn _limbs_mul_greater_to_out_toom_42_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = if xs_len >= 2 * ys_len {
        (xs_len + 3) >> 2
    } else {
        (ys_len + 1) >> 1
    };
    let s = xs_len.saturating_sub(3 * n);
    let t = ys_len.saturating_sub(n);
    0 < s && s <= n && 0 < t && t <= n
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_42`.
///
/// This is mpn_toom42_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_42_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = if xs_len >= 2 * ys_len {
        (xs_len + 3) >> 2
    } else {
        (ys_len + 1) >> 1
    };
    6 * n + 3
}

/// A helper function for `_limbs_mul_greater_to_out_toom_42`.
///
/// This is TOOM42_MUL_N_REC from mpn/generic/toom42_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_42_recursive(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    limbs_mul_same_length_to_out(out, xs, ys);
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_42_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_42_input_sizes_valid`. The gist is that `xs` must
/// be less than 4 times as long as `ys`.
///
/// This uses the Toom-42 algorithm.
///
/// Evaluate in: -1, 0, +1, +2, +inf
///
/// <-s--><--n---><--n---><--n--->
///  _____________________________
/// |xs3_|__xs2__|__xs1__|__xs0__|
///               |_ys1__|__ys0__|
///               <--t--><---n--->
///
/// v_0     =  xs0                          *  ys0          # X(0)  * Y(0)
/// v_1     = (xs0 +   xs1 +   xs2 +   xs3) * (ys0 + ys1)   # X(1)  * Y(1)   xh  <= 3  yh <= 1
/// v_neg_1 = (xs0 -   xs1 +   xs2 -   xs3) * (ys0 - ys1)   # X(-1) * Y(-1) |xh| <= 1  yh  = 0
/// v_2     = (xs0 + 2*xs1 + 4*xs2 + 8*xs3) * (ys0 + 2*ys1) # X(2)  * Y(2)   xh  <= 14 yh <= 2
/// v_inf   =  xs3 *     b1  # A(inf)*B(inf)
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom42_mul from mpn/generic/toom42_mul.c.
pub fn _limbs_mul_greater_to_out_toom_42(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let n = if xs_len >= 2 * ys_len {
        (xs_len + 3) >> 2
    } else {
        (ys_len + 1) >> 1
    };

    split_into_chunks!(xs, n, s, [xs_0, xs_1, xs_2], xs_3);
    split_into_chunks!(ys, n, t, [ys_0], ys_1);

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    let mut scratch2 = vec![0; 6 * n + 5];
    split_into_chunks_mut!(&mut scratch2, n + 1, [as1, asm1, as2, bs1], remainder);
    let (bsm1, bs2) = remainder.split_at_mut(n); // bsm1 length: n, bs2 length: n + 1

    // Compute as1 and asm1.
    let mut v_neg_1_neg =
        _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(as1, asm1, xs, n, s, &mut out[..n + 1]);

    // Compute as2.
    let mut carry = limbs_shl_to_out(as2, xs_3, 1);
    if limbs_slice_add_same_length_in_place_left(&mut as2[..s], &xs_2[..s]) {
        carry += 1;
    }
    if s != n {
        carry = if limbs_add_limb_to_out(&mut as2[s..], &xs_2[s..], carry) {
            1
        } else {
            0
        };
    }
    carry = 2 * carry + limbs_slice_shl_in_place(&mut as2[..n], 1);
    if limbs_slice_add_same_length_in_place_left(&mut as2[..n], xs_1) {
        carry += 1;
    }
    carry = 2 * carry + limbs_slice_shl_in_place(&mut as2[..n], 1);
    if limbs_slice_add_same_length_in_place_left(&mut as2[..n], xs_0) {
        carry += 1;
    }
    as2[n] = carry;

    // Compute bs1 and bsm1.
    if t == n {
        bs1[n] = if limbs_add_same_length_to_out(bs1, ys_0, ys_1) {
            1
        } else {
            0
        };
        if limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less {
            limbs_sub_same_length_to_out(bsm1, ys_1, ys_0);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(bsm1, ys_0, ys_1);
        }
    } else {
        bs1[n] = if limbs_add_to_out(bs1, ys_0, ys_1) {
            1
        } else {
            0
        };

        if limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less
        {
            limbs_sub_same_length_to_out(bsm1, ys_1, &ys_0[..t]);
            limbs_set_zero(&mut bsm1[t..]);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_to_out(bsm1, ys_0, ys_1);
        }
    }

    // Compute bs2, recycling bs1. bs2 = bs1 + ys_1
    limbs_add_to_out(bs2, bs1, ys_1);

    assert!(as1[n] <= 3);
    assert!(bs1[n] <= 1);
    assert!(asm1[n] <= 1);
    assert!(as2[n] <= 14);
    assert!(bs2[n] <= 2);

    let (v_neg_1, v_2) = scratch.split_at_mut(2 * n + 1); // v_neg_1 length: 2 * n + 1
    let v_inf_0;
    {
        split_into_chunks_mut!(out, 2 * n, [v_0, v_1], v_inf);

        // v_neg_1, 2 * n + 1 limbs
        _limbs_mul_same_length_to_out_toom_42_recursive(v_neg_1, &asm1[..n], bsm1);
        let mut carry = 0;
        if asm1[n] != 0 {
            carry = 0;
            if limbs_slice_add_same_length_in_place_left(&mut v_neg_1[n..2 * n], bsm1) {
                carry = 1;
            }
        }
        v_neg_1[2 * n] = carry;

        // v_2, 2 * n + 1 limbs
        _limbs_mul_same_length_to_out_toom_42_recursive(v_2, as2, bs2);

        // v_inf, s + t limbs
        limbs_mul_to_out(v_inf, xs_3, ys_1);

        v_inf_0 = v_inf[0]; // v_1 overlaps with this

        // v_1, 2 * n + 1 limbs
        _limbs_mul_same_length_to_out_toom_42_recursive(v_1, &as1[..n], &bs1[..n]);
        let v_1 = &mut v_1[n..];
        if as1[n] == 1 {
            carry = bs1[n];
            if limbs_slice_add_same_length_in_place_left(v_1, &bs1[..n]) {
                carry += 1;
            }
        } else if as1[n] == 2 {
            carry = bs1[n]
                .wrapping_mul(2)
                .wrapping_add(mpn_addmul_1(v_1, &bs1[..n], 2));
        } else if as1[n] == 3 {
            carry = bs1[n]
                .wrapping_mul(3)
                .wrapping_add(mpn_addmul_1(v_1, &bs1[..n], 3));
        }
        if bs1[n] != 0 && limbs_slice_add_same_length_in_place_left(v_1, &as1[..n]) {
            carry += 1;
        }
        v_inf[0] = carry;
        // v_0, 2 * n limbs
        _limbs_mul_same_length_to_out_toom_42_recursive(v_0, xs_0, ys_0);
    }
    _limbs_mul_toom_interpolate_5_points(out, v_2, v_neg_1, n, s + t, v_neg_1_neg, v_inf_0);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_43` are valid.
pub fn _limbs_mul_greater_to_out_toom_43_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if 3 * xs_len >= 4 * ys_len {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    let s = xs_len.saturating_sub(3 * n);
    let t = ys_len.saturating_sub(2 * n);
    0 < s && s <= n && 0 < t && t <= n && s + t >= 5
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_43`.
///
/// This is mpn_toom43_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_43_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1 + if 3 * xs_len >= 4 * ys_len {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    6 * n + 4
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_43_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_43_input_sizes_valid`. The gist is that `xs` must
/// be less than twice as long as `ys`.
///
/// This uses the Toom-43 algorithm.
///
/// <-s--><--n--><--n--><--n-->
///  __________________________
/// |xs3_|__xs2_|__xs1_|__xs0_|
///       |_ys2_|__ys1_|__ys0_|
///       <-t--><--n--><--n-->
///
/// v_0     =  xs0                          * ys0                   # X(0) *Y(0)
/// v_1     = (xs0 +   xs1 +   xs2 +   xs3) * (ys0 +   ys1 +   ys2) # X(1) *Y(1)   xh  <= 3  yh <= 2
/// v_neg_1 = (xs0 -   xs1 +   xs2 -   xs3) * (ys0 -   ys1 +   ys2) # X(-1)*Y(-1) |xh| <= 1 |yh|<= 1
/// v_2     = (xs0 + 2*xs1 + 4*xs2 + 8*xs3) * (ys0 + 2*ys1 + 4*ys2) # X(2) *Y(2)   xh  <= 14 yh <= 6
/// v_neg_2 = (xs0 - 2*xs1 + 4*xs2 - 8*xs3) * (ys0 - 2*ys1 + 4*ys2) # X(-2)*Y(-2) |xh| <= 9 |yh|<= 4
/// v_inf   =                          xs3 *                   ys2  # X(inf)*Y(inf)
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom43_mul from mpn/generic/toom43_mul.c.
pub fn _limbs_mul_greater_to_out_toom_43(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let n = 1 + if 3 * xs_len >= 4 * ys_len {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    let xs_3 = &xs[3 * n..];
    let s = xs_3.len();
    split_into_chunks!(ys, n, t, [ys_0, ys_1], ys_2);

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    // This is probably true whenever `xs_len` >= 25 or `ys_len` >= 19, I think. It guarantees that
    // we can fit 5 values of size n + 1 in the product area.
    assert!(s + t >= 5);

    // Total scratch need is 6 * n + 4; we allocate one extra limb, because products will overwrite
    // 2 * n + 2 limbs.
    let m = n + 1;
    let mut v_neg_1_neg = false;
    let mut v_neg_2_neg = false;
    {
        split_into_chunks_mut!(out, m, [bs1, bsm2, bs2, as2], as1);
        let as1 = &mut as1[..m]; // as1 length: n + 1
        {
            split_into_chunks_mut!(&mut scratch[2 * n + 2..], m, [bsm1, asm1], asm2);

            // Compute as2 and asm2.
            if _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(as2, asm2, xs, n, s, asm1) {
                v_neg_2_neg = true;
            }

            // Compute bs2 and bsm2.
            bsm1[n] = limbs_shl_to_out(bsm1, ys_1, 1); // 2 * ys_1
        }
        let mut carry = limbs_shl_to_out(scratch, ys_2, 2); // 4 * ys_2
        if limbs_slice_add_same_length_in_place_left(&mut scratch[..t], &ys_0[..t]) {
            carry += 1;
        }
        // 4 * ys_2 + ys_0
        if t != n {
            carry = if limbs_add_limb_to_out(&mut scratch[t..], &ys_0[t..], carry) {
                1
            } else {
                0
            };
        }
        scratch[n] = carry;

        split_into_chunks_mut!(scratch, m, [small_scratch, _unused, bsm1, asm1], asm2);
        limbs_add_same_length_to_out(bs2, small_scratch, bsm1);
        if limbs_cmp_same_length(small_scratch, bsm1) == Ordering::Less {
            limbs_sub_same_length_to_out(bsm2, bsm1, small_scratch);
            v_neg_2_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(bsm2, small_scratch, bsm1);
        }

        // Compute as1 and asm1.
        if _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(as1, asm1, xs, n, s, small_scratch) {
            v_neg_1_neg = true;
        }

        let (bsm1_last, bsm1_init) = bsm1.split_last_mut().unwrap();
        // Compute bs1 and bsm1.
        *bsm1_last = if limbs_add_to_out(bsm1_init, ys_0, ys_2) {
            1
        } else {
            0
        };
        bs1[n] = *bsm1_last;
        if limbs_add_same_length_to_out(bs1, bsm1_init, ys_1) {
            bs1[n] += 1;
        }
        if *bsm1_last == 0 && limbs_cmp_same_length(bsm1_init, ys_1) == Ordering::Less {
            limbs_sub_same_length_in_place_right(ys_1, bsm1_init);
            v_neg_1_neg.not_assign();
        } else if limbs_sub_same_length_in_place_left(bsm1_init, ys_1) {
            bsm1_last.wrapping_sub_assign(1);
        }

        assert!(as1[n] <= 3);
        assert!(bs1[n] <= 2);
        assert!(asm1[n] <= 1);
        assert!(*bsm1_last <= 1);
        assert!(as2[n] <= 14);
        assert!(bs2[n] <= 6);
        assert!(asm2[n] <= 9);
        assert!(bsm2[n] <= 4);
    }

    {
        let (v_neg_1, remainder) = scratch.split_at_mut(2 * m); // v_neg_1 length: 2 * n + 2
        let (bsm1, asm1) = remainder.split_at_mut(m); // bsm1 length: m
                                                      // v_neg_1, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(v_neg_1, &asm1[..m], bsm1); // W4
    }
    {
        // v_neg_2 length: 2 * n + 3
        let (v_neg_2, asm2) = scratch[2 * n + 1..].split_at_mut(2 * n + 3);
        // v_neg_2, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(v_neg_2, &asm2[..m], &out[m..2 * m]); // W2
    }
    {
        let (bs2, as2) = out[2 * m..].split_at_mut(m); // bs2 length: n + 1
                                                       // v_neg_2, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(&mut scratch[4 * n + 2..], &as2[..m], bs2); // W1
    }
    {
        let (bs1, remainder) = out.split_at_mut(2 * n); // bs1 length: 2 * n
        let (v_1, as1) = remainder.split_at_mut(2 * n + 4); // v_1 length: 2 * n + 4
                                                            // v_1, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(v_1, &as1[..m], &bs1[..m]); // W3
    }
    {
        let v_inf = &mut out[5 * n..];
        // v_inf, s + t limbs // W0
        limbs_mul_to_out(v_inf, xs_3, ys_2);
    }

    // v_0, 2 * n limbs
    limbs_mul_same_length_to_out(out, &xs[..n], ys_0); // W5
    split_into_chunks_mut!(scratch, 2 * n + 1, [v_neg_1, v_neg_2], v_2);
    _limbs_mul_toom_interpolate_6_points(
        out,
        n,
        t + s,
        v_neg_1_neg,
        v_neg_1,
        v_neg_2_neg,
        v_neg_2,
        v_2,
    );
}

//TODO tune
const TOOM44_MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM44_THRESHOLD < 4 * MUL_TOOM22_THRESHOLD;
const TOOM44_MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM44_THRESHOLD < 4 * MUL_TOOM33_THRESHOLD;
const TOOM44_MAYBE_MUL_TOOM44: bool =
    TUNE_PROGRAM_BUILD || MUL_TOOM6H_THRESHOLD >= 4 * MUL_TOOM44_THRESHOLD;

/// A helper function for `_limbs_mul_greater_to_out_toom_44`.
///
/// This is TOOM44_MUL_N_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_same_length_to_out_toom_44_recursive(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    if TOOM44_MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if TOOM44_MAYBE_MUL_TOOM22 && n < MUL_TOOM33_THRESHOLD {
        _limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if !TOOM44_MAYBE_MUL_TOOM44 || n < MUL_TOOM44_THRESHOLD {
        _limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
    } else {
        _limbs_mul_greater_to_out_toom_44(out, xs, ys, scratch);
    }
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_44` are valid.
pub fn _limbs_mul_greater_to_out_toom_44_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = (xs_len + 3) >> 2;
    let s = xs_len.saturating_sub(3 * n);
    let t = ys_len.saturating_sub(3 * n);
    0 < s && s <= n && 0 < t && t <= n && s >= t
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_44`.
///
/// This is mpn_toom44_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_44_scratch_size(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_44_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_44_input_sizes_valid`. The gist is that 3 times
///// `xs.len()` must be less than 4 times `ys.len()`.
///
/// This uses the Toom-44 algorithm.
///
/// Evaluate in: 0, +1, -1, +2, -2, 1/2, +inf
///
/// <-s--><--n--><--n--><--n-->
///  __________________________
/// |_xs3|__xs2_|__xs1_|__xs0_|
///  |ys3|__ys2_|__ys1_|__ys0_|
///  <-t-><--n--><--n--><--n-->
///
/// v_0     =   x0             *  y0              #   X(0)*Y(0)
/// v_1     = ( x0+ x1+ x2+ x3)*( y0+ y1+ y2+ y3) #   X(1)*Y(1)    xh  <= 3   yh  <= 3
/// v_neg_1 = ( x0- x1+ x2- x3)*( y0- y1+ y2- y3) #  X(-1)*Y(-1)  |xh| <= 1  |yh| <= 1
/// v_2     = ( x0+2x1+4x2+8x3)*( y0+2y1+4y2+8y3) #   X(2)*Y(2)    xh  <= 14  yh  <= 14
/// v_neg_2 = ( x0-2x1+4x2-8x3)*( y0-2y1+4y2-8y3) #   X(2)*Y(2)    xh  <= 9  |yh| <= 9
/// v_half  = (8x0+4x1+2x2+ x3)*(8y0+4y1+2y2+ y3) # X(1/2)*Y(1/2)  xh  <= 14  yh  <= 14
/// v_inf   =               x3 *          y2      # X(inf)*Y(inf)
///
/// Use of scratch space: In the product area, we store
///    _______________________
///   |v_inf|____|_v_1_|_v_0_|
///    s+t   2n-1  2n+1  2n
///
/// The other recursive products, v_neg_1, v_2, v_neg_2, and v_half, are stored in the scratch area.
/// When computing them, we use the product area for intermediate values.
///
/// Next, we compute v_1. We can store the intermediate factors at v_0 and at v_half + 2 * n + 2.
///
/// Finally, for v_0 and v_inf, factors are parts of the input operands, and we need scratch space
/// only for the recursive multiplication.
///
/// In all, if S(xs_len) is the scratch need, the needed space is bounded by
///
/// S(xs_len) <= 4 (2 * ceil(xs_len / 4) + 1) + 1 + S(ceil(xs_len / 4) + 1)
///
/// which should give S(n) = 8 * n / 3 + c * log(n) for some constant c.
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom44_mul from mpn/generic/toom44_mul.c.
pub fn _limbs_mul_greater_to_out_toom_44(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = (xs_len + 3) >> 2;
    let m = 2 * n + 1;
    split_into_chunks!(xs, n, s, [xs_0, xs_1, xs_2], xs_3);
    split_into_chunks!(ys, n, t, [ys_0, ys_1, ys_2], ys_3);

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    assert!(s >= t);

    // NOTE: The multiplications to v_2, v_neg_2, v_half, and v_neg_1 overwrites the following limb,
    // so these must be computed in order, and we need a one limb gap to scratch2.
    let mut w1_neg;
    let mut w3_neg;
    {
        // apx and bpx must not overlap with v1
        split_into_chunks_mut!(out, n + 1, [apx, amx], remainder);
        let (bmx, bpx) = remainder.split_at_mut(2 * n);
        let bmx = &mut bmx[..n + 1];
        let bpx = &mut bpx[..n + 1];

        // Total scratch need: 8 * n + 5 + scratch for recursive calls. This gives roughly
        // 32 * n / 3 + log term.
        {
            let (v_2, scratch2) = scratch.split_at_mut(8 * n + 5); // v_2 length: 8 * n + 5

            // Compute apx = xs_0 + 2 * xs_1 + 4 * xs_2 + 8 xs_3 and
            // amx = xs_0 - 2 * xs_1 + 4 * xs_2 - 8 * xs_3.
            w1_neg = _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(
                &mut apx[..n + 1],
                amx,
                xs,
                n,
                s,
                &mut scratch2[..n + 1],
            );

            // Compute bpx = ys_0 + 2 * ys_1 + 4 * ys_2 + 8 * ys_3 and
            // bmx = ys_0 - 2 * ys_1 + 4 * ys_2 - 8 * ys_3.
            if _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(
                bpx,
                bmx,
                ys,
                n,
                t,
                &mut scratch2[..n + 1],
            ) {
                w1_neg.not_assign();
            }

            // v_2, 2 * n + 1 limbs
            _limbs_mul_same_length_to_out_toom_44_recursive(v_2, &apx[..n + 1], bpx, scratch2);
        }
        {
            // v_neg_2 length: 6 * n + 4
            let (v_neg_2, scratch2) = scratch[m..].split_at_mut(6 * n + 4);
            _limbs_mul_same_length_to_out_toom_44_recursive(v_neg_2, amx, bmx, scratch2);

            // Compute apx = 8 * xs_0 + 4 * xs_1 + 2 * xs_2 + xs_3 =
            // (((2 * xs_0 + xs_1) * 2 + xs_2) * 2 + xs_3
            let (apx_last, apx_init) = apx.split_last_mut().unwrap();
            let mut carry = limbs_shl_to_out(apx_init, xs_0, 1);
            if limbs_slice_add_same_length_in_place_left(apx_init, xs_1) {
                carry.wrapping_add_assign(1);
            }
            carry = 2 * carry + limbs_slice_shl_in_place(apx_init, 1);
            if limbs_slice_add_same_length_in_place_left(apx_init, xs_2) {
                carry.wrapping_add_assign(1);
            }
            carry = 2 * carry + limbs_slice_shl_in_place(apx_init, 1);
            *apx_last = carry;
            if limbs_slice_add_greater_in_place_left(apx_init, xs_3) {
                apx_last.wrapping_add_assign(1);
            }

            // Compute bpx = 8 ys_0 + 4 ys_1 + 2 ys_2 + ys_3 =
            // (((2*ys_0 + ys_1) * 2 + ys_2) * 2 + ys_3
            let (bpx_last, bpx_init) = bpx.split_last_mut().unwrap();
            let mut carry = limbs_shl_to_out(bpx_init, ys_0, 1);
            if limbs_slice_add_same_length_in_place_left(bpx_init, ys_1) {
                carry.wrapping_add_assign(1);
            }
            carry = 2 * carry + limbs_slice_shl_in_place(bpx_init, 1);
            if limbs_slice_add_same_length_in_place_left(bpx_init, ys_2) {
                carry.wrapping_add_assign(1);
            }
            carry = 2 * carry + limbs_slice_shl_in_place(bpx_init, 1);
            *bpx_last = carry;
            if limbs_slice_add_greater_in_place_left(bpx_init, ys_3) {
                bpx_last.wrapping_add_assign(1);
            }

            assert!(*apx_last < 15);
            assert!(*bpx_last < 15);
        }
        {
            // v_half length: 4 * n + 3
            let (v_half, scratch2) = scratch[2 * m..].split_at_mut(4 * n + 3);

            // v_half, 2 * n + 1 limbs
            _limbs_mul_same_length_to_out_toom_44_recursive(v_half, apx, bpx, scratch2);

            // Compute apx = xs_0 + xs_1 + xs_2 + xs_3 and amx = xs_0 - xs_1 + xs_2 - xs_3.
            w3_neg = _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
                apx,
                amx,
                xs,
                n,
                s,
                &mut scratch2[..n + 1],
            );

            // Compute bpx = ys_0 + ys_1 + ys_2 + ys_3 and bmx = ys_0 - ys_1 + ys_2 - ys_3.
            if _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
                bpx,
                bmx,
                ys,
                n,
                t,
                &mut scratch2[..n + 1],
            ) {
                w3_neg.not_assign();
            }
        }
        // v_neg_1 length: m + 1
        let (v_neg_1, scratch2) = scratch[3 * m..].split_at_mut(m + 1);

        // v_neg_1, 2 * n + 1 limbs
        _limbs_mul_same_length_to_out_toom_44_recursive(v_neg_1, amx, bmx, scratch2);
    }
    {
        let scratch2 = &mut scratch[8 * n + 5..];
        {
            let (apx, remainder) = out.split_at_mut(2 * n); // apx length: 2 * n
            let (v_1, bpx) = remainder.split_at_mut(m + 1); // v_1 length: m + 1

            // Clobbers amx, bmx.
            // v_1, 2 * n + 1 limbs
            _limbs_mul_same_length_to_out_toom_44_recursive(
                v_1,
                &apx[..n + 1],
                &bpx[..n + 1],
                scratch2,
            );
        }
        {
            let (v_0, v_inf) = out.split_at_mut(2 * n); // v_0 length: 2 * n
            let v_inf = &mut v_inf[4 * n..];
            _limbs_mul_same_length_to_out_toom_44_recursive(v_0, xs_0, ys_0, scratch2);
            if s > t {
                limbs_mul_greater_to_out(v_inf, xs_3, ys_3);
            } else {
                // v_inf, s + t limbs
                _limbs_mul_same_length_to_out_toom_44_recursive(v_inf, xs_3, ys_3, scratch2);
            }
        }
    }
    split_into_chunks_mut!(scratch, m, [v_2, v_neg_2, v_half], remainder);
    let (v_neg_1, scratch2) = remainder.split_at_mut(m + 1);
    let v_neg_1 = &mut v_neg_1[..m];
    _limbs_mul_toom_interpolate_7_points(
        out,
        n,
        s + t,
        w1_neg,
        v_neg_2,
        w3_neg,
        v_neg_1,
        v_2,
        v_half,
        scratch2,
    );
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_52` are valid.
pub fn _limbs_mul_greater_to_out_toom_52_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if xs_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if 2 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) >> 1
    };
    let s = xs_len.saturating_sub(n << 2);
    let t = ys_len.saturating_sub(n);
    0 < s && s <= n && 0 < t && t <= n && s + t >= 5
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_52`.
///
/// This is mpn_toom52_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_52_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1 + if 2 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) >> 1
    };
    6 * n + 4
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_52_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_52_input_sizes_valid`. The gist is that `xs` must
/// be less than five times as long as `ys`.
///
/// This uses the Toom-52 algorithm.
///
/// Evaluate in: -2, -1, 0, +1, +2, +inf
///
/// <-s--><--n--><--n--><--n--><--n-->
///  _________________________________
/// |xs4_|__xs3_|__xs2_|__xs1_|__xs0_|
///                       |ys1|__ys0_|
///                       <-t-><--n-->
///
/// v_0     =  xs0                               * ys0          # X(0)   * Y(0)
/// v_1     = (xs0 +  xs1 +  xs2 +  xs3 +   xs4) * (ys0 +  ys1) # X(1)   * Y(1)   xh  <= 4   yh <= 1
/// v_neg_1 = (xs0 -  xs1 +  xs2 -  xs3 +   xs4) * (ys0 -  ys1) # X(-1)  * Y(-1) |xh| <= 2   yh  = 0
/// v_2     = (xs0 + 2xs1 + 4xs2 + 8xs3 + 16xs4) * (ys0 + 2ys1) # X(2)   * Y(2)   xh  <= 30  yh <= 2
/// v_neg_2 = (xs0 - 2xs1 + 4xs2 - 8xs3 + 16xs4) * (ys0 - 2ys1) # X(-2)  * Y(-2) |xh| <= 20 |yh|<= 1
/// v_inf   =                               xs4  *         ys1  # X(inf) * Y(inf)
///
/// Some slight optimization in evaluation are taken from the paper: "Towards Optimal Toom-Cook
/// Multiplication for Univariate and Multivariate Polynomials in Characteristic 2 and 0."
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom52_mul from mpn/generic/toom52_mul.c.
pub fn _limbs_mul_greater_to_out_toom_52(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = 1 + if 2 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) >> 1
    };
    split_into_chunks!(ys, n, t, [ys_0], ys_1);

    let s = xs_len - (n << 2);
    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    // Ensures that 5 values of n + 1 limbs each fits in the product area. Borderline cases are
    // xs_len = 32, ys_len = 8, n = 7, and xs_len = 36, ys_len = 9, n = 8.
    assert!(s + t >= 5);

    // Scratch need is 6 * n + 4. We need one extra limb, because products will overwrite 2 * n + 2
    // limbs.
    // Compute as2 and asm2.
    let mut v_neg_1_neg = false;
    let mut v_neg_2_neg;
    let m = n + 1;
    {
        split_into_chunks_mut!(out, m, [bs1, bsm2, bs2, as2, as1], _unused);
        {
            let (v_neg_1, scratch_hi) = scratch.split_at_mut(2 * n + 2);
            split_into_chunks_mut!(scratch_hi, m, [bsm1, asm1, asm2], _unused);
            let bsm1 = &mut bsm1[..n];
            v_neg_2_neg =
                _limbs_mul_toom_evaluate_poly_in_2_and_neg_2(as2, asm2, 4, xs, n, s, asm1);

            // Compute bs1 and bsm1.
            bs1[n] = 0;
            if t == n {
                if limbs_add_same_length_to_out(bs1, ys_0, ys_1) {
                    bs1[n] = 1;
                }
                if limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less {
                    limbs_sub_same_length_to_out(bsm1, ys_1, ys_0);
                    v_neg_1_neg = true;
                } else {
                    limbs_sub_same_length_to_out(bsm1, ys_0, ys_1);
                }
            } else {
                if limbs_add_to_out(bs1, ys_0, ys_1) {
                    bs1[n] = 1;
                }
                if limbs_test_zero(&ys_0[t..])
                    && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less
                {
                    limbs_sub_same_length_to_out(bsm1, ys_1, &ys_0[..t]);
                    limbs_set_zero(&mut bsm1[t..]);
                    v_neg_1_neg.not_assign();
                } else {
                    limbs_sub_to_out(bsm1, ys_0, ys_1);
                }
            }

            // Compute bs2 and bsm2, recycling bs1 and bsm1. bs2=bs1+ys_1; bsm2=bsm1-ys_1
            limbs_add_to_out(bs2, bs1, ys_1);
            let (bsm2_last, bsm2_init) = bsm2.split_last_mut().unwrap();
            *bsm2_last = 0;
            if v_neg_1_neg {
                if limbs_add_to_out(bsm2_init, bsm1, ys_1) {
                    *bsm2_last = 1;
                }
                v_neg_2_neg.not_assign();
            } else if t == n {
                if limbs_cmp_same_length(bsm1, ys_1) == Ordering::Less {
                    limbs_sub_same_length_to_out(bsm2_init, ys_1, bsm1);
                    v_neg_2_neg.not_assign();
                } else {
                    limbs_sub_same_length_to_out(bsm2_init, bsm1, ys_1);
                }
            } else if limbs_test_zero(&bsm1[t..])
                && limbs_cmp_same_length(&bsm1[..t], ys_1) == Ordering::Less
            {
                limbs_sub_same_length_to_out(bsm2_init, ys_1, &bsm1[..t]);
                limbs_set_zero(&mut bsm2_init[t..]);
                v_neg_2_neg.not_assign();
            } else {
                limbs_sub_to_out(bsm2_init, bsm1, ys_1);
            }

            // Compute as1 and asm1.
            if _limbs_mul_toom_evaluate_poly_in_1_and_neg_1(
                as1,
                asm1,
                4,
                xs,
                n,
                s,
                &mut v_neg_1[..m],
                xs,
                ys,
            ) {
                v_neg_1_neg.not_assign();
            }

            assert!(as1[n] <= 4);
            assert!(bs1[n] <= 1);
            assert!(asm1[n] <= 2);
            assert!(as2[n] <= 30);
            assert!(bs2[n] <= 2);
            assert!(asm2[n] <= 20);
            assert!(*bsm2_last <= 1);

            // v_neg_1, 2 * n + 1 limbs
            limbs_mul_greater_to_out(v_neg_1, asm1, bsm1); // W4
        }
        {
            let (v_neg_2, asm2) = scratch[2 * n + 1..].split_at_mut(2 * n + 3);
            // v_neg_2, 2n+1 limbs
            limbs_mul_same_length_to_out(v_neg_2, &asm2[..m], bsm2); // W2
        }

        // v_2, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(&mut scratch[4 * n + 2..], as2, bs2); // W1
    }
    {
        let (bs1, remainder) = out.split_at_mut(2 * n);
        let (v_1, as1) = remainder.split_at_mut(2 * n + 4);
        // v_1, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(v_1, &as1[..m], &bs1[..m]); // W3
    }
    {
        let (v_0, v_inf) = out.split_at_mut(5 * n);
        // v_inf, s + t limbs
        // W0
        limbs_mul_to_out(v_inf, &xs[4 * n..], ys_1);

        // v_0, 2 * n limbs
        limbs_mul_same_length_to_out(v_0, &xs[..n], ys_0); // W5
    }

    split_into_chunks_mut!(scratch, 2 * n + 1, [v_neg_1, v_neg_2], v_2);
    _limbs_mul_toom_interpolate_6_points(
        out,
        n,
        t + s,
        v_neg_1_neg,
        v_neg_1,
        v_neg_2_neg,
        v_neg_2,
        v_2,
    );
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_53` are valid.
pub fn _limbs_mul_greater_to_out_toom_53_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if xs_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if 3 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) / 3
    };
    let s = xs_len.saturating_sub(n << 2);
    let t = ys_len.saturating_sub(n << 1);
    0 < s && s <= n && 0 < t && t <= n
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_53`.
///
/// This is mpn_toom53_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_53_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1 + if 3 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) / 3
    };
    10 * (n + 1)
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_53_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_53_input_sizes_valid`. The gist is that 2 times
///   `xs.len()` must be less than 5 times `ys.len()`.
///
/// This uses the Toom-53 algorithm.
///
///  Evaluate in: 0, +1, -1, +2, -2, 1/2, +inf
///
/// <-s-><--n--><--n--><--n--><--n-->
///  _________________________________
/// |xs4_|__xs3_|__xs2_|__xs1_|__xs0_|
///               |ys2_|__ys1_|__ys0_|
///               <-t--><--n--><--n-->
///
/// v_0  =       x0                   *   y0          #    X(0) * Y(0)
/// v_1  =    (  x0+ x1+ x2+ x3+  x4) * ( y0+ y1+ y2) #    X(1) * Y(1)      xh  <= 4      yh <= 2
/// v_neg_1 = (  x0- x1+ x2- x3+  x4) * ( y0- y1+ y2) #   X(-1) * Y(-1)    |xh| <= 2      yh <= 1
/// v_2  =    (  x0+2x1+4x2+8x3+16x4) * ( y0+2y1+4y2) #    X(2) * Y(2)      xh  <= 30     yh <= 6
/// v_neg_2 = (  x0-2x1+4x2-8x3+16x4) * ( y0-2y1+4y2) #    X(2) * Y(2)    -9<=xh<=20  -1<=yh <= 4
/// v_half  = (16x0+8x1+4x2+2x3+  x4) * (4y0+2y1+ y2) #  X(1/2) * Y(1/2)    xh  <= 30     yh <= 6
/// v_inf=                        x4  *           y2  #  X(inf) * Y(inf)
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom53_mul from mpn/generic/toom53_mul.c.
pub fn _limbs_mul_greater_to_out_toom_53(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = 1 + if 3 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) / 3
    };
    split_into_chunks!(xs, n, s, [xs_0, xs_1, xs_2, xs_3], xs_4);
    split_into_chunks!(ys, n, t, [ys_0, ys_1], ys_2);
    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    let mut scratch2 = vec![0; 10 * (n + 1)];
    split_into_chunks_mut!(
        scratch2,
        n + 1,
        [as1, asm1, as2, asm2, ash, bs1, bsm1, bs2, bsm2, bsh],
        _unused
    );

    // Compute as1 and asm1.
    let mut v_neg_1_neg = _limbs_mul_toom_evaluate_poly_in_1_and_neg_1(
        as1,
        asm1,
        4,
        xs,
        n,
        s,
        &mut out[..n + 1],
        xs,
        ys,
    );

    // Compute as2 and asm2.
    let mut v_neg_2_neg = _limbs_mul_toom_evaluate_poly_in_2_and_neg_2(as2, asm2, 4, xs, n, s, out);

    // Compute ash = 16 * xs_0 + 8 * xs_1 + 4 * xs_2 + 2 * xs_3 + xs_4 =
    //      2 * (2 * (2 * (2 * xs_0 + xs_1) + xs_2) + xs_3) + xs_4
    {
        let (ash_last, ash_init) = ash.split_last_mut().unwrap();
        let mut carry = limbs_shl_to_out(ash_init, xs_0, 1);
        if limbs_slice_add_same_length_in_place_left(ash_init, xs_1) {
            carry += 1;
        }
        carry = (carry << 1) + limbs_slice_shl_in_place(ash_init, 1);
        if limbs_slice_add_same_length_in_place_left(ash_init, xs_2) {
            carry += 1;
        }
        carry = (carry << 1) + limbs_slice_shl_in_place(ash_init, 1);
        if limbs_slice_add_same_length_in_place_left(ash_init, xs_3) {
            carry += 1;
        }
        carry = (carry << 1) + limbs_slice_shl_in_place(ash_init, 1);
        if limbs_slice_add_greater_in_place_left(ash_init, xs_4) {
            carry += 1;
        }
        *ash_last = carry;
    }

    // Compute bs1 and bsm1.
    {
        let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
        // ys_0 + ys_2
        *bs1_last = if limbs_add_to_out(bs1_init, ys_0, ys_2) {
            1
        } else {
            0
        };
        if *bs1_last == 0 && limbs_cmp_same_length(bs1_init, ys_1) == Ordering::Less {
            limbs_sub_same_length_to_out(bsm1, ys_1, bs1_init);
            bsm1[n] = 0;
            v_neg_1_neg.not_assign();
        } else {
            bsm1[n] = *bs1_last;
            if limbs_sub_same_length_to_out(bsm1, bs1_init, ys_1) {
                bsm1[n].wrapping_sub_assign(1);
            }
        }
        // ys_0 + ys_1 + ys_2
        if limbs_slice_add_same_length_in_place_left(bs1_init, ys_1) {
            bs1_last.wrapping_add_assign(1);
        }
    }

    // Compute bs2 and bsm2.
    let cy = limbs_shl_to_out(out, ys_2, 2);
    bs2[n] = if limbs_add_to_out(bs2, ys_0, &out[..t]) {
        1
    } else {
        0
    };
    assert!(!limbs_slice_add_limb_in_place(&mut bs2[t..], cy));

    out[n] = limbs_shl_to_out(out, ys_1, 1);

    if limbs_cmp_same_length(bs2, &out[..n + 1]) == Ordering::Less {
        assert!(!limbs_sub_same_length_to_out(bsm2, &out[..n + 1], bs2));
        v_neg_2_neg.not_assign();
    } else {
        assert!(!limbs_sub_same_length_to_out(bsm2, bs2, &out[..n + 1]));
    }
    limbs_slice_add_same_length_in_place_left(bs2, &out[..n + 1]);

    // Compute bsh = 4 * ys_0 + 2 * ys_1 + ys_2 = 2 * (2 * ys_0 + ys_1) + ys_2.
    {
        let (bsh_last, bsh_init) = bsh.split_last_mut().unwrap();
        let mut carry = limbs_shl_to_out(bsh_init, ys_0, 1);
        if limbs_slice_add_same_length_in_place_left(bsh_init, ys_1) {
            carry += 1;
        }
        carry = (carry << 1) + limbs_slice_shl_in_place(bsh_init, 1);
        if limbs_slice_add_greater_in_place_left(bsh_init, ys_2) {
            carry += 1;
        }
        *bsh_last = carry
    }
    assert!(as1[n] <= 4);
    assert!(bs1[n] <= 2);
    assert!(asm1[n] <= 2);
    assert!(bsm1[n] <= 1);
    assert!(as2[n] <= 30);
    assert!(bs2[n] <= 6);
    assert!(asm2[n] <= 20);
    assert!(bsm2[n] <= 4);
    assert!(ash[n] <= 30);
    assert!(bsh[n] <= 6);
    {
        let (v_0, remainder) = out.split_at_mut(2 * n); // v_0 length: 2 * n
        let (v_1, v_inf) = remainder.split_at_mut(4 * n); // v_1 length: 4 * n

        // Total scratch need: 10 * n + 5
        // Must be in allocation order, as they overwrite one limb beyond 2 * n + 1.

        // v_2, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(scratch, as2, bs2);

        // v_neg_2, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(&mut scratch[2 * n + 1..], asm2, bsm2);

        // v_half, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(&mut scratch[4 * n + 2..], ash, &bsh[..n + 1]);

        let v_neg_1 = &mut scratch[6 * n + 3..8 * n + 4];
        // v_neg_1, 2 * n + 1 limbs
        if SMALLER_RECURSION {
            let (asm1_last, asm1_init) = asm1.split_last_mut().unwrap();
            let (bsm1_last, bsm1_init) = bsm1.split_last_mut().unwrap();
            let (v_neg_1_last, v_neg_1_init) = v_neg_1.split_last_mut().unwrap();
            limbs_mul_same_length_to_out(v_neg_1_init, asm1_init, bsm1_init);
            let mut carry = 0;
            {
                let v_neg_1_init_hi = &mut v_neg_1_init[n..];
                match *asm1_last {
                    1 => {
                        carry = *bsm1_last;
                        if limbs_slice_add_same_length_in_place_left(v_neg_1_init_hi, bsm1_init) {
                            carry.wrapping_add_assign(1)
                        }
                    }
                    2 => carry = (*bsm1_last << 1) + mpn_addmul_1(v_neg_1_init_hi, bsm1_init, 2),
                    _ => {}
                }
                if *bsm1_last != 0
                    && limbs_slice_add_same_length_in_place_left(v_neg_1_init_hi, asm1_init)
                {
                    carry.wrapping_add_assign(1);
                }
            }
            *v_neg_1_last = carry;
        } else {
            // this branch not tested
            v_neg_1[2 * n] = 0;
            if (asm1[n] | bsm1[n]) == 0 {
                limbs_mul_same_length_to_out(v_neg_1, &asm1[..n], &bsm1[..n]);
            } else {
                limbs_mul_same_length_to_out(v_neg_1, asm1, bsm1);
            }
        }

        // v_1, 2 * n + 1 limbs
        if SMALLER_RECURSION {
            let (as1_last, as1_init) = as1.split_last_mut().unwrap();
            let (bs1_last, bs1_init) = bs1.split_last_mut().unwrap();
            limbs_mul_same_length_to_out(v_1, as1_init, bs1_init);
            let mut carry = 0;
            match *as1_last {
                1 => {
                    carry = *bs1_last;
                    if limbs_slice_add_same_length_in_place_left(&mut v_1[n..2 * n], bs1_init) {
                        carry.wrapping_add_assign(1);
                    }
                }
                2 => carry = (*bs1_last << 1) + mpn_addmul_1(&mut v_1[n..], bs1_init, 2),
                0 => {}
                _ => {
                    carry = as1_last.wrapping_mul(*bs1_last)
                        + mpn_addmul_1(&mut v_1[n..], bs1_init, *as1_last)
                }
            }
            if *bs1_last == 1
                && limbs_slice_add_same_length_in_place_left(&mut v_1[n..2 * n], as1_init)
            {
                carry.wrapping_add_assign(1);
            } else if *bs1_last == 2 {
                carry += mpn_addmul_1(&mut v_1[n..2 * n], as1_init, 2);
            }
            v_1[2 * n] = carry;
        } else {
            // this branch not tested
            v_1[2 * n] = 0;
            if (as1[n] | bs1[n]) == 0 {
                limbs_mul_same_length_to_out(v_1, &as1[..n], &bs1[..n]);
            } else {
                limbs_mul_same_length_to_out(v_1, as1, bs1);
            }
        }
        limbs_mul_same_length_to_out(v_0, xs_0, ys_0); // v_0, 2 * n limbs
        limbs_mul_to_out(v_inf, xs_4, ys_2); // v_inf, s + t limbs
    }
    split_into_chunks_mut!(
        scratch,
        2 * n + 1,
        [v_2, v_neg_2, v_half, v_neg_1, scratch_out],
        _unused
    );
    _limbs_mul_toom_interpolate_7_points(
        out,
        n,
        s + t,
        v_neg_2_neg,
        v_neg_2,
        v_neg_1_neg,
        v_neg_1,
        v_2,
        v_half,
        scratch_out,
    );
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_54` are valid.
pub fn _limbs_mul_greater_to_out_toom_54_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if xs_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if 4 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) / 4
    };
    let s = xs_len.saturating_sub(n << 2);
    let t = ys_len.saturating_sub(3 * n);
    0 < s && s <= n && 0 < t && t <= n && s + t >= n && s + t > 4 && n > 2
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_54`.
///
/// This is mpn_toom54_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_54_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1
        + (if 4 * xs_len >= 5 * ys_len {
            (xs_len - 1) / 5
        } else {
            (ys_len - 1) / 4
        });
    9 * n + 3
}

/// A helper function for `_limbs_mul_greater_to_out_toom_54`.
///
/// This is TOOM_54_MUL_N_REC from from mpn/generic/toom54_mul.c.
fn _limbs_mul_same_length_to_out_toom_54_recursive(p: &mut [Limb], a: &[Limb], b: &[Limb]) {
    limbs_mul_same_length_to_out(p, a, b);
}

/// A helper function for `_limbs_mul_greater_to_out_toom_54`.
///
/// This is TOOM_54_MUL_REC from from mpn/generic/toom54_mul.c.
fn _limbs_mul_to_out_toom_54_recursive(p: &mut [Limb], a: &[Limb], b: &[Limb]) {
    limbs_mul_greater_to_out(p, a, b);
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_greater_to_out_toom_54_scratch_size`. The
/// following restrictions on the input slices must be met:
/// 1. `out`.len() >= `xs`.len() + `ys`.len()
/// 2. `xs`.len() >= `ys`.len()
/// 3. Others; see `_limbs_mul_greater_to_out_toom_54_input_sizes_valid`. The gist is that 2 times
///  `xs.len()` must be less than 5 times `ys.len()`.
///
/// This uses the Toom-54 algorithm (the splitting unbalanced version).
///
/// Evaluate in: infinity, +4, -4, +2, -2, +1, -1, 0.
///
/// <--s-><--n--><--n--><--n--><--n-->
///  _________________________________
/// |xs4_|_xs3__|_xs2__|_xs1__|_xs0__|
///        |ys3_|_ys2__|_ys1__|_ys0__|
///         <-t-><--n--><--n--><--n-->
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom54_mul from mpn/generic/toom54_mul.c.
pub fn _limbs_mul_greater_to_out_toom_54(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    // Decomposition
    let n = 1 + if 4 * xs_len >= 5 * ys_len {
        (xs_len - 1) / 5
    } else {
        (ys_len - 1) / 4
    };
    let a4 = &xs[4 * n..]; // a4 length: s
    let b3 = &ys[3 * n..]; // b3 length: t

    let s = xs_len - (n << 2);
    let t = ys_len - 3 * n;

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    assert!(s + t >= n);
    assert!(s + t > 4);
    assert!(n > 2);

    let neg_2_sign;
    // Also allocate 3 * `n` + 1 limbs for `scratch_hi`. `_limbs_mul_toom_interpolate_8_points` may
    // need all of them, when `shl_and_sub_same_length` uses a scratch.
    split_into_chunks_mut!(scratch, 3 * n + 1, [r7, r3], scratch_hi);
    {
        let (out_lo, out_hi) = out.split_at_mut(3 * n);
        split_into_chunks_mut!(out_hi, n + 1, [v0, v1, v2, v3], _unused);

        // Evaluation and recursive calls
        // 4, -4
        let neg_2_pow_sign = _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(
            v2,
            v0,
            4,
            xs,
            n,
            s,
            2,
            &mut out_lo[..n + 1],
        ) ^ _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(
            v3,
            v1,
            3,
            ys,
            n,
            t,
            2,
            &mut out_lo[..n + 1],
        );
        // X(-4) * Y(-4)
        _limbs_mul_same_length_to_out_toom_54_recursive(out_lo, v0, v1);
        // X(+4) * Y(+4)
        _limbs_mul_same_length_to_out_toom_54_recursive(r3, v2, v3);
        _limbs_toom_couple_handling(r3, &mut out_lo[..2 * n + 1], neg_2_pow_sign, n, 2, 4);

        // 1, -1
        let neg_1_sign = _limbs_mul_toom_evaluate_poly_in_1_and_neg_1(
            v2,
            v0,
            4,
            xs,
            n,
            s,
            &mut out_lo[..n + 1],
            xs,
            ys,
        ) != _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
            v3,
            v1,
            ys,
            n,
            t,
            &mut out_lo[..n + 1],
        );
        // X(-1) * Y(-1)
        _limbs_mul_same_length_to_out_toom_54_recursive(out_lo, v0, v1);
        // X(1) * Y(1)
        _limbs_mul_same_length_to_out_toom_54_recursive(r7, v2, v3);
        _limbs_toom_couple_handling(r7, &mut out_lo[..2 * n + 1], neg_1_sign, n, 0, 0);

        // 2, -2
        neg_2_sign = _limbs_mul_toom_evaluate_poly_in_2_and_neg_2(v2, v0, 4, xs, n, s, out_lo)
            != _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(
                v3,
                v1,
                ys,
                n,
                t,
                &mut out_lo[..n + 1],
            );
        // X(-2) * Y(-2)
        _limbs_mul_same_length_to_out_toom_54_recursive(out_lo, v0, v1);
    }
    {
        let (r5, remainder) = out[3 * n..].split_at_mut(2 * n + 2);
        let (v2, v3) = remainder.split_at_mut(n + 1);
        // X(2) * Y(2)
        _limbs_mul_same_length_to_out_toom_54_recursive(r5, v2, &v3[..n + 1]);
    }
    {
        let (out_lo, r5) = out.split_at_mut(3 * n);
        _limbs_toom_couple_handling(r5, &mut out_lo[..2 * n + 1], neg_2_sign, n, 1, 2);
    }

    // X(0) * Y(0)
    _limbs_mul_same_length_to_out_toom_54_recursive(out, &xs[..n], &ys[..n]);

    // Infinity
    if s > t {
        _limbs_mul_to_out_toom_54_recursive(&mut out[7 * n..], a4, b3);
    } else {
        _limbs_mul_to_out_toom_54_recursive(&mut out[7 * n..], b3, a4);
    };
    _limbs_mul_toom_interpolate_8_points(out, n, s + t, r3, r7, scratch_hi);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_greater_to_out_toom_62` are valid.
pub fn _limbs_mul_greater_to_out_toom_62_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if xs_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if xs_len >= 3 * ys_len {
        (xs_len - 1) / 6
    } else {
        (ys_len - 1) >> 1
    };
    let s = xs_len.saturating_sub(5 * n);
    let t = ys_len.saturating_sub(n);
    0 < s && s <= n && 0 < t && t <= n
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_greater_to_out_toom_62`.
///
/// This is mpn_toom62_mul_itch from gmp-impl.h.
pub fn _limbs_mul_greater_to_out_toom_62_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1 + if xs_len >= 3 * ys_len {
        (xs_len - 1) / 6
    } else {
        (ys_len - 1) >> 1
    };
    10 * (n + 1)
}

///  Evaluate in:
///  0, +1, -1, +2, -2, 1/2, +inf
///
/// <-s-><--n--><--n--><--n--><--n--><--n-->
///  ___ ______ ______ ______ ______ ______
/// |a5_|___a4_|___a3_|___a2_|___a1_|___a0_|
///                            |_b1_|___b0_|
///                            <-t--><--n-->
///
/// v0  =    a0                       *   b0      #    A(0)*B(0)
/// v1  = (  a0+  a1+ a2+ a3+  a4+  a5)*( b0+ b1) #    A(1)*B(1)      ah  <= 5   bh <= 1
/// vm1 = (  a0-  a1+ a2- a3+  a4-  a5)*( b0- b1) #   A(-1)*B(-1)    |ah| <= 2   bh  = 0
/// v2  = (  a0+ 2a1+4a2+8a3+16a4+32a5)*( b0+2b1) #    A(2)*B(2)      ah  <= 62  bh <= 2
/// vm2 = (  a0- 2a1+4a2-8a3+16a4-32a5)*( b0-2b1) #   A(-2)*B(-2)    -41<=ah<=20 -1<=bh<=0
/// vh  = (32a0+16a1+8a2+4a3+ 2a4+  a5)*(2b0+ b1) #  A(1/2)*B(1/2)    ah  <= 62  bh <= 2
/// vinf=                           a5 *      b1  #  A(inf)*B(inf)
pub fn _limbs_mul_greater_to_out_toom_62(
    pp: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let an = xs.len();
    let bn = ys.len();
    let n = 1 + if an >= 3 * bn {
        (an - 1) / 6
    } else {
        (bn - 1) >> 1
    };
    split_into_chunks!(xs, n, s, [a0, a1, a2, a3, a4], a5);
    split_into_chunks!(ys, n, t, [b0], b1);

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    let mut as1 = vec![0; n + 1];
    let mut asm1 = vec![0; n + 1];
    let mut as2 = vec![0; n + 1];
    let mut asm2 = vec![0; n + 1];
    let mut ash = vec![0; n + 1];

    let mut bs1 = vec![0; n + 1];
    let mut bsm1 = vec![0; n];
    let mut bs2 = vec![0; n + 1];
    let mut bsm2 = vec![0; n + 1];
    let mut bsh = vec![0; n + 1];

    // Compute as1 and asm1.
    let w3_neg_a = _limbs_mul_toom_evaluate_poly_in_1_and_neg_1(
        &mut as1,
        &mut asm1,
        5,
        xs,
        n,
        s,
        &mut pp[..n + 1],
        xs,
        ys,
    );

    // Compute as2 and asm2.
    let w1_neg_a =
        _limbs_mul_toom_evaluate_poly_in_2_and_neg_2(&mut as2, &mut asm2, 5, xs, n, s, pp);

    // Compute ash = 32 a0 + 16 a1 + 8 a2 + 4 a3 + 2 a4 + a5
    // = 2*(2*(2*(2*(2*a0 + a1) + a2) + a3) + a4) + a5
    let mut cy = limbs_shl_to_out(&mut ash, &a0[..n], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut ash[..n], &a1[..n]) {
        1
    } else {
        0
    };
    cy = 2 * cy + limbs_slice_shl_in_place(&mut ash[..n], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut ash[..n], &a2[..n]) {
        1
    } else {
        0
    };
    cy = 2 * cy + limbs_slice_shl_in_place(&mut ash[..n], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut ash[..n], &a3[..n]) {
        1
    } else {
        0
    };
    cy = 2 * cy + limbs_slice_shl_in_place(&mut ash[..n], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut ash[..n], &a4[..n]) {
        1
    } else {
        0
    };
    cy = 2 * cy + limbs_slice_shl_in_place(&mut ash[..n], 1);
    ash[n] = cy
        + if limbs_slice_add_greater_in_place_left(&mut ash[..n], &a5[..s]) {
            1
        } else {
            0
        };

    // Compute bs1 and bsm1.
    let mut w1_neg_b;
    let w3_neg_b;
    if t == n {
        bs1[n] = if limbs_add_same_length_to_out(&mut bs1, &b0[..n], &b1[..n]) {
            1
        } else {
            0
        };
        if limbs_cmp_same_length(&b0[..n], &b1[..n]) == Ordering::Less {
            limbs_sub_same_length_to_out(&mut bsm1, &b1[..n], &b0[..n]);
            w1_neg_b = false;
            w3_neg_b = true;
        } else {
            limbs_sub_same_length_to_out(&mut bsm1, &b0[..n], &b1[..n]);
            w1_neg_b = false;
            w3_neg_b = false;
        }
    } else {
        bs1[n] = if limbs_add_to_out(&mut bs1, &b0[..n], &b1[..t]) {
            1
        } else {
            0
        };
        if limbs_test_zero(&b0[t..n]) && limbs_cmp_same_length(&b0[..t], &b1[..t]) == Ordering::Less
        {
            limbs_sub_same_length_to_out(&mut bsm1, &b1[..t], &b0[..t]);
            limbs_set_zero(&mut bsm1[t..n]);
            w1_neg_b = false;
            w3_neg_b = true;
        } else {
            limbs_sub_to_out(&mut bsm1, &b0[..n], &b1[..t]);
            w1_neg_b = false;
            w3_neg_b = false;
        }
    }

    // Compute bs2 and bsm2. Recycling bs1 and bsm1; bs2=bs1+b1, bsm2 = bsm1 - b1
    limbs_add_to_out(&mut bs2, &bs1[..n + 1], &b1[..t]);
    if w3_neg_b {
        bsm2[n] = if limbs_add_to_out(&mut bsm2, &bsm1[..n], &b1[..t]) {
            1
        } else {
            0
        };
        w1_neg_b = true;
    } else {
        if t < n {
            if limbs_test_zero(&bsm1[t..n])
                && limbs_cmp_same_length(&bsm1[..t], &b1[..t]) == Ordering::Less
            {
                assert!(!limbs_sub_same_length_to_out(
                    &mut bsm2,
                    &b1[..t],
                    &bsm1[..t]
                ));
                limbs_set_zero(&mut bsm2[t..n + 1]);
                w1_neg_b = true;
            } else {
                assert!(!limbs_sub_to_out(&mut bsm2, &bsm1[..n], &b1[..t]));
                bsm2[n] = 0;
            }
        } else {
            if limbs_cmp_same_length(&bsm1[..n], &b1[..n]) == Ordering::Less {
                assert!(!limbs_sub_same_length_to_out(
                    &mut bsm2,
                    &b1[..n],
                    &bsm1[..n]
                ));
                w1_neg_b = true;
            } else {
                assert!(!limbs_sub_same_length_to_out(
                    &mut bsm2,
                    &bsm1[..n],
                    &b1[..n]
                ));
            }
            bsm2[n] = 0;
        }
    }

    // Compute bsh, recycling bs1. bsh=bs1+b0;
    bsh[n] = bs1[n]
        + if limbs_add_same_length_to_out(&mut bsh, &bs1[..n], &b0[..n]) {
            1
        } else {
            0
        };

    assert!(as1[n] <= 5);
    assert!(bs1[n] <= 1);
    assert!(asm1[n] <= 2);
    assert!(as2[n] <= 62);
    assert!(bs2[n] <= 2);
    assert!(asm2[n] <= 41);
    assert!(bsm2[n] <= 1);
    assert!(ash[n] <= 62);
    assert!(bsh[n] <= 2);

    // Must be in allocation order, as they overwrite one limb beyond 2n+1.
    limbs_mul_same_length_to_out(scratch, &as2[..n + 1], &bs2[..n + 1]); // v2, 2n+1 limbs
                                                                         // vm2, 2n+1 limbs
    limbs_mul_same_length_to_out(&mut scratch[2 * n + 1..], &asm2[..n + 1], &bsm2[..n + 1]);
    // vh, 2n+1 limbs
    limbs_mul_same_length_to_out(&mut scratch[4 * n + 2..], &ash[..n + 1], &bsh[..n + 1]);
    split_into_chunks_mut!(scratch, 2 * n + 1, [v2, vm2, vh, vm1, scratch_out], _unused);
    limbs_mul_same_length_to_out(vm1, &asm1[..n], &bsm1[..n]); // vm1, 2n+1 limbs
    let mut cy = 0;
    if asm1[n] == 1 {
        cy = if limbs_slice_add_same_length_in_place_left(&mut vm1[n..2 * n], &bsm1[..n]) {
            1
        } else {
            0
        };
    } else if asm1[n] == 2 {
        cy = mpn_addmul_1(&mut vm1[n..], &bsm1[..n], 2);
    }
    vm1[2 * n] = cy;
    {
        // v1, 2n+1 limbs
        let (v0, remainder) = pp.split_at_mut(2 * n);
        let (v1, vinf) = remainder.split_at_mut(4 * n);
        limbs_mul_same_length_to_out(v1, &as1[..n], &bs1[..n]);
        if as1[n] == 1 {
            cy = bs1[n]
                + if limbs_slice_add_same_length_in_place_left(&mut v1[n..2 * n], &bs1[..n]) {
                    1
                } else {
                    0
                };
        } else if as1[n] == 2 {
            cy = 2 * bs1[n] + mpn_addmul_1(&mut v1[n..], &bs1[..n], 2);
        } else if as1[n] != 0 {
            cy = as1[n] * bs1[n] + mpn_addmul_1(&mut v1[n..], &bs1[..n], as1[n]);
        } else {
            cy = 0;
        }
        if bs1[n] != 0 {
            cy += if limbs_slice_add_same_length_in_place_left(&mut v1[n..2 * n], &as1[..n]) {
                1
            } else {
                0
            };
        }
        v1[2 * n] = cy;

        limbs_mul_same_length_to_out(v0, &a0[..n], &b0[..n]); // v0, 2n limbs

        // vinf, s+t limbs
        limbs_mul_to_out(vinf, &a5[..s], &b1[..t]);
    }

    let w1_neg = w1_neg_a != w1_neg_b;
    let w3_neg = w3_neg_a != w3_neg_b;
    _limbs_mul_toom_interpolate_7_points(
        pp,
        n,
        s + t,
        w1_neg,
        vm2,
        w3_neg,
        vm1,
        v2,
        vh,
        scratch_out,
    );
}
