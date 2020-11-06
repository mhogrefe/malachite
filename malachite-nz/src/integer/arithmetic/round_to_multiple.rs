use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::rounding_modes::RoundingMode;

use integer::Integer;

impl RoundToMultiple<Integer> for Integer {
    type Output = Integer;

    /// Rounds an `Integer` to a multiple of an `Integer` according to a specified rounding mode,
    /// taking both `Integer`s by value.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple(other, RoundingMode::Exact)`
    /// `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-5).round_to_multiple(Integer::ZERO, RoundingMode::Down), 0);
    ///
    /// assert_eq!(Integer::from(-10).round_to_multiple(Integer::from(4), RoundingMode::Down), -8);
    /// assert_eq!(Integer::from(-10).round_to_multiple(Integer::from(4), RoundingMode::Up), -12);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(Integer::from(5), RoundingMode::Exact),
    ///     -10);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(Integer::from(3), RoundingMode::Nearest),
    ///     -9);
    /// assert_eq!(
    ///     Integer::from(-20).round_to_multiple(Integer::from(3), RoundingMode::Nearest),
    ///     -21);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(Integer::from(4), RoundingMode::Nearest),
    ///     -8);
    /// assert_eq!(
    ///     Integer::from(-14).round_to_multiple(Integer::from(4), RoundingMode::Nearest),
    ///     -16
    /// );
    ///
    /// assert_eq!(Integer::from(-10).round_to_multiple(Integer::from(-4), RoundingMode::Down), -8);
    /// assert_eq!(Integer::from(-10).round_to_multiple(Integer::from(-4), RoundingMode::Up), -12);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(Integer::from(-5), RoundingMode::Exact),
    ///     -10
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(Integer::from(-3), RoundingMode::Nearest),
    ///     -9
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).round_to_multiple(Integer::from(-3), RoundingMode::Nearest),
    ///     -21
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(Integer::from(-4), RoundingMode::Nearest),
    ///     -8
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).round_to_multiple(Integer::from(-4), RoundingMode::Nearest),
    ///     -16
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: Integer, rm: RoundingMode) -> Integer {
        self.round_to_multiple_assign(other, rm);
        self
    }
}

impl<'a> RoundToMultiple<&'a Integer> for Integer {
    type Output = Integer;

    /// Rounds an `Integer` to a multiple of an `Integer` according to a specified rounding mode,
    /// taking the first `Integer` by value and the second by reference.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple(other, RoundingMode::Exact)`
    /// `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-5).round_to_multiple(&Integer::ZERO, RoundingMode::Down), 0);
    ///
    /// assert_eq!(Integer::from(-10).round_to_multiple(&Integer::from(4), RoundingMode::Down), -8);
    /// assert_eq!(Integer::from(-10).round_to_multiple(&Integer::from(4), RoundingMode::Up), -12);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(&Integer::from(5), RoundingMode::Exact),
    ///     -10);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(&Integer::from(3), RoundingMode::Nearest),
    ///     -9);
    /// assert_eq!(
    ///     Integer::from(-20).round_to_multiple(&Integer::from(3), RoundingMode::Nearest),
    ///     -21);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(&Integer::from(4), RoundingMode::Nearest),
    ///     -8);
    /// assert_eq!(
    ///     Integer::from(-14).round_to_multiple(&Integer::from(4), RoundingMode::Nearest),
    ///     -16
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(&Integer::from(-4), RoundingMode::Down),
    ///     -8
    /// );
    /// assert_eq!(Integer::from(-10).round_to_multiple(&Integer::from(-4), RoundingMode::Up), -12);
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(&Integer::from(-5), RoundingMode::Exact),
    ///     -10
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(&Integer::from(-3), RoundingMode::Nearest),
    ///     -9
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).round_to_multiple(&Integer::from(-3), RoundingMode::Nearest),
    ///     -21
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple(&Integer::from(-4), RoundingMode::Nearest),
    ///     -8
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).round_to_multiple(&Integer::from(-4), RoundingMode::Nearest),
    ///     -16
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: &'a Integer, rm: RoundingMode) -> Integer {
        self.round_to_multiple_assign(other, rm);
        self
    }
}

impl<'a> RoundToMultiple<Integer> for &'a Integer {
    type Output = Integer;

    /// Rounds an `Integer` to a multiple of an `Integer` according to a specified rounding mode,
    /// taking the first `Integer` by reference and the second by value.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple(other, RoundingMode::Exact)`
    /// `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-5)).round_to_multiple(Integer::ZERO, RoundingMode::Down), 0);
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(4), RoundingMode::Down),
    ///     -8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(4), RoundingMode::Up),
    ///     -12
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(5), RoundingMode::Exact),
    ///     -10);
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(3), RoundingMode::Nearest),
    ///     -9);
    /// assert_eq!(
    ///     (&Integer::from(-20)).round_to_multiple(Integer::from(3), RoundingMode::Nearest),
    ///     -21);
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(4), RoundingMode::Nearest),
    ///     -8);
    /// assert_eq!(
    ///     (&Integer::from(-14)).round_to_multiple(Integer::from(4), RoundingMode::Nearest),
    ///     -16
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(-4), RoundingMode::Down),
    ///     -8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(-4), RoundingMode::Up),
    ///     -12
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(-5), RoundingMode::Exact),
    ///     -10
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(-3), RoundingMode::Nearest),
    ///     -9
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).round_to_multiple(Integer::from(-3), RoundingMode::Nearest),
    ///     -21
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(Integer::from(-4), RoundingMode::Nearest),
    ///     -8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).round_to_multiple(Integer::from(-4), RoundingMode::Nearest),
    ///     -16
    /// );
    /// ```
    fn round_to_multiple(self, other: Integer, rm: RoundingMode) -> Integer {
        if self.sign {
            Integer::from((&self.abs).round_to_multiple(other.abs, rm))
        } else {
            -(&self.abs).round_to_multiple(other.abs, -rm)
        }
    }
}

impl<'a, 'b> RoundToMultiple<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Rounds an `Integer` to a multiple of an `Integer` according to a specified rounding mode,
    /// taking both `Integer`s by reference.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple(other, RoundingMode::Exact)`
    /// `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-5)).round_to_multiple(&Integer::ZERO, RoundingMode::Down), 0);
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(4), RoundingMode::Down),
    ///     -8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(4), RoundingMode::Up),
    ///     -12
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(5), RoundingMode::Exact),
    ///     -10);
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(3), RoundingMode::Nearest),
    ///     -9);
    /// assert_eq!(
    ///     (&Integer::from(-20)).round_to_multiple(&Integer::from(3), RoundingMode::Nearest),
    ///     -21);
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(4), RoundingMode::Nearest),
    ///     -8);
    /// assert_eq!(
    ///     (&Integer::from(-14)).round_to_multiple(&Integer::from(4), RoundingMode::Nearest),
    ///     -16
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(-4), RoundingMode::Down),
    ///     -8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(-4), RoundingMode::Up),
    ///     -12
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(-5), RoundingMode::Exact),
    ///     -10
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(-3), RoundingMode::Nearest),
    ///     -9
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).round_to_multiple(&Integer::from(-3), RoundingMode::Nearest),
    ///     -21
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple(&Integer::from(-4), RoundingMode::Nearest),
    ///     -8
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).round_to_multiple(&Integer::from(-4), RoundingMode::Nearest),
    ///     -16
    /// );
    /// ```
    fn round_to_multiple(self, other: &'b Integer, rm: RoundingMode) -> Integer {
        if self.sign {
            Integer::from((&self.abs).round_to_multiple(&other.abs, rm))
        } else {
            -(&self.abs).round_to_multiple(&other.abs, -rm)
        }
    }
}

impl RoundToMultipleAssign<Integer> for Integer {
    /// Rounds an `Integer` to a multiple of another `Integer` in place according to a specified
    /// rounding mode, taking the `Integer` on the RHS by value.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple_assign(other, RoundingMode::Exact);`
    /// `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-5);
    /// x.round_to_multiple_assign(Integer::ZERO, RoundingMode::Down);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(4), RoundingMode::Down);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(4), RoundingMode::Up);
    /// assert_eq!(x, -12);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(5), RoundingMode::Exact);
    /// assert_eq!(x, -10);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(x, -9);
    ///
    /// let mut x = Integer::from(-20);
    /// x.round_to_multiple_assign(Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(x, -21);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-14);
    /// x.round_to_multiple_assign(Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(x, -16);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(-4), RoundingMode::Down);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(-4), RoundingMode::Up);
    /// assert_eq!(x, -12);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(-5), RoundingMode::Exact);
    /// assert_eq!(x, -10);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(x, -9);
    ///
    /// let mut x = Integer::from(-20);
    /// x.round_to_multiple_assign(Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(x, -21);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-14);
    /// x.round_to_multiple_assign(Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(x, -16);
    /// ```
    fn round_to_multiple_assign(&mut self, other: Integer, rm: RoundingMode) {
        if self.sign {
            self.abs.round_to_multiple_assign(other.abs, rm);
        } else {
            self.abs.round_to_multiple_assign(other.abs, -rm);
            self.sign = self.abs == 0;
        }
    }
}

impl<'a> RoundToMultipleAssign<&'a Integer> for Integer {
    /// Rounds an `Integer` to a multiple of another `Integer` in place according to a specified
    /// rounding mode, taking the `Integer` on the RHS by reference.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple_assign(other, RoundingMode::Exact);`
    /// `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-5);
    /// x.round_to_multiple_assign(&Integer::ZERO, RoundingMode::Down);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(4), RoundingMode::Down);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(4), RoundingMode::Up);
    /// assert_eq!(x, -12);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(5), RoundingMode::Exact);
    /// assert_eq!(x, -10);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(x, -9);
    ///
    /// let mut x = Integer::from(-20);
    /// x.round_to_multiple_assign(&Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(x, -21);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-14);
    /// x.round_to_multiple_assign(&Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(x, -16);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(-4), RoundingMode::Down);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(-4), RoundingMode::Up);
    /// assert_eq!(x, -12);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(-5), RoundingMode::Exact);
    /// assert_eq!(x, -10);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(x, -9);
    ///
    /// let mut x = Integer::from(-20);
    /// x.round_to_multiple_assign(&Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(x, -21);
    ///
    /// let mut x = Integer::from(-10);
    /// x.round_to_multiple_assign(&Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-14);
    /// x.round_to_multiple_assign(&Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(x, -16);
    /// ```
    fn round_to_multiple_assign(&mut self, other: &'a Integer, rm: RoundingMode) {
        if self.sign {
            self.abs.round_to_multiple_assign(&other.abs, rm);
        } else {
            self.abs.round_to_multiple_assign(&other.abs, -rm);
            self.sign = self.abs == 0;
        }
    }
}
