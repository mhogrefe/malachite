use integer::Integer;
use malachite_base::num::arithmetic::traits::Sign;
use std::cmp::Ordering;

impl Sign for Integer {
    /// Compares an [`Integer`] to zero.
    ///
    /// Returns `Greater`, `Equal`, or `Less`, depending on whether the [`Integer`] is positive,
    /// zero, or negative, respectively.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Integer::ZERO.sign(), Ordering::Equal);
    /// assert_eq!(Integer::from(123).sign(), Ordering::Greater);
    /// assert_eq!(Integer::from(-123).sign(), Ordering::Less);
    /// ```
    fn sign(&self) -> Ordering {
        if self.sign {
            if self.abs == 0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Less
        }
    }
}
