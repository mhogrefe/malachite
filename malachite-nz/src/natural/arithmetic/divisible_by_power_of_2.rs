use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is divisible by 2 raised to a given power.
///
/// This function assumes that `xs` is nonempty and does not only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(pow, `xs.len()`)
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by_power_of_2::*;
///
/// assert_eq!(limbs_divisible_by_power_of_2(&[3], 1), false);
/// // 10^12 = 232 * 2^32 + 3567587328
/// assert_eq!(limbs_divisible_by_power_of_2(&[3567587328, 232], 11), true);
/// assert_eq!(limbs_divisible_by_power_of_2(&[3567587328, 232], 12), true);
/// assert_eq!(limbs_divisible_by_power_of_2(&[3567587328, 232], 13), false);
/// ```
///
/// This is mpz_divisible_2exp_p from mpz/divis_2exp.c, GMP 6.2.1, where a is non-negative.
#[doc(hidden)]
pub fn limbs_divisible_by_power_of_2(xs: &[Limb], pow: u64) -> bool {
    let zeros = usize::exact_from(pow >> Limb::LOG_WIDTH);
    zeros < xs.len()
        && slice_test_zero(&xs[..zeros])
        && xs[zeros].divisible_by_power_of_2(pow & Limb::WIDTH_MASK)
}

impl<'a> DivisibleByPowerOf2 for &'a Natural {
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
    /// use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.divisible_by_power_of_2(100), true);
    /// assert_eq!(Natural::from(100u32).divisible_by_power_of_2(2), true);
    /// assert_eq!(Natural::from(100u32).divisible_by_power_of_2(3), false);
    /// assert_eq!(Natural::trillion().divisible_by_power_of_2(12), true);
    /// assert_eq!(Natural::trillion().divisible_by_power_of_2(13), false);
    /// ```
    fn divisible_by_power_of_2(self, pow: u64) -> bool {
        match (self, pow) {
            (_, 0) => true,
            (&Natural(Small(small)), pow) => small.divisible_by_power_of_2(pow),
            (&Natural(Large(ref limbs)), pow) => limbs_divisible_by_power_of_2(limbs, pow),
        }
    }
}
