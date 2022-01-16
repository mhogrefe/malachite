use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::slices::slice_leading_zeros;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, returns the
/// number of trailing zeros in the binary expansion of a `Natural` (equivalently, the multiplicity
/// of 2 in its prime factorization). The limbs cannot be empty or all zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` only contains zeros.
///
/// # Examples
/// ```
/// use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
///
/// assert_eq!(limbs_trailing_zeros(&[4]), 2);
/// assert_eq!(limbs_trailing_zeros(&[0, 4]), 34);
/// ```
#[doc(hidden)]
pub fn limbs_trailing_zeros(xs: &[Limb]) -> u64 {
    let zeros = slice_leading_zeros(xs);
    let remaining_zeros = TrailingZeros::trailing_zeros(xs[zeros]);
    (u64::wrapping_from(zeros) << Limb::LOG_WIDTH) + remaining_zeros
}

impl Natural {
    /// Returns the number of trailing zeros in the binary expansion of a `Natural` (equivalently,
    /// the multiplicity of 2 in its prime factorization) or `None` is the `Natural` is 0.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.trailing_zeros(), None);
    /// assert_eq!(Natural::from(3u32).trailing_zeros(), Some(0));
    /// assert_eq!(Natural::from(72u32).trailing_zeros(), Some(3));
    /// assert_eq!(Natural::from(100u32).trailing_zeros(), Some(2));
    /// assert_eq!(Natural::trillion().trailing_zeros(), Some(12));
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        match *self {
            natural_zero!() => None,
            Natural(Small(small)) => Some(TrailingZeros::trailing_zeros(small)),
            Natural(Large(ref limbs)) => Some(limbs_trailing_zeros(limbs)),
        }
    }
}
