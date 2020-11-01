use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoMul, ModPowerOfTwoMulAssign, ModPowerOfTwoSquare, ModPowerOfTwoSquareAssign,
    Parity, Square, WrappingSquare,
};
use malachite_base::num::conversion::traits::SplitInHalf;

use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use natural::arithmetic::shl::limbs_slice_shl_in_place;
use natural::arithmetic::square::_limbs_square_diagonal;
use natural::Natural;
use platform::{DoubleLimb, Limb};

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
fn _limbs_square_diagonal_shl_add(out: &mut [Limb], scratch: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    assert_eq!(scratch.len(), n - 1);
    assert_eq!(out.len(), n);
    _limbs_square_low_diagonal(out, xs);
    limbs_slice_shl_in_place(scratch, 1);
    limbs_slice_add_same_length_in_place_left(&mut out[1..], scratch);
}

//TODO tune
pub const SQRLO_DC_THRESHOLD_LIMIT: usize = 100;

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
            out[1] = xs_0.wrapping_mul(xs[1]).wrapping_mul(2).wrapping_add(p_hi);
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
