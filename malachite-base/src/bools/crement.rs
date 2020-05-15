use crement::Crementable;

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
