use malachite_base::num::BitAccess;
use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

/// Provides functions for accessing and modifying the `index`th bit of a `Natural`, or the
/// coefficient of 2^<pow>`index`</pow> in its binary expansion.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::BitAccess;
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ZERO;
///     x.assign_bit(2, true);
///     x.assign_bit(5, true);
///     x.assign_bit(6, true);
///     assert_eq!(x.to_string(), "100");
///     x.assign_bit(2, false);
///     x.assign_bit(5, false);
///     x.assign_bit(6, false);
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Natural::ZERO;
///     x.flip_bit(10);
///     assert_eq!(x.to_string(), "1024");
///     x.flip_bit(10);
///     assert_eq!(x.to_string(), "0");
/// }
/// ```
impl BitAccess for Natural {
    /// Determines whether the `index`th bit of a `Natural`, or the coefficient of
    /// 2<sup>`index`</sup> in its binary expansion, is 0 or 1. `false` means 0, `true` means 1.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::BitAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(123u32).get_bit(2), false);
    ///     assert_eq!(Natural::from(123u32).get_bit(3), true);
    ///     assert_eq!(Natural::from(123u32).get_bit(100), false);
    ///     assert_eq!(Natural::trillion().get_bit(12), true);
    ///     assert_eq!(Natural::trillion().get_bit(100), false);
    /// }
    /// ```
    fn get_bit(&self, index: u64) -> bool {
        match *self {
            Small(small) => small.get_bit(index),
            Large(ref limbs) => {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                limbs.get(limb_index).map_or(false, |limb| {
                    limb.get_bit(index & u64::from(LIMB_BITS_MASK))
                })
            }
        }
    }

    /// Sets the `index`th bit of a `Natural`, or the coefficient of 2<sup>`index`</sup> in its
    /// binary expansion, to 1.
    ///
    /// Time: worst case O(`index`)
    ///
    /// Additional memory: worst case O(`index`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::BitAccess;
    /// use malachite_base::num::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::ZERO;
    ///     x.set_bit(2);
    ///     x.set_bit(5);
    ///     x.set_bit(6);
    ///     assert_eq!(x.to_string(), "100");
    /// }
    /// ```
    fn set_bit(&mut self, index: u64) {
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                if index < LIMB_BITS.into() {
                    let mut modified = *small;
                    modified.set_bit(index);
                    Some(modified)
                } else {
                    None
                }
            },
            {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                if limb_index >= limbs.len() {
                    limbs.resize(limb_index + 1, 0);
                }
                limbs[limb_index].set_bit(index & u64::from(LIMB_BITS_MASK));
            }
        );
    }

    /// Sets the `index`th bit of a `Natural`, or the coefficient of 2<sup>`index`</sup> in its
    /// binary expansion, to 0.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::BitAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(0x7fu32);
    ///     x.clear_bit(0);
    ///     x.clear_bit(1);
    ///     x.clear_bit(3);
    ///     x.clear_bit(4);
    ///     assert_eq!(x.to_string(), "100");
    /// }
    /// ```
    fn clear_bit(&mut self, index: u64) {
        match *self {
            Small(ref mut small) => {
                small.clear_bit(index);
            }
            Large(ref mut limbs) => {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                if limb_index < limbs.len() {
                    limbs[limb_index].clear_bit(index & u64::from(LIMB_BITS_MASK));
                } else {
                    return;
                }
            }
        }
        self.trim();
    }
}