use std::ops::Neg;

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::NotAssign;

use integer::Integer;

impl Neg for Integer {
    type Output = Integer;

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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((-Integer::ZERO).to_string(), "0");
    /// assert_eq!((-Integer::from(123)).to_string(), "-123");
    /// assert_eq!((-Integer::from(-123)).to_string(), "123");
    /// ```
    fn neg(mut self) -> Integer {
        if self.abs != 0 {
            self.sign.not_assign();
        }
        self
    }
}

impl<'a> Neg for &'a Integer {
    type Output = Integer;

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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((-&Integer::ZERO).to_string(), "0");
    /// assert_eq!((-&Integer::from(123)).to_string(), "-123");
    /// assert_eq!((-&Integer::from(-123)).to_string(), "123");
    /// ```
    fn neg(self) -> Integer {
        if self.abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: !self.sign,
                abs: self.abs.clone(),
            }
        }
    }
}

impl NegAssign for Integer {
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
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "0");
    ///
    /// let mut x = Integer::from(123);
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "-123");
    ///
    /// let mut x = Integer::from(-123);
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "123");
    /// ```
    fn neg_assign(&mut self) {
        if self.abs != 0 {
            self.sign.not_assign();
        }
    }
}
