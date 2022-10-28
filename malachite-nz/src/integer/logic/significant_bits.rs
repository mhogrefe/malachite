use crate::integer::Integer;
use malachite_base::num::logic::traits::SignificantBits;

impl<'a> SignificantBits for &'a Integer {
    /// Returns the number of significant bits of an [`Integer`]'s absolute value.
    ///
    /// $$
    /// f(n) = \\begin{cases}
    ///     0 & \text{if} \\quad n = 0, \\\\
    ///     \lfloor \log_2 |n| \rfloor + 1 & \\text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.significant_bits(), 0);
    /// assert_eq!(Integer::from(100).significant_bits(), 7);
    /// assert_eq!(Integer::from(-100).significant_bits(), 7);
    /// ```
    fn significant_bits(self) -> u64 {
        self.abs.significant_bits()
    }
}
