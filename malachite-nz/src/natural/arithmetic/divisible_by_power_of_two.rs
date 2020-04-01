use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is divisible by 2 raised to a given power.
///
/// This function assumes that `limbs` is nonempty and does not only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(pow, `limbs.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by_power_of_two::*;
///
/// assert_eq!(limbs_divisible_by_power_of_two(&[3], 1), false);
/// // 10^12 = 232 * 2^32 + 3567587328
/// assert_eq!(limbs_divisible_by_power_of_two(&[3_567_587_328, 232], 11), true);
/// assert_eq!(limbs_divisible_by_power_of_two(&[3_567_587_328, 232], 12), true);
/// assert_eq!(limbs_divisible_by_power_of_two(&[3_567_587_328, 232], 13), false);
/// ```
///
/// This is mpz_divisible_2exp_p from mpz/divis_2exp.c, GMP 6.1.2, where a is non-negative.
pub fn limbs_divisible_by_power_of_two(xs: &[Limb], pow: u64) -> bool {
    let zero_limbs = usize::exact_from(pow >> Limb::LOG_WIDTH);
    zero_limbs < xs.len()
        && slice_test_zero(&xs[..zero_limbs])
        && xs[zero_limbs].divisible_by_power_of_two(pow & Limb::WIDTH_MASK)
}

impl<'a> DivisibleByPowerOfTwo for &'a Natural {
    /// Returns whether `self` is divisible by 2<sup>`pow`</sup>. If `self` is 0, the result is
    /// always true; otherwise, it is equivalent to `self.trailing_zeros().unwrap() <= pow`, but
    /// more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(pow, `self.significant_bits`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.divisible_by_power_of_two(100), true);
    /// assert_eq!(Natural::from(100u32).divisible_by_power_of_two(2), true);
    /// assert_eq!(Natural::from(100u32).divisible_by_power_of_two(3), false);
    /// assert_eq!(Natural::trillion().divisible_by_power_of_two(12), true);
    /// assert_eq!(Natural::trillion().divisible_by_power_of_two(13), false);
    /// ```
    fn divisible_by_power_of_two(self, pow: u64) -> bool {
        match (self, pow) {
            (_, 0) => true,
            (&Natural(Small(small)), pow) => small.divisible_by_power_of_two(pow),
            (&Natural(Large(ref limbs)), pow) => limbs_divisible_by_power_of_two(limbs, pow),
        }
    }
}
