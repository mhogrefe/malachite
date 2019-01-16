use malachite_base::limbs::limbs_leading_zero_limbs;
use malachite_base::num::PrimitiveInteger;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, returns the
/// number of trailing zeros in the binary expansion of a `Natural` (equivalently, the multiplicity
/// of 2 in its prime factorization). The limbs cannot be empty or all zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
///
/// assert_eq!(limbs_trailing_zeros(&[4]), 2);
/// assert_eq!(limbs_trailing_zeros(&[0, 4]), 34);
/// ```
pub fn limbs_trailing_zeros(limbs: &[Limb]) -> u64 {
    let zero_limbs = limbs_leading_zero_limbs(limbs);
    let remaining_zeros = u64::from(limbs[zero_limbs].trailing_zeros());
    ((zero_limbs as u64) << Limb::LOG_WIDTH) + remaining_zeros
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
    /// use malachite_base::num::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.trailing_zeros(), None);
    ///     assert_eq!(Natural::from(3u32).trailing_zeros(), Some(0));
    ///     assert_eq!(Natural::from(72u32).trailing_zeros(), Some(3));
    ///     assert_eq!(Natural::from(100u32).trailing_zeros(), Some(2));
    ///     assert_eq!(Natural::trillion().trailing_zeros(), Some(12));
    /// }
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        match *self {
            Small(0) => None,
            Small(small) => Some(small.trailing_zeros().into()),
            Large(ref limbs) => Some(limbs_trailing_zeros(limbs)),
        }
    }
}
