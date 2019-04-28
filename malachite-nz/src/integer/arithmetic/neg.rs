use integer::Integer;
use malachite_base::num::traits::{NegAssign, NotAssign, Zero};
use platform::Limb;
use std::ops::Neg;

/// Returns the negative of an `Integer`, taking the `Integer` by value.
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((-Integer::ZERO).to_string(), "0");
///     assert_eq!((-Integer::from(123)).to_string(), "-123");
///     assert_eq!((-Integer::from(-123)).to_string(), "123");
/// }
/// ```
impl Neg for Integer {
    type Output = Integer;

    fn neg(mut self) -> Integer {
        if self.abs != 0 as Limb {
            self.sign.not_assign();
        }
        self
    }
}

/// Returns the negative of an `Integer`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((-&Integer::ZERO).to_string(), "0");
///     assert_eq!((-&Integer::from(123)).to_string(), "-123");
///     assert_eq!((-&Integer::from(-123)).to_string(), "123");
/// }
/// ```
impl<'a> Neg for &'a Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        if self.abs == 0 as Limb {
            Integer::ZERO
        } else {
            Integer {
                sign: !self.sign,
                abs: self.abs.clone(),
            }
        }
    }
}

/// Replaces an `Integer` with its negative.
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
/// use malachite_base::num::traits::{NegAssign, Zero};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x.neg_assign();
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Integer::from(123);
///     x.neg_assign();
///     assert_eq!(x.to_string(), "-123");
///
///     let mut x = Integer::from(-123);
///     x.neg_assign();
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl NegAssign for Integer {
    fn neg_assign(&mut self) {
        if self.abs != 0 as Limb {
            self.sign.not_assign();
        }
    }
}
