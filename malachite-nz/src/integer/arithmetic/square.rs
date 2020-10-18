use integer::Integer;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};

impl Square for Integer {
    type Output = Integer;

    /// Squares a `Integer`, taking it by value.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.square(), 0);
    /// assert_eq!(Integer::from(123).square(), 15129);
    /// assert_eq!(Integer::from(-123).square(), 15129);
    /// ```
    #[inline]
    fn square(mut self) -> Integer {
        self.square_assign();
        self
    }
}

impl<'a> Square for &'a Integer {
    type Output = Integer;

    /// Squares a `Integer`, taking it by reference.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO).square(), 0);
    /// assert_eq!((&Integer::from(123)).square(), 15129);
    /// assert_eq!((&Integer::from(-123)).square(), 15129);
    /// ```
    #[inline]
    fn square(self) -> Integer {
        Integer {
            sign: true,
            abs: (&self.abs).square(),
        }
    }
}

impl SquareAssign for Integer {
    /// Squares a `Integer` in place.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x.square_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Integer::from(123);
    /// x.square_assign();
    /// assert_eq!(x, 15129);
    ///
    /// let mut x = Integer::from(-123);
    /// x.square_assign();
    /// assert_eq!(x, 15129);
    /// ```
    fn square_assign(&mut self) {
        self.sign = true;
        self.abs.square_assign();
    }
}
