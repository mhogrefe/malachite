use malachite_base::num::PrimitiveInteger;
use natural::Natural::{self, Large, Small};

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
            Large(ref limbs) => {
                let zero_limbs = limbs.iter().take_while(|&&limb| limb == 0).count();
                let remaining_zeros = u64::from(limbs[zero_limbs].trailing_zeros());
                Some(((zero_limbs as u64) << u32::LOG_WIDTH) + remaining_zeros)
            }
        }
    }
}
