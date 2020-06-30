use std::cmp::max;

use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{SplitInHalf, WrappingFrom};

use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use natural::arithmetic::shl::limbs_slice_shl_in_place;
use natural::Natural;
use platform::{DoubleLimb, Limb};

// This is mpn_toom4_sqr_itch from gmp-impl.h, GMP 6.1.2.
const fn _limbs_square_to_out_toom_4_scratch_len(xs_len: usize) -> usize {
    3 * xs_len + (Limb::WIDTH as usize)
}

//TODO tune
pub const SQR_TOOM2_THRESHOLD: usize = 28;
pub(crate) const SQR_TOOM3_THRESHOLD: usize = 93;
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
fn _limbs_square_diagonal_add_shl_1(out: &mut [Limb], scratch: &mut [Limb], xs: &[Limb]) {
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
/// Additional memory: worst case O(1)
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
