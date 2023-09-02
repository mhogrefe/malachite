use crate::Float;
use crate::InnerFloat::{Finite, Infinity, Zero};
use malachite_base::num::arithmetic::traits::Sign;
use std::cmp::Ordering;

impl Sign for Float {
    /// Returns the sign of a [`Float`].
    ///
    /// Returns `Greater` if the sign is positive and `Less` if the sign is negative. Never returns
    /// `Equal`. Positive infinity and positive zero have a positive sign, and negative infinity
    /// and negative zero have a negative sign.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is NaN.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero
    /// };
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Float::INFINITY.sign(), Ordering::Greater);
    /// assert_eq!(Float::NEGATIVE_INFINITY.sign(), Ordering::Less);
    /// assert_eq!(Float::ZERO.sign(), Ordering::Greater);
    /// assert_eq!(Float::NEGATIVE_ZERO.sign(), Ordering::Less);
    /// assert_eq!(Float::ONE.sign(), Ordering::Greater);
    /// assert_eq!(Float::NEGATIVE_ONE.sign(), Ordering::Less);
    /// ```
    fn sign(&self) -> Ordering {
        match self {
            Float(Infinity { sign }) | Float(Zero { sign }) | Float(Finite { sign, .. }) => {
                if *sign {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            _ => panic!(),
        }
    }
}
