use comparison::{Max, Min};
use crement::Crementable;
use num::logic::traits::NotAssign;

impl NotAssign for bool {
    /// Replaces a `bool` with its negation.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::NotAssign;
    ///
    /// let mut b = false;
    /// b.not_assign();
    /// assert_eq!(b, true);
    ///
    /// let mut b = true;
    /// b.not_assign();
    /// assert_eq!(b, false);
    /// ```
    fn not_assign(&mut self) {
        *self = !*self
    }
}

/// The minimum value of a `bool`, false.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Min for bool {
    const MIN: bool = false;
}

/// The maximum value of a `bool`, true.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Max for bool {
    const MAX: bool = true;
}

impl Crementable for bool {
    /// Increments this `bool`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `self` is `true`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::crement::Crementable;
    ///
    /// let mut b = false;
    /// b.increment();
    /// assert_eq!(b, true);
    /// ```
    fn increment(&mut self) {
        if *self {
            panic!("Cannot increment bool 'true'");
        } else {
            *self = true;
        }
    }

    /// Decrements this `bool`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `self` is `false`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::crement::Crementable;
    ///
    /// let mut b = true;
    /// b.decrement();
    /// assert_eq!(b, false);
    /// ```
    #[allow(clippy::panic_params)]
    fn decrement(&mut self) {
        if *self {
            *self = false;
        } else {
            panic!("Cannot decrement bool 'false'");
        }
    }
}
