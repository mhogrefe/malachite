use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting a slice of `Limb`s, as the limbs (in ascending order) of a `Natural`, counts the
/// number of ones in the binary expansion of the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::count_ones::limbs_count_ones;
///
/// assert_eq!(limbs_count_ones(&[0, 1, 2]), 2);
/// assert_eq!(limbs_count_ones(&[1, 0xffff_ffff]), 33);
/// ```
pub fn limbs_count_ones(limbs: &[Limb]) -> u64 {
    limbs.iter().map(|limb| u64::from(limb.count_ones())).sum()
}

impl Natural {
    /// Counts the number of ones in the binary expansion of a `Natural`.
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
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.count_ones(), 0);
    ///     // 105 = 1101001b
    ///     assert_eq!(Natural::from(105u32).count_ones(), 4);
    ///     // 10^12 = 1110100011010100101001010001000000000000b
    ///     assert_eq!(Natural::trillion().count_ones(), 13);
    /// }
    /// ```
    pub fn count_ones(&self) -> u64 {
        match *self {
            Small(small) => small.count_ones().into(),
            Large(ref limbs) => limbs_count_ones(limbs),
        }
    }
}
