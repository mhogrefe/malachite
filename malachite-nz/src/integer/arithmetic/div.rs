use std::ops::{Div, DivAssign};

use integer::Integer;

impl Div<Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking both `Integer`s by value. The quotient is
    /// rounded towards zero. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((Integer::from(23) / Integer::from(10)).to_string(), "2");
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!((Integer::from(23) / Integer::from(-10)).to_string(), "-2");
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!((Integer::from(-23) / Integer::from(10)).to_string(), "-2");
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((Integer::from(-23) / Integer::from(-10)).to_string(), "2");
    /// ```
    #[inline]
    fn div(mut self, other: Integer) -> Integer {
        self /= other;
        self
    }
}

impl<'a> Div<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking the first `Integer` by value and the second by
    /// reference. The quotient is rounded towards zero. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((Integer::from(23) / &Integer::from(10)).to_string(), "2");
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!((Integer::from(23) / &Integer::from(-10)).to_string(), "-2");
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!((Integer::from(-23) / &Integer::from(10)).to_string(), "-2");
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((Integer::from(-23) / &Integer::from(-10)).to_string(), "2");
    /// ```
    #[inline]
    fn div(mut self, other: &'a Integer) -> Integer {
        self /= other;
        self
    }
}

impl<'a> Div<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking the first `Integer` by reference and the second
    /// by value. The quotient is rounded towards zero. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Integer::from(23) / Integer::from(10)).to_string(), "2");
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!((&Integer::from(23) / Integer::from(-10)).to_string(), "-2");
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!((&Integer::from(-23) / Integer::from(10)).to_string(), "-2");
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((&Integer::from(-23) / Integer::from(-10)).to_string(), "2");
    /// ```
    #[inline]
    fn div(self, other: Integer) -> Integer {
        let q = &self.abs / other.abs;
        if self.sign == other.sign {
            Integer::from(q)
        } else {
            -q
        }
    }
}

impl<'a, 'b> Div<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an `Integer` by an `Integer`, taking both `Integer`s by reference. The quotient is
    /// rounded towards zero. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Integer::from(23) / &Integer::from(10)).to_string(), "2");
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!((&Integer::from(23) / &Integer::from(-10)).to_string(), "-2");
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!((&Integer::from(-23) / &Integer::from(10)).to_string(), "-2");
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((&Integer::from(-23) / &Integer::from(-10)).to_string(), "2");
    /// ```
    #[inline]
    fn div(self, other: &'b Integer) -> Integer {
        let q = &self.abs / &other.abs;
        if self.sign == other.sign {
            Integer::from(q)
        } else {
            -q
        }
    }
}

impl DivAssign<Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place, taking the second `Integer` by value. The
    /// quotient is rounded towards zero. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x /= Integer::from(10);
    /// assert_eq!(x.to_string(), "2");
    ///
    /// // -2 * -10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x /= Integer::from(-10);
    /// assert_eq!(x.to_string(), "-2");
    ///
    /// // -2 * 10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x /= Integer::from(10);
    /// assert_eq!(x.to_string(), "-2");
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x /= Integer::from(-10);
    /// assert_eq!(x.to_string(), "2");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Integer) {
        self.abs /= other.abs;
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}

impl<'a> DivAssign<&'a Integer> for Integer {
    /// Divides an `Integer` by an `Integer` in place, taking the second `Integer` by reference and
    /// returning the remainder. The quotient is rounded towards zero. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x /= &Integer::from(10);
    /// assert_eq!(x.to_string(), "2");
    ///
    /// // -2 * -10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x /= &Integer::from(-10);
    /// assert_eq!(x.to_string(), "-2");
    ///
    /// // -2 * 10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x /= &Integer::from(10);
    /// assert_eq!(x.to_string(), "-2");
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x /= &Integer::from(-10);
    /// assert_eq!(x.to_string(), "2");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &'a Integer) {
        self.abs /= &other.abs;
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}
