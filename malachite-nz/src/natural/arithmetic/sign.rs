use crate::natural::Natural;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::Sign;

impl Sign for Natural {
    /// Compares a [`Natural`] to zero.
    ///
    /// Returns `Greater` or `Equal` depending on whether the [`Natural`] is positive or zero,
    /// respectively.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(Natural::ZERO.sign(), Ordering::Equal);
    /// assert_eq!(Natural::from(123u32).sign(), Ordering::Greater);
    /// ```
    fn sign(&self) -> Ordering {
        if *self == 0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
