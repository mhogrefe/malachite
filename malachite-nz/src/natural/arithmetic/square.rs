use std::cmp::{max, Ordering};

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, DivRound, Square, SquareAssign, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::conversion::traits::{SplitInHalf, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode;

use fail_on_untested_path;
use natural::arithmetic::add::{
    limbs_add_limb_to_out, limbs_add_same_length_to_out, limbs_add_to_out,
    limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use natural::arithmetic::mul::poly_interpolate::_limbs_mul_toom_interpolate_5_points;
use natural::arithmetic::mul::toom::{TUNE_PROGRAM_BUILD, WANT_FAT_BINARY};
use natural::arithmetic::shl::limbs_slice_shl_in_place;
use natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
};
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural;
use platform::{DoubleLimb, Limb, SQR_TOOM2_THRESHOLD, SQR_TOOM3_THRESHOLD};

// This is mpn_toom4_sqr_itch from gmp-impl.h, GMP 6.1.2.
const fn _limbs_square_to_out_toom_4_scratch_len(xs_len: usize) -> usize {
    3 * xs_len + (Limb::WIDTH as usize)
}

const SQR_TOOM6_THRESHOLD: usize = 351;
const SQR_TOOM8_THRESHOLD: usize = 454;

// This is mpn_toom6_sqr_itch from gmp-impl.h, GMP 6.1.2.
pub(crate) fn _limbs_square_to_out_toom_6_scratch_len(n: usize) -> usize {
    (n << 1)
        + max(
            (SQR_TOOM6_THRESHOLD << 1) + usize::wrapping_from(Limb::WIDTH) * 6,
            _limbs_square_to_out_toom_4_scratch_len(SQR_TOOM6_THRESHOLD),
        )
        - (SQR_TOOM6_THRESHOLD << 1)
}

// This is mpn_toom8_sqr_itch from gmp-impl.h, GMP 6.1.2.
pub(crate) fn _limbs_square_to_out_toom_8_scratch_len(n: usize) -> usize {
    ((n * 15) >> 3)
        + max(
            ((SQR_TOOM8_THRESHOLD * 15) >> 3) + usize::wrapping_from(Limb::WIDTH) * 6,
            _limbs_square_to_out_toom_6_scratch_len(SQR_TOOM8_THRESHOLD),
        )
        - ((SQR_TOOM8_THRESHOLD * 15) >> 3)
}

/// This is MPN_SQR_DIAGONAL from mpn/generic/sqr_basecase.c, GMP 6.1.2.
#[inline]
fn _limbs_square_diagonal(out: &mut [Limb], xs: &[Limb]) {
    for (i, &x) in xs.iter().enumerate() {
        let (square_hi, square_lo) = DoubleLimb::from(x).square().split_in_half();
        let i_2 = i << 1;
        out[i_2] = square_lo;
        out[i_2 | 1] = square_hi;
    }
}

/// scratch must have length 2 * xs.len() - 2 and out must have length 2 * xs.len().
///
/// This is MPN_SQR_DIAG_ADDLSH1 from mpn/generic/sqr_basecase.c, GMP 6.1.2.
#[inline]
pub fn _limbs_square_diagonal_add_shl_1(out: &mut [Limb], scratch: &mut [Limb], xs: &[Limb]) {
    _limbs_square_diagonal(out, xs);
    let (out_last, out_init) = out.split_last_mut().unwrap();
    *out_last += limbs_slice_shl_in_place(scratch, 1);
    if limbs_slice_add_same_length_in_place_left(&mut out_init[1..], scratch) {
        *out_last += 1;
    }
}

/// Interpreting a slices of `Limb`s as the limbs (in ascending order) of a `Natural`s, writes the
/// `2 * xs.len()` least-significant limbs of the square of the `Natural`s to an output slice. The
/// output must be at least twice as long as `xs.len()`, `xs.len()` must be less than
/// `SQR_TOOM2_THRESHOLD`, and `xs` cannot be empty.
///
/// Time: worst case O(n<sup>2</sup>)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is less than twice the length of `xs`, `xs.len()` > SQR_TOOM2_THRESHOLD, or if
/// `xs` is empty.
///
/// This is mpn_sqr_basecase from mpn/generic/sqr_basecase.c, GMP 6.1.2.
pub fn _limbs_square_to_out_basecase(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let (xs_head, xs_tail) = xs.split_first().unwrap();
    let (square_hi, square_lo) = DoubleLimb::from(*xs_head).square().split_in_half();
    out[0] = square_lo;
    out[1] = square_hi;
    if n > 1 {
        assert!(n <= SQR_TOOM2_THRESHOLD);
        let scratch = &mut [0; SQR_TOOM2_THRESHOLD << 1];
        let two_n = n << 1;
        let scratch = &mut scratch[..two_n - 2];
        let (scratch_last, scratch_init) = scratch[..n].split_last_mut().unwrap();
        *scratch_last = limbs_mul_limb_to_out(scratch_init, xs_tail, *xs_head);
        for i in 1..n - 1 {
            let (scratch_last, scratch_init) = scratch[i..][i..n].split_last_mut().unwrap();
            let (xs_head, xs_tail) = xs[i..].split_first().unwrap();
            *scratch_last =
                limbs_slice_add_mul_limb_same_length_in_place_left(scratch_init, xs_tail, *xs_head);
        }
        _limbs_square_diagonal_add_shl_1(&mut out[..two_n], scratch, xs);
    }
}

/// This is mpn_toom2_sqr_itch from gmp-impl.h, GMP 6.1.2.
pub const fn _limbs_square_to_out_toom_2_scratch_len(xs_len: usize) -> usize {
    (xs_len + Limb::WIDTH as usize) << 1
}

/// This is MAYBE_sqr_toom2 from mpn/generic/toom2_sqr.c, GMP 6.1.2.
pub const TOOM2_MAYBE_SQR_TOOM2: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || SQR_TOOM3_THRESHOLD >= 2 * SQR_TOOM2_THRESHOLD;

/// This is TOOM2_SQR_REC from mpn/generic/toom2_sqr.c, GMP 6.1.2.
fn _limbs_square_to_out_toom_2_recursive(p: &mut [Limb], a: &[Limb], ws: &mut [Limb]) {
    if !TOOM2_MAYBE_SQR_TOOM2 || a.len() < SQR_TOOM2_THRESHOLD {
        _limbs_square_to_out_basecase(p, a);
    } else {
        _limbs_square_to_out_toom_2(p, a, ws);
    }
}

/// Seems to be never faster than basecase over basecase's range
///
/// Interpreting a slices of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// `2 * xs.len()` least-significant limbs of the square of the `Natural` to an output slice. A
/// "scratch" slice is provided for the algorithm to use. An upper bound for the number of scratch
/// limbs needed is provided by `_limbs_square_to_out_toom_2_scratch_len`. The following
/// restrictions on the input slices must be met:
/// 1. `out`.len() >= 2 * `xs`.len()
/// 2. `xs`.len() > 1
///
/// Evaluate in: -1, 0, infinity.
///
/// <-s--><--n-->
///  ____ ______
/// |xs1_|__xs0_|
///
/// v_0     = xs_0 ^ 2          # X(0) ^ 2
/// v_neg_1 = (xs_0 - xs_1) ^ 2 # X(-1) ^ 2
/// v_inf   = xs_1 ^ 2          # X(inf) ^ 2
///
/// Time: O(n<sup>log<sub>2</sub>3</sup>)
///
/// Additional memory: O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom2_sqr from mpn/generic/toom2_sqr.c, GMP 6.1.2.
pub fn _limbs_square_to_out_toom_2(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let xs_len = xs.len();
    assert!(xs_len > 1);
    let out = &mut out[..xs_len << 1];
    let s = xs_len >> 1;
    let n = xs_len - s;
    let (xs_0, xs_1) = xs.split_at(n);
    if s == n {
        if limbs_cmp_same_length(xs_0, xs_1) == Ordering::Less {
            limbs_sub_same_length_to_out(out, xs_1, xs_0);
        } else {
            limbs_sub_same_length_to_out(out, xs_0, xs_1);
        }
    } else {
        // n - s == 1
        let (xs_0_last, xs_0_init) = xs_0.split_last().unwrap();
        let (out_last, out_init) = out[..n].split_last_mut().unwrap();
        if *xs_0_last == 0 && limbs_cmp_same_length(xs_0_init, xs_1) == Ordering::Less {
            limbs_sub_same_length_to_out(out_init, xs_1, xs_0_init);
            *out_last = 0;
        } else {
            *out_last = *xs_0_last;
            if limbs_sub_same_length_to_out(out_init, xs_0_init, xs_1) {
                out_last.wrapping_sub_assign(1);
            }
        }
    }
    let (v_0, v_inf) = out.split_at_mut(n << 1);
    let (v_neg_1, scratch_out) = scratch.split_at_mut(n << 1);
    _limbs_square_to_out_toom_2_recursive(v_neg_1, &v_0[..n], scratch_out);
    _limbs_square_to_out_toom_2_recursive(v_inf, xs_1, scratch_out);
    _limbs_square_to_out_toom_2_recursive(v_0, xs_0, scratch_out);
    let (v_0_lo, v_0_hi) = v_0.split_at_mut(n);
    let (v_inf_lo, v_inf_hi) = v_inf.split_at_mut(n);
    let mut carry = Limb::iverson(limbs_slice_add_same_length_in_place_left(v_inf_lo, v_0_hi));
    let mut carry2 = carry;
    if limbs_add_same_length_to_out(v_0_hi, v_inf_lo, v_0_lo) {
        carry2 += 1;
    }
    if limbs_slice_add_greater_in_place_left(v_inf_lo, &v_inf_hi[..s + s - n]) {
        carry += 1;
    }
    if limbs_sub_same_length_in_place_left(&mut out[n..3 * n], v_neg_1) {
        carry.wrapping_sub_assign(1);
    }
    assert!(carry.wrapping_add(1) <= 3);
    assert!(carry2 <= 2);
    assert!(!limbs_slice_add_limb_in_place(&mut out[n << 1..], carry2));
    let out_hi = &mut out[3 * n..];
    if carry <= 2 {
        assert!(!limbs_slice_add_limb_in_place(out_hi, carry));
    } else {
        assert!(!limbs_sub_limb_in_place(out_hi, 1));
    }
}

/// This function can be used to determine whether the size of the input slice to
/// `_limbs_square_to_out_toom_3` is valid.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
pub fn _limbs_square_to_out_toom_3_input_size_valid(xs_len: usize) -> bool {
    let n = xs_len.div_round(3, RoundingMode::Ceiling);
    xs_len > n << 1 && xs_len <= 3 * n
}

/// This is mpn_toom3_sqr_itch from gmp-impl.h, GMP 6.1.2.
pub const fn _limbs_square_to_out_toom_3_scratch_len(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}

//TODO
const SQR_TOOM4_THRESHOLD: usize = 28;

//TODO
const SMALLER_RECURSION_TOOM_3: bool = true;

/// This is MAYBE_sqr_toom3 from mpn/generic/toom3_sqr.c, GMP 6.1.2.
pub const TOOM3_MAYBE_SQR_TOOM3: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || SQR_TOOM4_THRESHOLD >= 3 * SQR_TOOM3_THRESHOLD;

/// This is MAYBE_sqr_basecase from mpn/generic/toom3_sqr.c, GMP 6.1.2.
pub const TOOM3_MAYBE_SQR_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || SQR_TOOM3_THRESHOLD < 3 * SQR_TOOM2_THRESHOLD;

/// This is TOOM3_SQR_REC from mpn/generic/toom3_sqr.c, GMP 6.1.2.
fn _limbs_square_to_out_toom_3_recursive(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let n = xs.len();
    if TOOM3_MAYBE_SQR_BASECASE && n < SQR_TOOM2_THRESHOLD {
        _limbs_square_to_out_basecase(out, xs);
    } else if !TOOM3_MAYBE_SQR_TOOM3 || n < SQR_TOOM3_THRESHOLD {
        _limbs_square_to_out_toom_2(out, xs, scratch);
    } else {
        _limbs_square_to_out_toom_3(out, xs, scratch);
    }
}

/// xs_len >= 3
/// This is mpn_toom3_sqr from mpn/generic/toom3_sqr.c, GMP 6.1.2.
pub fn _limbs_square_to_out_toom_3(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let xs_len = xs.len();
    let n = xs_len.div_round(3, RoundingMode::Ceiling);
    let m = n + 1;
    let k = m + n;
    let s = xs_len - (n << 1);
    assert_ne!(s, 0);
    assert!(s <= n);
    split_into_chunks!(xs, n, [xs_0, xs_1], xs_2);
    split_into_chunks_mut!(scratch, m << 1, [scratch_lo, asm1], as1);
    let (asm1_last, asm1_init) = asm1[..m].split_last_mut().unwrap();
    let (as1_last, as1_init) = as1[..m].split_last_mut().unwrap();
    let scratch_lo = &mut scratch_lo[..n];
    let mut carry = Limb::iverson(limbs_add_to_out(scratch_lo, xs_0, xs_2));
    *as1_last = carry;
    if limbs_add_same_length_to_out(as1_init, scratch_lo, xs_1) {
        *as1_last += 1;
    }
    if carry == 0 && limbs_cmp_same_length(scratch_lo, xs_1) == Ordering::Less {
        limbs_sub_same_length_to_out(asm1_init, xs_1, scratch_lo);
    } else if limbs_sub_same_length_to_out(asm1_init, scratch_lo, xs_1) {
        carry.wrapping_sub_assign(1);
    }
    *asm1_last = carry;
    let as2 = &mut out[m..m << 1];
    let (as2_last, as2_init) = as2.split_last_mut().unwrap();
    let (as1_lo, as1_hi) = as1_init.split_at_mut(s);
    let mut carry = Limb::iverson(limbs_add_same_length_to_out(as2_init, xs_2, as1_lo));
    if s != n {
        carry = Limb::iverson(limbs_add_limb_to_out(&mut as2_init[s..], as1_hi, carry));
    }
    carry.wrapping_add_assign(*as1_last);
    carry = carry.arithmetic_checked_shl(1).unwrap();
    carry.wrapping_add_assign(limbs_slice_shl_in_place(as2_init, 1));
    if limbs_sub_same_length_in_place_left(as2_init, xs_0) {
        carry.wrapping_sub_assign(1);
    }
    *as2_last = carry;
    assert!(*as1_last <= 2);
    assert!(*asm1_last <= 1);
    let (scratch_lo, scratch_out) = scratch.split_at_mut(5 * m);
    if SMALLER_RECURSION_TOOM_3 {
        let (v_neg_1, asm1) = scratch_lo.split_at_mut(k);
        let (v_neg_1_last, v_neg_1_init) = v_neg_1.split_last_mut().unwrap();
        let (asm1_last, asm1_init) = asm1[1..n + 2].split_last().unwrap();
        _limbs_square_to_out_toom_3_recursive(v_neg_1_init, asm1_init, scratch_out);
        *v_neg_1_last = if *asm1_last != 0 {
            asm1_last.wrapping_add(limbs_slice_add_mul_limb_same_length_in_place_left(
                &mut v_neg_1_init[n..],
                asm1_init,
                2,
            ))
        } else {
            0
        };
    } else {
        fail_on_untested_path("_limbs_square_to_out_toom_3, !SMALLER_RECURSION");
        let (v_neg_1, asm1) = scratch_lo.split_at_mut(m << 1);
        _limbs_square_to_out_toom_3_recursive(v_neg_1, &asm1[..m], scratch_out);
    }
    _limbs_square_to_out_toom_3_recursive(&mut scratch_lo[k..], as2, scratch_out);
    let v_inf = &mut out[n << 2..];
    _limbs_square_to_out_toom_3_recursive(v_inf, xs_2, scratch_out);
    let v_inf_0 = v_inf[0];
    let (as1, scratch_out) = &mut scratch[m << 2..].split_at_mut(m);
    let out_hi = &mut out[n << 1..];
    if SMALLER_RECURSION_TOOM_3 {
        let (v_1_last, v_1_init) = out_hi[..k].split_last_mut().unwrap();
        let (as1_last, as1_init) = as1.split_last_mut().unwrap();
        _limbs_square_to_out_toom_3_recursive(v_1_init, as1_init, scratch_out);
        let v_1_init = &mut v_1_init[n..];
        *v_1_last = if *as1_last == 1 {
            limbs_slice_add_mul_limb_same_length_in_place_left(v_1_init, as1_init, 2)
                .wrapping_add(1)
        } else if *as1_last != 0 {
            let carry: Limb = as1_last.arithmetic_checked_shl(1).unwrap();
            carry.wrapping_add(limbs_slice_add_mul_limb_same_length_in_place_left(
                v_1_init, as1_init, 4,
            ))
        } else {
            0
        };
    } else {
        let carry = out_hi[k];
        _limbs_square_to_out_toom_3_recursive(out_hi, as1, scratch_out);
        out_hi[k] = carry;
    }
    let (v_neg_1, remainder) = scratch.split_at_mut(k);
    let (v_2, scratch_out) = remainder.split_at_mut(3 * n + 4);
    _limbs_square_to_out_toom_3_recursive(out, xs_0, scratch_out);
    _limbs_mul_toom_interpolate_5_points(out, v_2, v_neg_1, n, s << 1, false, v_inf_0);
}

impl Square for Natural {
    type Output = Natural;

    /// Squares a `Natural`, taking it by value.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.square(), 0);
    /// assert_eq!(Natural::from(123u32).square(), 15_129);
    /// ```
    #[inline]
    fn square(mut self) -> Natural {
        self.square_assign();
        self
    }
}

impl<'a> Square for &'a Natural {
    type Output = Natural;

    /// Squares a `Natural`, taking it by reference.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).square(), 0);
    /// assert_eq!((&Natural::from(123u32)).square(), 15_129);
    /// ```
    #[inline]
    fn square(self) -> Natural {
        //TODO use better algorithm
        self * self
    }
}

impl SquareAssign for Natural {
    /// Squares a `Natural` in place.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.square_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.square_assign();
    /// assert_eq!(x, 15_129);
    /// ```
    fn square_assign(&mut self) {
        //TODO use better algorithm
        *self *= self.clone();
    }
}
