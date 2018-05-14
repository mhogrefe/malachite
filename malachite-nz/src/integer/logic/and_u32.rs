use integer::Integer;
use malachite_base::misc::WrappingFrom;
use natural::Natural::{self, Small};
use std::ops::{BitAnd, BitAndAssign};

/// Takes the bitwise and of an `Integer` and a `u32`, taking the `Integer` by reference. The output
/// is a `Natural`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(&Integer::ZERO & 123u32, 0);
///     assert_eq!(&Integer::from(123) & 0u32, 0);
///     assert_eq!(&Integer::from(-123) & 456u32, 384);
///     assert_eq!(&(-Integer::from(0xffff_ffffu32)) & 2u32, 0);
/// }
/// ```
impl<'a> BitAnd<u32> for &'a Integer {
    type Output = u32;

    fn bitand(self, other: u32) -> u32 {
        if self.sign {
            &self.abs & other
        } else {
            self.abs.neg_and_u32(other)
        }
    }
}

/// Takes the bitwise and of a `u32` and an `Integer`, taking the `Integer` by reference. The output
/// is a `Natural`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(123u32 & &Integer::ZERO, 0);
///     assert_eq!(0u32 & &Integer::from(123), 0);
///     assert_eq!(456u32 & &Integer::from(-123), 384);
///     assert_eq!(2u32 & &(-Integer::from(0xffff_ffffu32)), 0);
/// }
/// ```
impl<'a> BitAnd<&'a Integer> for u32 {
    type Output = u32;

    fn bitand(self, other: &'a Integer) -> u32 {
        other & self
    }
}

/// Bitwise-ands an `Integer` with a `u32` in place.
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
/// use malachite_base::num::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x &= 0xf0ff_ffffu32;
///     x &= 0xfff0_ffffu32;
///     x &= 0xffff_f0ffu32;
///     x &= 0xffff_fff0u32;
///     assert_eq!(x, 0xf0f0f0f0u32);
/// }
/// ```
impl BitAndAssign<u32> for Integer {
    fn bitand_assign(&mut self, other: u32) {
        if self.sign {
            self.abs &= other;
        } else {
            self.sign = true;
            self.abs = Small(self.abs.neg_and_u32(other));
        }
    }
}

impl Natural {
    fn neg_and_u32(&self, other: u32) -> u32 {
        u32::wrapping_from(self).wrapping_neg() & other
    }
}
