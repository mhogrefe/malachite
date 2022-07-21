use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign};
use malachite_base::rounding_modes::RoundingMode;

impl DivRound<Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and rounding according
    /// to a specified rounding mode.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $$
    /// f(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(4), RoundingMode::Down), -2);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(Integer::from(3), RoundingMode::Floor),
    ///     -333333333334i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(4), RoundingMode::Up), -3);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(Integer::from(3), RoundingMode::Ceiling),
    ///     -333333333333i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(5), RoundingMode::Exact), -2);
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(3), RoundingMode::Nearest), -3);
    /// assert_eq!(Integer::from(-20).div_round(Integer::from(3), RoundingMode::Nearest), -7);
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(4), RoundingMode::Nearest), -2);
    /// assert_eq!(Integer::from(-14).div_round(Integer::from(4), RoundingMode::Nearest), -4);
    ///
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(-4), RoundingMode::Down), 2);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(Integer::from(-3), RoundingMode::Floor),
    ///     333333333333i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(-4), RoundingMode::Up), 3);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(Integer::from(-3), RoundingMode::Ceiling),
    ///     333333333334i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(-5), RoundingMode::Exact), 2);
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(-3), RoundingMode::Nearest), 3);
    /// assert_eq!(Integer::from(-20).div_round(Integer::from(-3), RoundingMode::Nearest), 7);
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(-4), RoundingMode::Nearest), 2);
    /// assert_eq!(Integer::from(-14).div_round(Integer::from(-4), RoundingMode::Nearest), 4);
    /// ```
    #[inline]
    fn div_round(mut self, other: Integer, rm: RoundingMode) -> Integer {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and rounding according to a specified rounding mode.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $$
    /// f(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(4), RoundingMode::Down), -2);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(&Integer::from(3), RoundingMode::Floor),
    ///     -333333333334i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(4), RoundingMode::Up), -3);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(&Integer::from(3), RoundingMode::Ceiling),
    ///     -333333333333i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(5), RoundingMode::Exact), -2);
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(3), RoundingMode::Nearest), -3);
    /// assert_eq!(Integer::from(-20).div_round(&Integer::from(3), RoundingMode::Nearest), -7);
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(4), RoundingMode::Nearest), -2);
    /// assert_eq!(Integer::from(-14).div_round(&Integer::from(4), RoundingMode::Nearest), -4);
    ///
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(-4), RoundingMode::Down), 2);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(&Integer::from(-3), RoundingMode::Floor),
    ///     333333333333i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(-4), RoundingMode::Up), 3);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(&Integer::from(-3), RoundingMode::Ceiling),
    ///     333333333334i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(-5), RoundingMode::Exact), 2);
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(-3), RoundingMode::Nearest), 3);
    /// assert_eq!(Integer::from(-20).div_round(&Integer::from(-3), RoundingMode::Nearest), 7);
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(-4), RoundingMode::Nearest), 2);
    /// assert_eq!(Integer::from(-14).div_round(&Integer::from(-4), RoundingMode::Nearest), 4);
    /// ```
    #[inline]
    fn div_round(mut self, other: &'a Integer, rm: RoundingMode) -> Integer {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and rounding according to a specified rounding mode.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $$
    /// f(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-10)).div_round(Integer::from(4), RoundingMode::Down), -2);
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(Integer::from(3), RoundingMode::Floor),
    ///     -333333333334i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(4), RoundingMode::Up), -3);
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(Integer::from(3), RoundingMode::Ceiling),
    ///     -333333333333i64
    /// );
    /// assert_eq!((&Integer::from(-10)).div_round(Integer::from(5), RoundingMode::Exact), -2);
    /// assert_eq!((&Integer::from(-10)).div_round(Integer::from(3), RoundingMode::Nearest), -3);
    /// assert_eq!((&Integer::from(-20)).div_round(Integer::from(3), RoundingMode::Nearest), -7);
    /// assert_eq!((&Integer::from(-10)).div_round(Integer::from(4), RoundingMode::Nearest), -2);
    /// assert_eq!((&Integer::from(-14)).div_round(Integer::from(4), RoundingMode::Nearest), -4);
    ///
    /// assert_eq!((&Integer::from(-10)).div_round(Integer::from(-4), RoundingMode::Down), 2);
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(Integer::from(-3), RoundingMode::Floor),
    ///     333333333333i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(Integer::from(-4), RoundingMode::Up), 3);
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(Integer::from(-3), RoundingMode::Ceiling),
    ///     333333333334i64
    /// );
    /// assert_eq!((&Integer::from(-10)).div_round(Integer::from(-5), RoundingMode::Exact), 2);
    /// assert_eq!((&Integer::from(-10)).div_round(Integer::from(-3), RoundingMode::Nearest), 3);
    /// assert_eq!((&Integer::from(-20)).div_round(Integer::from(-3), RoundingMode::Nearest), 7);
    /// assert_eq!((&Integer::from(-10)).div_round(Integer::from(-4), RoundingMode::Nearest), 2);
    /// assert_eq!((&Integer::from(-14)).div_round(Integer::from(-4), RoundingMode::Nearest), 4);
    /// ```
    fn div_round(self, other: Integer, rm: RoundingMode) -> Integer {
        let q_sign = self.sign == other.sign;
        Integer::from_sign_and_abs(
            q_sign,
            (&self.abs).div_round(other.abs, if q_sign { rm } else { -rm }),
        )
    }
}

impl<'a, 'b> DivRound<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and rounding
    /// according to a specified rounding mode.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $$
    /// f(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-10)).div_round(&Integer::from(4), RoundingMode::Down), -2);
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(&Integer::from(3), RoundingMode::Floor),
    ///     -333333333334i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(4), RoundingMode::Up), -3);
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(&Integer::from(3), RoundingMode::Ceiling),
    ///     -333333333333i64
    /// );
    /// assert_eq!((&Integer::from(-10)).div_round(&Integer::from(5), RoundingMode::Exact), -2);
    /// assert_eq!((&Integer::from(-10)).div_round(&Integer::from(3), RoundingMode::Nearest), -3);
    /// assert_eq!((&Integer::from(-20)).div_round(&Integer::from(3), RoundingMode::Nearest), -7);
    /// assert_eq!((&Integer::from(-10)).div_round(&Integer::from(4), RoundingMode::Nearest), -2);
    /// assert_eq!((&Integer::from(-14)).div_round(&Integer::from(4), RoundingMode::Nearest), -4);
    ///
    /// assert_eq!((&Integer::from(-10)).div_round(&Integer::from(-4), RoundingMode::Down), 2);
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(&Integer::from(-3), RoundingMode::Floor),
    ///     333333333333i64
    /// );
    /// assert_eq!(Integer::from(-10).div_round(&Integer::from(-4), RoundingMode::Up), 3);
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(&Integer::from(-3), RoundingMode::Ceiling),
    ///     333333333334i64
    /// );
    /// assert_eq!((&Integer::from(-10)).div_round(&Integer::from(-5), RoundingMode::Exact), 2);
    /// assert_eq!((&Integer::from(-10)).div_round(&Integer::from(-3), RoundingMode::Nearest), 3);
    /// assert_eq!((&Integer::from(-20)).div_round(&Integer::from(-3), RoundingMode::Nearest), 7);
    /// assert_eq!((&Integer::from(-10)).div_round(&Integer::from(-4), RoundingMode::Nearest), 2);
    /// assert_eq!((&Integer::from(-14)).div_round(&Integer::from(-4), RoundingMode::Nearest), 4);
    /// ```
    fn div_round(self, other: &'b Integer, rm: RoundingMode) -> Integer {
        let q_sign = self.sign == other.sign;
        Integer::from_sign_and_abs(
            q_sign,
            (&self.abs).div_round(&other.abs, if q_sign { rm } else { -rm }),
        )
    }
}

impl DivRoundAssign<Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value and rounding according to a specified rounding mode.
    ///
    /// See the [`DivRound`](malachite_base::num::arithmetic::traits::DivRound) documentation for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRoundAssign, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(4), RoundingMode::Down);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// n.div_round_assign(Integer::from(3), RoundingMode::Floor);
    /// assert_eq!(n, -333333333334i64);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(4), RoundingMode::Up);
    /// assert_eq!(n, -3);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// n.div_round_assign(Integer::from(3), RoundingMode::Ceiling);
    /// assert_eq!(n, -333333333333i64);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(5), RoundingMode::Exact);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(n, -3);
    ///
    /// let mut n = Integer::from(-20);
    /// n.div_round_assign(Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(n, -7);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = Integer::from(-14);
    /// n.div_round_assign(Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(n, -4);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-4), RoundingMode::Down);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// n.div_round_assign(Integer::from(-3), RoundingMode::Floor);
    /// assert_eq!(n, 333333333333i64);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-4), RoundingMode::Up);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// n.div_round_assign(Integer::from(-3), RoundingMode::Ceiling);
    /// assert_eq!(n, 333333333334i64);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-5), RoundingMode::Exact);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Integer::from(-20);
    /// n.div_round_assign(Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Integer::from(-14);
    /// n.div_round_assign(Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: Integer, rm: RoundingMode) {
        let q_sign = self.sign == other.sign;
        self.abs
            .div_round_assign(other.abs, if q_sign { rm } else { -rm });
        self.sign = q_sign || self.abs == 0;
    }
}

impl<'a> DivRoundAssign<&'a Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference and rounding according to a specified rounding mode.
    ///
    /// See the [`DivRound`](malachite_base::num::arithmetic::traits::DivRound) documentation for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRoundAssign, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(4), RoundingMode::Down);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// n.div_round_assign(&Integer::from(3), RoundingMode::Floor);
    /// assert_eq!(n, -333333333334i64);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(4), RoundingMode::Up);
    /// assert_eq!(n, -3);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// n.div_round_assign(&Integer::from(3), RoundingMode::Ceiling);
    /// assert_eq!(n, -333333333333i64);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(5), RoundingMode::Exact);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(n, -3);
    ///
    /// let mut n = Integer::from(-20);
    /// n.div_round_assign(&Integer::from(3), RoundingMode::Nearest);
    /// assert_eq!(n, -7);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = Integer::from(-14);
    /// n.div_round_assign(&Integer::from(4), RoundingMode::Nearest);
    /// assert_eq!(n, -4);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-4), RoundingMode::Down);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// n.div_round_assign(&Integer::from(-3), RoundingMode::Floor);
    /// assert_eq!(n, 333333333333i64);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-4), RoundingMode::Up);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// n.div_round_assign(&Integer::from(-3), RoundingMode::Ceiling);
    /// assert_eq!(n, 333333333334i64);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-5), RoundingMode::Exact);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Integer::from(-20);
    /// n.div_round_assign(&Integer::from(-3), RoundingMode::Nearest);
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Integer::from(-10);
    /// n.div_round_assign(&Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Integer::from(-14);
    /// n.div_round_assign(&Integer::from(-4), RoundingMode::Nearest);
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: &'a Integer, rm: RoundingMode) {
        let q_sign = self.sign == other.sign;
        self.abs
            .div_round_assign(&other.abs, if q_sign { rm } else { -rm });
        self.sign = q_sign || self.abs == 0;
    }
}
