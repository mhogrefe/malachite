use natural::LOG_LIMB_BITS;
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
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(0u32).trailing_zeros(), None);
    /// assert_eq!(Natural::from(3u32).trailing_zeros(), Some(0));
    /// assert_eq!(Natural::from(72u32).trailing_zeros(), Some(3));
    /// assert_eq!(Natural::from(100u32).trailing_zeros(), Some(2));
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().trailing_zeros(), Some(12));
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        match *self {
            Small(0) => None,
            Small(small) => Some(small.trailing_zeros() as u64),
            Large(ref limbs) => {
                let zero_limbs = limbs.iter().take_while(|&&limb| limb == 0).count();
                Some(((zero_limbs as u64) << LOG_LIMB_BITS) +
                     limbs[zero_limbs].trailing_zeros() as u64)
            }
        }
    }
}
