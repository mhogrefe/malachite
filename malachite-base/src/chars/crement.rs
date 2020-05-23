use chars::char_to_contiguous_range::char_to_contiguous_range;
use chars::contiguous_range_to_char::contiguous_range_to_char;
use comparison::traits::Min;
use crement::Crementable;

impl Crementable for char {
    /// Increments this `char`, skipping over the surrogate range.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `self` is `char::MAX`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::crement::Crementable;
    ///
    /// let mut c = '\u{0}';
    /// c.increment();
    /// assert_eq!(c, '\u{1}');
    ///
    /// let mut c = 'a';
    /// c.increment();
    /// assert_eq!(c, 'b');
    /// ```
    fn increment(&mut self) {
        *self = contiguous_range_to_char(char_to_contiguous_range(*self) + 1)
            .expect("Cannot increment char::MAX")
    }

    /// Decrements this `char`, skipping over the surrogate range.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `self` is `'\u{0}'`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::crement::Crementable;
    ///
    /// let mut c = '\u{1}';
    /// c.decrement();
    /// assert_eq!(c, '\u{0}');
    ///
    /// let mut c = 'b';
    /// c.decrement();
    /// assert_eq!(c, 'a');
    /// ```
    #[allow(clippy::panic_params)]
    fn decrement(&mut self) {
        if *self == char::MIN {
            panic!("Cannot decrement char '{}'", *self);
        } else {
            *self = contiguous_range_to_char(char_to_contiguous_range(*self) - 1).unwrap();
        }
    }
}
