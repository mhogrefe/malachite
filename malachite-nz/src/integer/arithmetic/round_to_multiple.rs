use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::rounding_modes::RoundingMode;

impl RoundToMultiple<Integer> for Integer {
    type Output = Integer;

    /// Rounds an [`Integer`] to a multiple of another [`Integer`], according to a specified
    /// rounding mode. Both [`Integer`]s are taken by value.
    ///
    /// Let $q = \frac{x}{|y|}$:
    ///
    /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
    ///
    /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
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

    /// Rounds an [`Integer`] to a multiple of another [`Integer`], according to a specified
    /// rounding mode. The first [`Integer`] is taken by value and the second by reference.
    ///
    /// Let $q = \frac{x}{|y|}$:
    ///
    /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
    ///
    /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
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

    /// Rounds an [`Integer`] to a multiple of another [`Integer`], according to a specified
    /// rounding mode. The first [`Integer`] is taken by reference and the second by value.
    ///
    /// Let $q = \frac{x}{|y|}$:
    ///
    /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
    ///
    /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
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
        Integer::from_sign_and_abs(
            self.sign,
            (&self.abs).round_to_multiple(other.abs, if self.sign { rm } else { -rm }),
        )
    }
}

impl<'a, 'b> RoundToMultiple<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Rounds an [`Integer`] to a multiple of another [`Integer`], according to a specified
    /// rounding mode. Both [`Integer`]s are taken by reference.
    ///
    /// Let $q = \frac{x}{|y|}$:
    ///
    /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
    ///
    /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
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
        Integer::from_sign_and_abs(
            self.sign,
            (&self.abs).round_to_multiple(&other.abs, if self.sign { rm } else { -rm }),
        )
    }
}

impl RoundToMultipleAssign<Integer> for Integer {
    /// Rounds an [`Integer`] to a multiple of another [`Integer`] in place, according to a
    /// specified rounding mode. The [`Integer`] on the right-hand side is taken by value.
    ///
    /// See the [`RoundToMultiple`](malachite_base::num::arithmetic::traits::RoundToMultiple)
    /// documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_assign(other, RoundingMode::Exact);`
    /// - `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
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
    /// Rounds an [`Integer`] to a multiple of another [`Integer`] in place, according to a
    /// specified rounding mode. The [`Integer`] on the right-hand side is taken by reference.
    ///
    /// See the [`RoundToMultiple`](malachite_base::num::arithmetic::traits::RoundToMultiple)
    /// documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_assign(other, RoundingMode::Exact);`
    /// - `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
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
