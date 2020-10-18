use integer::Integer;
use malachite_base::num::arithmetic::traits::{
    RoundToMultipleOfPowerOfTwo, RoundToMultipleOfPowerOfTwoAssign,
};
use malachite_base::rounding_modes::RoundingMode;

impl RoundToMultipleOfPowerOfTwo<u64> for Integer {
    type Output = Integer;

    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, taking
    /// `self` by value.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple_of_power_of_two(pow, RoundingMode::Exact)`
    /// `{ assert!(x.divisible_by_power_of_two(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = max(self.significant_bits(), pow)
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of two.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwo;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10).round_to_multiple_of_power_of_two(2, RoundingMode::Floor), 8);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple_of_power_of_two(2, RoundingMode::Ceiling),
    ///     -8
    /// );
    /// assert_eq!(Integer::from(10).round_to_multiple_of_power_of_two(2, RoundingMode::Down), 8);
    /// assert_eq!(Integer::from(-10).round_to_multiple_of_power_of_two(2, RoundingMode::Up), -12);
    /// assert_eq!(
    ///     Integer::from(10).round_to_multiple_of_power_of_two(2, RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     Integer::from(-12).round_to_multiple_of_power_of_two(2, RoundingMode::Exact),
    ///     -12
    /// );
    /// ```
    #[inline]
    fn round_to_multiple_of_power_of_two(mut self, pow: u64, rm: RoundingMode) -> Integer {
        self.round_to_multiple_of_power_of_two_assign(pow, rm);
        self
    }
}

impl<'a> RoundToMultipleOfPowerOfTwo<u64> for &'a Integer {
    type Output = Integer;

    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, taking
    /// `self` by reference.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple_of_power_of_two(pow, RoundingMode::Exact)`
    /// `{ assert!(x.divisible_by_power_of_two(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = max(self.significant_bits(), pow)
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of two.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwo;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(10)).round_to_multiple_of_power_of_two(2, RoundingMode::Floor),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple_of_power_of_two(2, RoundingMode::Ceiling),
    ///     -8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(10)).round_to_multiple_of_power_of_two(2, RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple_of_power_of_two(2, RoundingMode::Up),
    ///     -12
    /// );
    /// assert_eq!(
    ///     (&Integer::from(10)).round_to_multiple_of_power_of_two(2, RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-12)).round_to_multiple_of_power_of_two(2, RoundingMode::Exact),
    ///     -12
    /// );
    /// ```
    fn round_to_multiple_of_power_of_two(self, pow: u64, rm: RoundingMode) -> Integer {
        if self.sign {
            Integer {
                sign: self.sign,
                abs: (&self.abs).round_to_multiple_of_power_of_two(pow, rm),
            }
        } else {
            -(&self.abs).round_to_multiple_of_power_of_two(pow, -rm)
        }
    }
}

impl RoundToMultipleOfPowerOfTwoAssign<u64> for Integer {
    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, in
    /// place.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple_of_power_of_two_assign(pow, RoundingMode::Exact);`
    /// `assert!(x.divisible_by_power_of_two(pow));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = max(self.significant_bits(), pow)
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of two.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwoAssign;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut n = Integer::from(10);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Floor);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Integer::from(-10);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Ceiling);
    /// assert_eq!(n, -8);
    ///
    /// let mut n = Integer::from(10);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Down);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Integer::from(-10);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Up);
    /// assert_eq!(n, -12);
    ///
    /// let mut n = Integer::from(10);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Nearest);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Integer::from(-12);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Exact);
    /// assert_eq!(n, -12);
    /// ```
    fn round_to_multiple_of_power_of_two_assign(&mut self, pow: u64, rm: RoundingMode) {
        if self.sign {
            self.abs.round_to_multiple_of_power_of_two_assign(pow, rm);
        } else {
            self.abs.round_to_multiple_of_power_of_two_assign(pow, -rm);
            if self.abs == 0 {
                self.sign = true;
            }
        }
    }
}
