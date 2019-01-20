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
};
use natural::arithmetic::mul::poly_interpolate::{
    _limbs_mul_toom_interpolate_5_points, _limbs_mul_toom_interpolate_6_points,
};
use natural::arithmetic::mul::{
    _limbs_mul_to_out_basecase, limbs_mul_same_length_to_out, limbs_mul_to_out,
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

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_22`.
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
pub fn _limbs_mul_to_out_toom_22_scratch_size(xs_len: usize) -> usize {
    2 * (xs_len + Limb::WIDTH as usize)
}

// TODO make these compiler flags?
pub const TUNE_PROGRAM_BUILD: bool = true;
pub const WANT_FAT_BINARY: bool = false;

pub const MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD >= 2 * MUL_TOOM22_THRESHOLD;

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// This is TOOM22_MUL_N_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_same_length_to_out_toom_22_recursive(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    assert_eq!(xs.len(), ys.len());
    if !MAYBE_MUL_TOOM22 || xs.len() < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else {
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    }
}

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// Normally, this calls `_limbs_mul_to_out_basecase` or `_limbs_mul_to_out_toom_22`. But when when
/// the fraction MUL_TOOM33_THRESHOLD / MUL_TOOM22_THRESHOLD is large, an initially small relative
/// unbalance will become a larger and larger relative unbalance with each recursion (the difference
/// s - t will be invariant over recursive calls). Therefore, we need to call
/// `_limbs_mul_to_out_toom_32`.
///
/// This is TOOM22_MUL_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_to_out_toom_22_recursive(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if !MAYBE_MUL_TOOM22 || ys_len < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else if 4 * xs_len < 5 * ys_len {
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    } else if _limbs_mul_to_out_toom_32_input_sizes_valid(xs_len, ys_len) {
        _limbs_mul_to_out_toom_32(out_limbs, xs, ys, scratch);
    } else {
        limbs_mul_to_out(out_limbs, xs, ys);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_22_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. xs.len() > 2
/// 4. ys.len() > 0
/// 5a. If xs.len() is even, xs.len() < 2 * ys.len()
/// 5b. If xs.len() is odd, xs.len() + 1 < 2 * ys.len()
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
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom22_mul from mpn/generic/toom22_mul.c.
pub fn _limbs_mul_to_out_toom_22(
    out_limbs: &mut [Limb],
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

    let (xs0, xs1) = xs.split_at(n); // xs0: length n, xs1: length s
    let (ys0, ys1) = ys.split_at(n); // ys0: length n, ys1: length t

    let mut v_neg_1_neg = false;
    {
        let (asm1, bsm1) = out_limbs.split_at_mut(n); // asm1: length n

        // Compute asm1.
        if s == n {
            if limbs_cmp_same_length(xs0, xs1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs1, xs0);
                v_neg_1_neg = true;
            } else {
                limbs_sub_same_length_to_out(asm1, xs0, xs1);
            }
        } else {
            // n - s == 1
            if xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs1, &xs0[..s]);
                asm1[s] = 0;
                v_neg_1_neg = true;
            } else {
                asm1[s] = xs0[s];
                if limbs_sub_same_length_to_out(asm1, &xs0[..s], xs1) {
                    asm1[s].wrapping_sub_assign(1);
                }
            }
        }

        // Compute bsm1.
        if t == n {
            if limbs_cmp_same_length(ys0, ys1) == Ordering::Less {
                limbs_sub_same_length_to_out(bsm1, ys1, ys0);
                v_neg_1_neg.not_assign();
            } else {
                limbs_sub_same_length_to_out(bsm1, ys0, ys1);
            }
        } else if limbs_test_zero(&ys0[t..])
            && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
        {
            limbs_sub_same_length_to_out(bsm1, ys1, &ys0[..t]);
            limbs_set_zero(&mut bsm1[t..n]);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_to_out(bsm1, ys0, ys1);
        }

        let (v_neg_1, scratch_out) = scratch.split_at_mut(2 * n); // v_neg_1: length 2 * n
        _limbs_mul_same_length_to_out_toom_22_recursive(v_neg_1, asm1, &bsm1[..n], scratch_out);
    }
    let (v_neg_1, scratch_out) = scratch.split_at_mut(2 * n); // v_neg_1: length 2 * n
    let mut carry = 0;
    let mut carry2;
    {
        let (v_0, v_pos_inf) = out_limbs.split_at_mut(2 * n); // v_0: length 2 * n
        if s > t {
            _limbs_mul_to_out_toom_22_recursive(v_pos_inf, xs1, ys1, scratch_out);
        } else {
            _limbs_mul_same_length_to_out_toom_22_recursive(v_pos_inf, xs1, &ys1[..s], scratch_out);
        }

        // v_0, 2 * n limbs
        _limbs_mul_same_length_to_out_toom_22_recursive(v_0, xs0, ys0, scratch_out);

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
        if limbs_slice_add_same_length_in_place_left(&mut out_limbs[n..3 * n], v_neg_1) {
            carry += 1;
        }
    } else if limbs_sub_same_length_in_place_left(&mut out_limbs[n..3 * n], v_neg_1) {
        carry.wrapping_sub_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut out_limbs[2 * n..2 * n + s + t],
        carry2
    ));
    if carry <= 2 {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[3 * n..2 * n + s + t],
            carry
        ));
    } else {
        assert!(!limbs_sub_limb_in_place(
            &mut out_limbs[3 * n..2 * n + s + t],
            1
        ));
    }
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_32`.
///
/// This is mpn_toom32_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_32_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };
    2 * n + 1
}

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// This is TOOM32_MUL_N_REC from mpn/generic/toom32_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_32_recursive(p: &mut [Limb], a: &[Limb], b: &[Limb]) {
    limbs_mul_same_length_to_out(p, a, b);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_32` are valid.
pub fn _limbs_mul_to_out_toom_32_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
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
    let s = xs_len - 2 * n;
    let t = ys_len - n;
    0 < s && s <= n && 0 < t && t <= n && s + t >= n
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_32_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_32_input_sizes_valid`. The gist is that `xs` must be less
///    than 3 times as long as `ys`.
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
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom32_mul from mpn/generic/toom32_mul.c.
#[allow(unreachable_code)] //TODO remove
pub fn _limbs_mul_to_out_toom_32(
    out_limbs: &mut [Limb],
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

    let (xs0, remainder) = xs.split_at(n); // xs0: length n
    let (xs1, xs2) = remainder.split_at(n); // xs1: length n, xs2: length s
    let s = xs2.len();
    let (ys0, ys1) = ys.split_at(n); // ys0: length n
    let t = ys1.len();

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    assert!(s + t >= n);

    let mut hi: SignedLimb;
    let mut v_neg_1_neg;
    {
        // Product area of size xs_len + ys_len = 3 * n + s + t >= 4 * n + 2.
        // out_limbs_lo: length 2 * n
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        let (am1, bm1) = out_limbs_hi.split_at_mut(n); // am1: length n
        {
            let (ap1, bp1) = out_limbs_lo.split_at_mut(n); // ap1: length n, bp1: length n

            // Compute ap1 = xs0 + xs1 + a3, am1 = xs0 - xs1 + a3
            let mut ap1_hi = 0;
            if limbs_add_to_out(ap1, xs0, xs2) {
                ap1_hi = 1;
            }
            if ap1_hi == 0 && limbs_cmp_same_length(ap1, xs1) == Ordering::Less {
                assert!(!limbs_sub_same_length_to_out(am1, xs1, ap1));
                hi = 0;
                v_neg_1_neg = true;
            } else {
                hi = ap1_hi;
                if limbs_sub_same_length_to_out(am1, ap1, xs1) {
                    hi -= 1;
                }
                v_neg_1_neg = false;
            }
            if limbs_slice_add_same_length_in_place_left(ap1, xs1) {
                ap1_hi += 1;
            }

            let bp1_hi;
            // Compute bp1 = ys0 + ys1 and bm1 = ys0 - ys1.
            if t == n {
                bp1_hi = limbs_add_same_length_to_out(bp1, ys0, ys1);
                if limbs_cmp_same_length(ys0, ys1) == Ordering::Less {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys1, ys0));
                    v_neg_1_neg.not_assign();
                } else {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys0, ys1));
                }
            } else {
                bp1_hi = limbs_add_to_out(bp1, ys0, ys1);
                if limbs_test_zero(&ys0[t..])
                    && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
                {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys1, &ys0[..t]));
                    limbs_set_zero(&mut bm1[t..n]);
                    v_neg_1_neg.not_assign();
                } else {
                    assert!(!limbs_sub_to_out(bm1, ys0, ys1));
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
        _limbs_mul_same_length_to_out_toom_32_recursive(out_limbs_lo, am1, &bm1[..n]);
        if hi != 0 {
            hi = 0;
            if limbs_slice_add_same_length_in_place_left(&mut out_limbs_lo[n..], &bm1[..n]) {
                hi = 1;
            }
        }
    }
    out_limbs[2 * n] = hi.to_unsigned_bitwise();

    // v1 <-- (v1 + vm1) / 2 = x0 + x2
    {
        let scratch = &mut scratch[..2 * n + 1];
        let out_limbs = &out_limbs[..2 * n + 1];
        if v_neg_1_neg {
            limbs_sub_same_length_in_place_left(scratch, out_limbs);
            assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
        } else {
            limbs_slice_add_same_length_in_place_left(scratch, &out_limbs);
            assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
        }
    }

    // We get x1 + x3 = (x0 + x2) - (x0 - x1 + x2 - x3), and hence
    //
    // y = x1 + x3 + (x0 + x2) * B
    //   = (x0 + x2) * B + (x0 + x2) - vm1.
    //
    // y is 3 * n + 1 limbs, y = y0 + y1 B + y2 B^2. We store them as follows: y0 at scratch, y1 at
    // out_limbs + 2 * n, and y2 at scratch + n (already in place, except for carry propagation).
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
    hi = out_limbs[2 * n].to_signed_bitwise();
    let mut scratch_high = scratch[2 * n];
    if limbs_add_same_length_to_out(&mut out_limbs[2 * n..], &scratch[..n], &scratch[n..2 * n]) {
        scratch_high += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut scratch[n..2 * n + 1],
        scratch_high
    ));

    if v_neg_1_neg {
        let carry = limbs_slice_add_same_length_in_place_left(&mut scratch[..n], &out_limbs[..n]);
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        if _limbs_add_same_length_with_carry_in_in_place_left(
            &mut out_limbs_hi[..n],
            &out_limbs_lo[n..],
            carry,
        ) {
            hi += 1;
        }
        assert!(!limbs_slice_add_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi.to_unsigned_bitwise()
        ));
    } else {
        let carry = limbs_sub_same_length_in_place_left(&mut scratch[..n], &out_limbs[..n]);
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        if _limbs_sub_same_length_with_borrow_in_in_place_left(
            &mut out_limbs_hi[..n],
            &out_limbs_lo[n..],
            carry,
        ) {
            hi += 1;
        }
        assert!(!limbs_sub_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi.to_unsigned_bitwise()
        ));
    }

    _limbs_mul_same_length_to_out_toom_32_recursive(out_limbs, xs0, ys0);
    // s + t limbs. Use mpn_mul for now, to handle unbalanced operands
    if s > t {
        limbs_mul_to_out(&mut out_limbs[3 * n..], xs2, ys1);
    } else {
        limbs_mul_to_out(&mut out_limbs[3 * n..], ys1, xs2);
    }

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
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        let (out_limbs_0, out_limbs_1) = out_limbs_lo.split_at_mut(n);
        // out_limbs_0: length n, out_limbs_1: length n
        let (out_limbs_2, out_limbs_3) = out_limbs_hi.split_at_mut(n); // out_limbs_2: length n
        let carry = limbs_sub_same_length_in_place_left(out_limbs_1, &out_limbs_3[..n]);
        hi = scratch[2 * n].to_signed_bitwise();
        if carry {
            hi.wrapping_add_assign(1);
        }

        let borrow =
            _limbs_sub_same_length_with_borrow_in_in_place_left(out_limbs_2, out_limbs_0, carry);
        if _limbs_sub_same_length_with_borrow_in_to_out(
            out_limbs_3,
            &scratch[n..2 * n],
            out_limbs_1,
            borrow,
        ) {
            hi -= 1;
        }
    }

    if limbs_slice_add_greater_in_place_left(&mut out_limbs[n..4 * n], &scratch[..n]) {
        hi += 1;
    }

    if s + t > n {
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(4 * n);
        // out_limbs_lo: length 4 * n
        let out_limbs_hi = &mut out_limbs_hi[..s + t - n];
        if limbs_sub_in_place_left(&mut out_limbs_lo[2 * n..], out_limbs_hi) {
            hi -= 1;
        }

        if hi < 0 {
            //TODO remove once this is seen
            panic!("hi < 0 second time: {:?} {:?}", xs, ys);
            assert!(!limbs_sub_limb_in_place(
                out_limbs_hi,
                Limb::checked_from(-hi).unwrap()
            ));
        } else {
            assert!(!limbs_slice_add_limb_in_place(
                out_limbs_hi,
                Limb::checked_from(hi).unwrap()
            ));
        }
    } else {
        assert_eq!(hi, 0);
    }
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_33`.
///
/// Scratch need is 5 * xs_len / 2 + 10 * k, where k is the recursion depth. We use 3 * xs_len + C,
/// so that we can use a smaller constant.
///
/// This is mpn_toom33_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_33_scratch_size(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}

pub const MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD < 3 * MUL_TOOM22_THRESHOLD;
pub const MAYBE_MUL_TOOM33: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM44_THRESHOLD >= 3 * MUL_TOOM33_THRESHOLD;

/// A helper function for `_limbs_mul_to_out_toom_33`.
///
/// This is TOOM33_MUL_N_REC from mpn/generic/toom33_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_33_recursive(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(xs.len(), n);
    if MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else if !MAYBE_MUL_TOOM33 || n < MUL_TOOM33_THRESHOLD {
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    } else {
        _limbs_mul_to_out_toom_33(out_limbs, xs, ys, scratch);
    }
}

const SMALLER_RECURSION: bool = false;

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_33` are valid.
pub fn _limbs_mul_to_out_toom_33_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = (xs_len + 2) / 3;
    let s = xs_len - 2 * n;
    if ys_len < 2 * n {
        return false;
    }
    let t = ys_len - 2 * n;
    0 < s && s <= n && 0 < t && t <= n
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_33_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_33_input_sizes_valid`. The gist is that 2 * `xs.len()`
///    must be less than 3 times `ys.len()`.
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
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom33_mul from mpn/generic/toom33_mul.c.
#[allow(clippy::cyclomatic_complexity)]
pub fn _limbs_mul_to_out_toom_33(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = (xs_len + 2) / 3;
    let (xs_0, remainder) = xs.split_at(n); // xs_0: length n
    let (xs_1, xs_2) = remainder.split_at(n); // xs_1: length n
    let s = xs_2.len();
    let (ys_0, remainder) = ys.split_at(n); // ys_0: length n
    let (ys_1, ys_2) = remainder.split_at(n); // ys_1: length n
    let t = ys_2.len();

    assert!(ys_len >= 2 * n);
    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    let mut v_neg_1_neg = false;
    {
        let (bs1, remainder) = out_limbs.split_at_mut(n + 1); // bs1 length: n + 1
        let (as2, bs2) = remainder.split_at_mut(n + 1); // as2 length: n + 1
        {
            // we need 4n+4 <= 4n+s+t
            let (gp, remainder) = scratch.split_at_mut(2 * n + 2);
            let gp = &mut gp[..n]; // gp length: n
            let (asm1, remainder) = remainder.split_at_mut(n + 1); // asm1 length: n + 1
            let (bsm1, as1) = remainder.split_at_mut(n + 1); // bsm1 length: n + 1

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
            let mut carry = 0;
            if limbs_add_to_out(gp, ys_0, ys_2) {
                carry = 1;
            }
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
                // this branch not tested
                _limbs_mul_same_length_to_out_toom_33_recursive(
                    v_neg_1,
                    &asm1[..n],
                    &bsm1[..n],
                    scratch_out,
                );
                let mut carry = 0;
                if asm1[n] != 0 {
                    carry = bsm1[n];
                    if limbs_slice_add_same_length_in_place_left(&mut v_neg_1[n..2 * n], &bsm1[..n])
                    {
                        carry += 1;
                    }
                }
                if bsm1[n] != 0
                    && limbs_slice_add_same_length_in_place_left(&mut v_neg_1[n..2 * n], &asm1[..n])
                {
                    carry += 1;
                }
                v_neg_1[2 * n] = carry;
            } else {
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
        let v_inf = &mut out_limbs[4 * n..];
        // v_inf, s + t limbs
        if s > t {
            limbs_mul_to_out(v_inf, xs_2, ys_2);
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

    if SMALLER_RECURSION {
        // this branch not tested
        let (bs1, v_1) = out_limbs.split_at_mut(2 * n); // bs1 length: 2 * n
        let (as1, scratch_out) = scratch[3 * n + 3..].split_at_mut(n + 1); // as1 length: 3 * n + 3
                                                                           // v_1, 2n+1 limbs
        _limbs_mul_same_length_to_out_toom_33_recursive(v_1, &as1[..n], &bs1[..n], scratch_out);
        let mut carry = 0;
        if as1[n] == 1 {
            carry = bs1[n];
            if limbs_slice_add_same_length_in_place_left(&mut v_1[n..2 * n], &bs1[..n]) {
                carry += 1;
            }
        } else if as1[n] != 0 {
            carry = 2 * bs1[n] + mpn_addmul_1(&mut v_1[n..], &bs1[..n], 2);
        }
        if bs1[n] == 1 {
            if limbs_slice_add_same_length_in_place_left(&mut v_1[n..2 * n], &as1[..n]) {
                carry += 1;
            }
        } else if bs1[n] != 0 {
            carry += mpn_addmul_1(&mut v_1[n..], &as1[..n], 2);
        }
        v_1[2 * n] = carry;
    } else {
        let carry = out_limbs[4 * n + 1];
        {
            let (bs1, v1) = out_limbs.split_at_mut(2 * n); // bs1 length: 2 * n
            let (as1, scratch_out) = scratch[4 * n + 4..].split_at_mut(n + 1); // as1 length: n + 1
            _limbs_mul_same_length_to_out_toom_33_recursive(v1, as1, &bs1[..n + 1], scratch_out);
        }
        out_limbs[4 * n + 1] = carry;
    }
    // v_0, 2 * n limbs
    _limbs_mul_same_length_to_out_toom_33_recursive(
        out_limbs,
        &xs[..n],
        &ys[..n],
        &mut scratch[5 * n + 5..],
    );

    let (v_neg_1, v_2) = scratch.split_at_mut(2 * n + 1); // v_neg_1 length: 2 * n + 1
    _limbs_mul_toom_interpolate_5_points(out_limbs, v_2, v_neg_1, n, s + t, v_neg_1_neg, v_inf0);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_42` are valid.
pub fn _limbs_mul_to_out_toom_42_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = if xs_len >= 2 * ys_len {
        (xs_len + 3) >> 2
    } else {
        (ys_len + 1) >> 1
    };
    if xs_len < 3 * n {
        return false;
    }
    let s = xs_len - 3 * n;
    if ys_len < n {
        return false;
    }
    let t = ys_len - n;
    0 < s && s <= n && 0 < t && t <= n
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_42`.
///
/// This is mpn_toom42_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_42_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = if xs_len >= 2 * ys_len {
        (xs_len + 3) >> 2
    } else {
        (ys_len + 1) >> 1
    };
    6 * n + 3
}

/// A helper function for `_limbs_mul_to_out_toom_42`.
///
/// This is TOOM42_MUL_N_REC from mpn/generic/toom42_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_42_recursive(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
) {
    limbs_mul_same_length_to_out(out_limbs, xs, ys);
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_42_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_42_input_sizes_valid`. The gist is that `xs` must be less
///    than 4 times as long as `ys`.
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
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom42_mul from mpn/generic/toom42_mul.c.
pub fn _limbs_mul_to_out_toom_42(
    out_limbs: &mut [Limb],
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
    let (xs_0, remainder) = xs.split_at(n); // xs_0 length: n
    let (xs_1, remainder) = remainder.split_at(n); // xs_1 length: n
    let (xs_2, xs_3) = remainder.split_at(n); // xs_2 length: n
    let s = xs_3.len();
    let (ys_0, ys_1) = ys.split_at(n); // ys_0 length: n
    let t = ys_1.len();

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    let mut scratch2 = vec![0; 6 * n + 5];
    let (as1, remainder) = scratch2.split_at_mut(n + 1); // as1 length: n + 1
    let (asm1, remainder) = remainder.split_at_mut(n + 1); // asm1 length: n + 1
    let (as2, remainder) = remainder.split_at_mut(n + 1); // as2 length: n + 1
    let (bs1, remainder) = remainder.split_at_mut(n + 1); // bs1 length: n + 1
    let (bsm1, bs2) = remainder.split_at_mut(n); // bsm1 length: n, bs2 length: n + 1

    // Compute as1 and asm1.
    let mut v_neg_1_neg = _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
        as1,
        asm1,
        xs,
        n,
        s,
        &mut out_limbs[..n + 1],
    );

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
        let (v_0, remainder) = out_limbs.split_at_mut(2 * n); // v_0 length: 2 * n
        let (v_1, v_inf) = remainder.split_at_mut(2 * n); // v_1 length: 2 * n

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
        if s > t {
            limbs_mul_to_out(v_inf, xs_3, ys_1);
        } else {
            limbs_mul_to_out(v_inf, ys_1, xs_3);
        }

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
    _limbs_mul_toom_interpolate_5_points(out_limbs, v_2, v_neg_1, n, s + t, v_neg_1_neg, v_inf_0);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_43` are valid.
pub fn _limbs_mul_to_out_toom_43_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if 3 * xs_len >= 4 * ys_len {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    let s = xs_len - 3 * n;
    let t = ys_len - 2 * n;
    0 < s && s <= n && 0 < t && t <= n && s + t >= 5
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_43`.
///
/// This is mpn_toom43_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_43_scratch_size(xs_len: usize, ys_len: usize) -> usize {
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
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_43_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_43_input_sizes_valid`. The gist is that `xs` must be less
///    than twice as long as `ys`.
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
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom43_mul from mpn/generic/toom43_mul.c.
pub fn _limbs_mul_to_out_toom_43(
    out_limbs: &mut [Limb],
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
    let (ys_0, remainder) = ys.split_at(n); // ys_0 length: n
    let (ys_1, ys_2) = remainder.split_at(n); // ys_1 length: n
    let t = ys_2.len();

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    // This is probably true whenever `xs_len` >= 25 or `ys_len` >= 19, I think. It guarantees that
    // we can fit 5 values of size n + 1 in the product area.
    assert!(s + t >= 5);

    // Total scratch need is 6 * n + 4; we allocate one extra limb, because products will overwrite
    // 2 * n + 2 limbs.
    let limit = n + 1;
    let mut v_neg_1_neg = false;
    let mut v_neg_2_neg = false;
    {
        let (bs1, remainder) = out_limbs.split_at_mut(limit); // bs1 length: n + 1
        let (bsm2, remainder) = remainder.split_at_mut(limit); // bsm1 length: n + 1
        let (bs2, remainder) = remainder.split_at_mut(limit); // bs2 length: n + 1
        let (as2, as1) = remainder.split_at_mut(limit); // as2 length: n + 1
        let as1 = &mut as1[..limit]; // as1 length: n + 1
        {
            // bsm1 length: n + 1
            let (bsm1, remainder) = &mut scratch[2 * n + 2..].split_at_mut(limit);
            let (asm1, asm2) = remainder.split_at_mut(limit); // asm1 length: n + 1

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

        let (small_scratch, remainder) = scratch.split_at_mut(2 * n + 2);
        let small_scratch = &mut small_scratch[..limit]; // small_scratch length: n + 1
        let (bsm1, remainder) = remainder.split_at_mut(limit); // bsm1 length: n + 1
        let (asm1, asm2) = remainder.split_at_mut(limit); // asm1 length: n + 1
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
        let (v_neg_1, remainder) = scratch.split_at_mut(2 * limit); // v_neg_1 length: 2 * n + 2
        let (bsm1, asm1) = remainder.split_at_mut(limit); // bsm1 length: limit
                                                          // v_neg_1, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(v_neg_1, &asm1[..limit], bsm1); // W4
    }
    {
        // v_neg_2 length: 2 * n + 3
        let (v_neg_2, asm2) = scratch[2 * n + 1..].split_at_mut(2 * n + 3);
        // v_neg_2, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(v_neg_2, &asm2[..limit], &out_limbs[limit..2 * limit]); // W2
    }
    {
        let (bs2, as2) = out_limbs[2 * limit..].split_at_mut(limit); // bs2 length: n + 1
                                                                     // v_neg_2, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(&mut scratch[4 * n + 2..], &as2[..limit], bs2); // W1
    }
    {
        let (bs1, remainder) = out_limbs.split_at_mut(2 * n); // bs1 length: 2 * n
        let (v_1, as1) = remainder.split_at_mut(2 * n + 4); // v_1 length: 2 * n + 4
                                                            // v_1, 2 * n + 1 limbs
        limbs_mul_same_length_to_out(v_1, &as1[..limit], &bs1[..limit]); // W3
    }
    {
        let v_inf = &mut out_limbs[5 * n..];
        // v_inf, s + t limbs // W0
        if s > t {
            limbs_mul_to_out(v_inf, xs_3, ys_2);
        } else {
            limbs_mul_to_out(v_inf, ys_2, xs_3);
        }
    }

    // v_0, 2 * n limbs
    limbs_mul_same_length_to_out(out_limbs, &xs[..n], ys_0); // W5
    let (v_neg_1, remainder) = scratch.split_at_mut(2 * n + 1); // v_neg_1 length: 2 * n + 1
    let (v_neg_2, v_2) = remainder.split_at_mut(2 * n + 1); // v_neg_2 length: 2 * n + 1
    _limbs_mul_toom_interpolate_6_points(
        out_limbs,
        n,
        t + s,
        v_neg_1_neg,
        v_neg_1,
        v_neg_2_neg,
        v_neg_2,
        v_2,
    );
}
