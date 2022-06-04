use integer::Integer;
use std::ops::{Mul, MulAssign};

impl Mul<Integer> for Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::ONE * Integer::from(123), 123);
    /// assert_eq!(Integer::from(123) * Integer::ZERO, 0);
    /// assert_eq!(Integer::from(123) * Integer::from(-456), -56088);
    /// assert_eq!(
    ///     (Integer::from(-123456789000i64) * Integer::from(-987654321000i64)).to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(mut self, other: Integer) -> Integer {
        self *= other;
        self
    }
}

impl<'a> Mul<&'a Integer> for Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::ONE * &Integer::from(123), 123);
    /// assert_eq!(Integer::from(123) * &Integer::ZERO, 0);
    /// assert_eq!(Integer::from(123) * &Integer::from(-456), -56088);
    /// assert_eq!(
    ///     (Integer::from(-123456789000i64) * &Integer::from(-987654321000i64)).to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(mut self, other: &'a Integer) -> Integer {
        self *= other;
        self
    }
}

impl<'a> Mul<Integer> for &'a Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(&Integer::ONE * Integer::from(123), 123);
    /// assert_eq!(&Integer::from(123) * Integer::ZERO, 0);
    /// assert_eq!(&Integer::from(123) * Integer::from(-456), -56088);
    /// assert_eq!(
    ///     (&Integer::from(-123456789000i64) * Integer::from(-987654321000i64)).to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

impl<'a, 'b> Mul<&'a Integer> for &'b Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(&Integer::ONE * &Integer::from(123), 123);
    /// assert_eq!(&Integer::from(123) * &Integer::ZERO, 0);
    /// assert_eq!(&Integer::from(123) * &Integer::from(-456), -56088);
    /// assert_eq!(
    ///     (&Integer::from(-123456789000i64) * &Integer::from(-987654321000i64)).to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(self, other: &'a Integer) -> Integer {
        let product_abs = &self.abs * &other.abs;
        Integer {
            sign: self.sign == other.sign || product_abs == 0,
            abs: product_abs,
        }
    }
}

impl MulAssign<Integer> for Integer {
    /// Multiplies an [`Integer`] by an [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value.
    ///
    /// $$
    /// x \gets = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// let mut x = Integer::NEGATIVE_ONE;
    /// x *= Integer::from(1000);
    /// x *= Integer::from(2000);
    /// x *= Integer::from(3000);
    /// x *= Integer::from(4000);
    /// assert_eq!(x, -24000000000000i64);
    /// ```
    fn mul_assign(&mut self, other: Integer) {
        self.abs *= other.abs;
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}

impl<'a> MulAssign<&'a Integer> for Integer {
    /// Multiplies an [`Integer`] by an [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// let mut x = Integer::NEGATIVE_ONE;
    /// x *= &Integer::from(1000);
    /// x *= &Integer::from(2000);
    /// x *= &Integer::from(3000);
    /// x *= &Integer::from(4000);
    /// assert_eq!(x, -24000000000000i64);
    /// ```
    fn mul_assign(&mut self, other: &'a Integer) {
        self.abs *= &other.abs;
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}
