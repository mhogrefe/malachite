use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::Sign;

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
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use core::cmp::Ordering;
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
