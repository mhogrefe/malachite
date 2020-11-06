use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoMul, ModPowerOfTwoMulAssign, ModPowerOfTwoSquare, ModPowerOfTwoSquareAssign,
    Parity, Square, WrappingSquare,
};
use malachite_base::num::conversion::traits::SplitInHalf;

use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul::_limbs_mul_greater_to_out_basecase;
use natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use natural::arithmetic::mul::mul_low::_limbs_mul_low_same_length_basecase;
use natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use natural::arithmetic::mul::toom::{TUNE_PROGRAM_BUILD, WANT_FAT_BINARY};
use natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::square::{_limbs_square_diagonal, limbs_square_to_out};
use natural::Natural;
use platform::{
    DoubleLimb, Limb, MULLO_BASECASE_THRESHOLD, MULLO_DC_THRESHOLD, SQRLO_DC_THRESHOLD,
    SQR_TOOM2_THRESHOLD, SQR_TOOM3_THRESHOLD, SQR_TOOM4_THRESHOLD, SQR_TOOM8_THRESHOLD,
};

/// This is MPN_SQRLO_DIAGONAL from mpn/generic/sqrlo_basecase.c, GMP 6.1.2.
fn _limbs_square_low_diagonal(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let half_n = n >> 1;
    _limbs_square_diagonal(out, &xs[..half_n]);
    if n.odd() {
        out[n - 1] = xs[half_n].wrapping_square();
    }
}

/// This is MPN_SQRLO_DIAG_ADDLSH1 from mpn/generic/sqrlo_basecase.c, GMP 6.1.2.
pub fn _limbs_square_diagonal_shl_add(out: &mut [Limb], scratch: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    assert_eq!(scratch.len(), n - 1);
    assert_eq!(out.len(), n);
    _limbs_square_low_diagonal(out, xs);
    limbs_slice_shl_in_place(scratch, 1);
    limbs_slice_add_same_length_in_place_left(&mut out[1..], scratch);
}

//TODO tune
pub const SQRLO_DC_THRESHOLD_LIMIT: usize = 500;

const SQRLO_BASECASE_ALLOC: usize = if SQRLO_DC_THRESHOLD_LIMIT < 2 {
    1
} else {
    SQRLO_DC_THRESHOLD_LIMIT - 1
};

/// TODO complexity
///
/// This is mpn_sqrlo_basecase from mpn/generic/sqrlo_basecase.c, GMP 6.1.2.
pub fn _limbs_square_low_basecase(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let out = &mut out[..n];
    assert_ne!(n, 0);
    let xs_0 = xs[0];
    match n {
        1 => out[0] = xs_0.wrapping_square(),
        2 => {
            let (p_hi, p_lo) = DoubleLimb::from(xs_0).square().split_in_half();
            out[0] = p_lo;
            out[1] = (xs_0.wrapping_mul(xs[1]) << 1).wrapping_add(p_hi);
        }
        _ => {
            let scratch = &mut [0; SQRLO_BASECASE_ALLOC];
            // must fit n - 1 limbs in scratch
            assert!(n <= SQRLO_DC_THRESHOLD_LIMIT);
            let scratch = &mut scratch[..n - 1];
            limbs_mul_limb_to_out(scratch, &xs[1..], xs_0);
            for i in 1.. {
                let two_i = i << 1;
                if two_i >= n - 1 {
                    break;
                }
                limbs_slice_add_mul_limb_same_length_in_place_left(
                    &mut scratch[two_i..],
                    &xs[i + 1..n - i],
                    xs[i],
                );
            }
            _limbs_square_diagonal_shl_add(out, scratch, xs);
        }
    }
}

//TODO tune
const SQRLO_BASECASE_THRESHOLD: usize = 10;

/// This is MAYBE_range_basecase from mpn/generic/sqrlo.c, GMP 6.1.2.
const MAYBE_RANGE_BASECASE: bool = TUNE_PROGRAM_BUILD
    || WANT_FAT_BINARY
    || (if SQRLO_DC_THRESHOLD == 0 {
        SQRLO_BASECASE_THRESHOLD
    } else {
        SQRLO_DC_THRESHOLD
    }) < SQR_TOOM2_THRESHOLD * 36 / (36 - 11);

/// This is MAYBE_range_toom22 from mpn/generic/sqrlo.c, GMP 6.1.2.
const MAYBE_RANGE_TOOM22: bool = TUNE_PROGRAM_BUILD
    || WANT_FAT_BINARY
    || (if SQRLO_DC_THRESHOLD == 0 {
        SQRLO_BASECASE_THRESHOLD
    } else {
        SQRLO_DC_THRESHOLD
    }) < SQR_TOOM3_THRESHOLD * 36 / (36 - 11);

/// This is mpn_sqrlo_itch from mpn/generic/sqrlo.c, GMP 6.1.2.
pub const fn _limbs_square_low_scratch_len(len: usize) -> usize {
    len << 1
}

/// Requires a scratch space of 2 * `xs.len()` limbs at `scratch`.
///
/// TODO complexity
///
/// This is mpn_dc_sqrlo from mpn/generic/sqrlo.c, GMP 6.1.2.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn _limbs_square_low_divide_and_conquer(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let len = xs.len();
    let out = &mut out[..len];
    assert!(len > 1);
    // We need a fractional approximation of the value 0 < a <= 1/2, giving the minimum in the
    // function k = (1 - a) ^ e / (1 - 2 * a ^ e).
    let len_small = if MAYBE_RANGE_BASECASE && len < SQR_TOOM2_THRESHOLD * 36 / (36 - 11) {
        len >> 1
    } else if MAYBE_RANGE_TOOM22 && len < SQR_TOOM3_THRESHOLD * 36 / (36 - 11) {
        len * 11 / 36 // n1 ~= n*(1-.694...)
    } else if len < SQR_TOOM4_THRESHOLD * 40 / (40 - 9) {
        len * 9 / 40 // n1 ~= n*(1-.775...)
    } else if len < SQR_TOOM8_THRESHOLD * 10 / 9 {
        len * 7 / 39 // n1 ~= n*(1-.821...)
    } else {
        len / 10 // n1 ~= n*(1-.899...) [TOOM88]
    };
    let len_big = len - len_small;
    // x0 ^ 2
    let (xs_lo, xs_hi) = xs.split_at(len_big);
    limbs_square_to_out(scratch, xs_lo);
    let xs_lo = &xs_lo[..len_small];
    let (out_lo, out_hi) = out.split_at_mut(len_big);
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(len);
    out_lo.copy_from_slice(&scratch_lo[..len_big]);
    // x1 * x0 * 2^(n2 GMP_NUMB_BITS)
    if len_small < MULLO_BASECASE_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(scratch_hi, xs_hi, xs_lo);
    } else if len_small < MULLO_DC_THRESHOLD {
        _limbs_mul_low_same_length_basecase(scratch_hi, xs_hi, xs_lo);
    } else {
        limbs_mul_low_same_length(scratch_hi, xs_hi, xs_lo);
    }
    limbs_shl_to_out(out_hi, &scratch_hi[..len_small], 1);
    limbs_slice_add_same_length_in_place_left(out_hi, &scratch_lo[len_big..]);
}

impl ModPowerOfTwoSquare for Natural {
    type Output = Natural;

    /// Computes `self.square()` mod 2<sup>`pow`</sup>, taking `self` by value. Assumes the input is
    /// already reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSquare;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_two_square(2), 0);
    /// assert_eq!(Natural::from(5u32).mod_power_of_two_square(3), 1);
    /// assert_eq!(
    ///     Natural::from_str("12345678987654321").unwrap().mod_power_of_two_square(64).to_string(),
    ///     "16556040056090124897"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_square(mut self, pow: u64) -> Natural {
        self.mod_power_of_two_square_assign(pow);
        self
    }
}

impl<'a> ModPowerOfTwoSquare for &'a Natural {
    type Output = Natural;

    /// Computes `self.square()` mod 2<sup>`pow`</sup>, taking `self` by reference. Assumes the
    /// input is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSquare;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Natural::ZERO).mod_power_of_two_square(2), 0);
    /// assert_eq!((&Natural::from(5u32)).mod_power_of_two_square(3), 1);
    /// assert_eq!(
    ///     (&Natural::from_str("12345678987654321").unwrap())
    ///         .mod_power_of_two_square(64).to_string(),
    ///     "16556040056090124897"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_square(self, pow: u64) -> Natural {
        self.mod_power_of_two_mul(self, pow)
    }
}

impl ModPowerOfTwoSquareAssign for Natural {
    /// Replaces `self` with `self.square()` mod 2<sup>`pow`</sup>. Assumes the input is already
    /// reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSquareAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_two_square_assign(2);
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::from(5u32);
    /// n.mod_power_of_two_square_assign(3);
    /// assert_eq!(n, 1);
    ///
    /// let mut n = Natural::from_str("12345678987654321").unwrap();
    /// n.mod_power_of_two_square_assign(64);
    /// assert_eq!(n.to_string(), "16556040056090124897");
    /// ```
    #[inline]
    fn mod_power_of_two_square_assign(&mut self, pow: u64) {
        self.mod_power_of_two_mul_assign(self.clone(), pow);
    }
}
