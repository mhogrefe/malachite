use malachite_base::num::traits::Assign;
use natural::Natural::{self, Small};
use platform::Limb;

/// Assigns a `Limb` to a `Natural`.
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
/// use malachite_base::num::traits::Assign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(123u32);
///     x.assign(456u32);
///     assert_eq!(x.to_string(), "456");
/// }
/// ```
impl Assign<Limb> for Natural {
    #[inline]
    fn assign(&mut self, other: Limb) {
        *self = Small(other);
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Assign<u32> for Natural {
    #[inline]
    fn assign(&mut self, other: u32) {
        self.assign(Limb::from(other))
    }
}
