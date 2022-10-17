use crate::Rational;
use malachite_base::num::arithmetic::traits::{
    RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;

impl RoundToMultipleOfPowerOf2<i64> for Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of $2^k$ according to a specified rounding
    /// mode. The [`Rational`] is taken by value.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = 2^k \operatorname{sgn}(q) \lfloor |q| \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = 2^k \operatorname{sgn}(q) \lceil |q| \rceil.$
    ///
    /// $f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Floor).to_string(),
    ///     "25/8"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Down).to_string(),
    ///     "25/8"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Ceiling).to_string(),
    ///     "13/4"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Up).to_string(),
    ///     "13/4"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple_of_power_of_2(-3, RoundingMode::Nearest).to_string(),
    ///     "25/8"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple_of_power_of_2(mut self, pow: i64, rm: RoundingMode) -> Rational {
        self.round_to_multiple_of_power_of_2_assign(pow, rm);
        self
    }
}

impl<'a> RoundToMultipleOfPowerOf2<i64> for &'a Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of $2^k$ according to a specified rounding
    /// mode. The [`Rational`] is taken by reference.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = 2^k \operatorname{sgn}(q) \lfloor |q| \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = 2^k \operatorname{sgn}(q) \lceil |q| \rceil.$
    ///
    /// $f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Floor).to_string(),
    ///     "25/8"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Down).to_string(),
    ///     "25/8"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Ceiling).to_string(),
    ///     "13/4"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Up).to_string(),
    ///     "13/4"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, RoundingMode::Nearest).to_string(),
    ///     "25/8"
    /// );
    /// ```
    fn round_to_multiple_of_power_of_2(self, pow: i64, rm: RoundingMode) -> Rational {
        Rational::from(Integer::rounding_from(self >> pow, rm)) << pow
    }
}

impl RoundToMultipleOfPowerOf2Assign<i64> for Rational {
    /// Rounds a [`Rational`] to a multiple of $2^k$ in place, according to a specified rounding
    /// mode.
    ///
    /// See the [`RoundToMultipleOfPowerOf2`](RoundToMultipleOfPowerOf2) documentation for details.
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Floor);
    /// assert_eq!(x.to_string(), "25/8");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Down);
    /// assert_eq!(x.to_string(), "25/8");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Ceiling);
    /// assert_eq!(x.to_string(), "13/4");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Up);
    /// assert_eq!(x.to_string(), "13/4");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_of_power_of_2_assign(-3, RoundingMode::Nearest);
    /// assert_eq!(x.to_string(), "25/8");
    /// ```
    fn round_to_multiple_of_power_of_2_assign(&mut self, pow: i64, rm: RoundingMode) {
        *self >>= pow;
        *self = Rational::from(Integer::rounding_from(&*self, rm)) << pow
    }
}
