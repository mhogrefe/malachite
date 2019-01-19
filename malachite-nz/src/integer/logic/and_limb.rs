use integer::Integer;
use malachite_base::misc::WrappingFrom;
use natural::Natural::{self, Small};
use platform::Limb;
use std::ops::{BitAnd, BitAndAssign};

/// Takes the bitwise and of an `Integer` and a `Limb`, taking the `Integer` by value. The output is
/// a `Limb`.
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
///     assert_eq!(Integer::ZERO & 123u32, 0);
///     assert_eq!(Integer::from(123) & 0u32, 0);
///     assert_eq!(Integer::from(-123) & 456u32, 384);
///     assert_eq!((-Integer::from(0xffff_ffffu32)) & 2u32, 0);
/// }
/// ```
impl BitAnd<Limb> for Integer {
    type Output = Limb;

    fn bitand(self, other: Limb) -> Limb {
        if self.sign {
            self.abs & other
        } else {
            self.abs.and_neg_limb_pos(other)
        }
    }
}

/// Takes the bitwise and of an `Integer` and a `Limb`, taking the `Integer` by reference. The
/// output is a `Limb`.
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
impl<'a> BitAnd<Limb> for &'a Integer {
    type Output = Limb;

    fn bitand(self, other: Limb) -> Limb {
        if self.sign {
            &self.abs & other
        } else {
            self.abs.and_neg_limb_pos(other)
        }
    }
}

/// Takes the bitwise and of a `Limb` and an `Integer`, taking the `Integer` by value. The output is
/// a `Limb`.
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
///     assert_eq!(123u32 & Integer::ZERO, 0);
///     assert_eq!(0u32 & Integer::from(123), 0);
///     assert_eq!(456u32 & Integer::from(-123), 384);
///     assert_eq!(2u32 & -Integer::from(0xffff_ffffu32), 0);
/// }
/// ```
impl BitAnd<Integer> for Limb {
    type Output = Limb;

    fn bitand(self, other: Integer) -> Limb {
        other & self
    }
}

/// Takes the bitwise and of a `Limb` and an `Integer`, taking the `Integer` by reference. The
/// output is a `Limb`.
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
impl<'a> BitAnd<&'a Integer> for Limb {
    type Output = Limb;

    fn bitand(self, other: &'a Integer) -> Limb {
        other & self
    }
}

/// Bitwise-ands an `Integer` with a `Limb` in place.
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
impl BitAndAssign<Limb> for Integer {
    fn bitand_assign(&mut self, other: Limb) {
        if self.sign {
            self.abs &= other;
        } else {
            self.sign = true;
            self.abs = Small(self.abs.and_neg_limb_pos(other));
        }
    }
}

/// Bitwise-ands a `Limb` with an `Integer` in place, taking the `Integer` by value.
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
///     let mut x = 0xffff_ffffu32;
///     x &= Integer::from(0xf0ff_ffffu32);
///     x &= Integer::from(0xfff0_ffffu32);
///     x &= Integer::from(0xffff_f0ffu32);
///     x &= Integer::from(0xffff_fff0u32);
///     assert_eq!(x, 0xf0f0f0f0u32);
/// }
/// ```
impl BitAndAssign<Integer> for Limb {
    fn bitand_assign(&mut self, other: Integer) {
        *self = other & *self;
    }
}

/// Bitwise-ands a `Limb` with an `Integer` in place, taking the `Integer` by reference.
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
///     let mut x = 0xffff_ffffu32;
///     x &= &Integer::from(0xf0ff_ffffu32);
///     x &= &Integer::from(0xfff0_ffffu32);
///     x &= &Integer::from(0xffff_f0ffu32);
///     x &= &Integer::from(0xffff_fff0u32);
///     assert_eq!(x, 0xf0f0f0f0u32);
/// }
/// ```
impl<'a> BitAndAssign<&'a Integer> for Limb {
    fn bitand_assign(&mut self, other: &'a Integer) {
        *self = other & *self;
    }
}

impl Natural {
    fn and_neg_limb_pos(&self, other: Limb) -> Limb {
        Limb::wrapping_from(self).wrapping_neg() & other
    }
}
