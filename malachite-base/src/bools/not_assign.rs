use crate::num::logic::traits::NotAssign;

impl NotAssign for bool {
    /// Replaces a [`bool`] by its opposite.
    ///
    /// $b \gets \lnot b$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
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
    #[inline]
    fn not_assign(&mut self) {
        *self = !*self
    }
}
