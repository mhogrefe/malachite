use malachite_base::num::conversion::traits::Assign;

use integer::Integer;
use platform::Limb;

/// Assigns a `Limb` to to an `Integer`.
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
/// use malachite_base::num::conversion::traits::Assign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(-123);
///     x.assign(456);
///     assert_eq!(x.to_string(), "456");
/// }
/// ```
impl Assign<Limb> for Integer {
    fn assign(&mut self, other: Limb) {
        self.sign = true;
        self.abs.assign(other);
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Assign<u32> for Integer {
    #[inline]
    fn assign(&mut self, other: u32) {
        self.assign(Limb::from(other));
    }
}
