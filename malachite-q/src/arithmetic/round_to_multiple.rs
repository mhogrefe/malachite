use crate::Rational;
use malachite_base::num::arithmetic::traits::{NegAssign, RoundToMultiple, RoundToMultipleAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;

impl RoundToMultiple<Rational> for Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`], according to a
    /// specified rounding mode. Both [`Rational`]s are taken by value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \Z$.
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
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(-5).round_to_multiple(Rational::ZERO, RoundingMode::Down), 0);
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    /// assert_eq!(
    ///     q.clone().round_to_multiple(hundredth.clone(), RoundingMode::Down).to_string(),
    ///     "157/50"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple(hundredth.clone(), RoundingMode::Floor).to_string(),
    ///     "157/50"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple(hundredth.clone(), RoundingMode::Up).to_string(),
    ///     "63/20"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple(hundredth.clone(), RoundingMode::Ceiling).to_string(),
    ///     "63/20"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple(hundredth.clone(), RoundingMode::Nearest).to_string(),
    ///     "157/50"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: Rational, rm: RoundingMode) -> Rational {
        self.round_to_multiple_assign(other, rm);
        self
    }
}

impl<'a> RoundToMultiple<&'a Rational> for Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`], according to a
    /// specified rounding mode. The first [`Rational`] is taken by value and the second by
    /// reference.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \Z$.
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
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(-5).round_to_multiple(&Rational::ZERO, RoundingMode::Down), 0);
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    /// assert_eq!(
    ///     q.clone().round_to_multiple(&hundredth, RoundingMode::Down).to_string(),
    ///     "157/50"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple(&hundredth, RoundingMode::Floor).to_string(),
    ///     "157/50"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple(&hundredth, RoundingMode::Up).to_string(),
    ///     "63/20"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple(&hundredth, RoundingMode::Ceiling).to_string(),
    ///     "63/20"
    /// );
    /// assert_eq!(
    ///     q.clone().round_to_multiple(&hundredth, RoundingMode::Nearest).to_string(),
    ///     "157/50"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: &'a Rational, rm: RoundingMode) -> Rational {
        self.round_to_multiple_assign(other, rm);
        self
    }
}

impl<'a> RoundToMultiple<Rational> for &'a Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`], according to a
    /// specified rounding mode. The first [`Rational`] is taken by reference and the second by
    /// value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \Z$.
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
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(-5).round_to_multiple(Rational::ZERO, RoundingMode::Down), 0);
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), RoundingMode::Down).to_string(),
    ///     "157/50"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), RoundingMode::Floor).to_string(),
    ///     "157/50"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), RoundingMode::Up).to_string(),
    ///     "63/20"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), RoundingMode::Ceiling).to_string(),
    ///     "63/20"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), RoundingMode::Nearest).to_string(),
    ///     "157/50"
    /// );
    /// ```
    fn round_to_multiple(self, other: Rational, mut rm: RoundingMode) -> Rational {
        if *self == other {
            return self.clone();
        }
        if other == 0u32 {
            if rm == RoundingMode::Down
                || rm == RoundingMode::Nearest
                || rm
                    == if *self >= 0u32 {
                        RoundingMode::Floor
                    } else {
                        RoundingMode::Ceiling
                    }
            {
                return Rational::ZERO;
            } else {
                panic!("Cannot round {} to zero using RoundingMode {}", self, rm);
            }
        }
        if !other.sign {
            rm.neg_assign();
        }
        Rational::from(Integer::rounding_from(self / &other, rm)) * other
    }
}

impl<'a, 'b> RoundToMultiple<&'b Rational> for &'a Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`], according to a
    /// specified rounding mode. Both [`Rational`]s are taken by reference.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \Z$.
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
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(-5).round_to_multiple(Rational::ZERO, RoundingMode::Down), 0);
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    /// assert_eq!((&q).round_to_multiple(&hundredth, RoundingMode::Down).to_string(), "157/50");
    /// assert_eq!((&q).round_to_multiple(&hundredth, RoundingMode::Floor).to_string(), "157/50");
    /// assert_eq!((&q).round_to_multiple(&hundredth, RoundingMode::Up).to_string(), "63/20");
    /// assert_eq!((&q).round_to_multiple(&hundredth, RoundingMode::Ceiling).to_string(), "63/20");
    /// assert_eq!(
    ///     (&q).round_to_multiple(&hundredth, RoundingMode::Nearest).to_string(),
    ///     "157/50"
    /// );
    /// ```
    fn round_to_multiple(self, other: &'b Rational, mut rm: RoundingMode) -> Rational {
        if self == other {
            return self.clone();
        }
        if *other == 0u32 {
            if rm == RoundingMode::Down
                || rm == RoundingMode::Nearest
                || rm
                    == if *self >= 0u32 {
                        RoundingMode::Floor
                    } else {
                        RoundingMode::Ceiling
                    }
            {
                return Rational::ZERO;
            } else {
                panic!("Cannot round {} to zero using RoundingMode {}", self, rm);
            }
        }
        if !other.sign {
            rm.neg_assign();
        }
        Rational::from(Integer::rounding_from(self / other, rm)) * other
    }
}

impl RoundToMultipleAssign<Rational> for Rational {
    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`] in place, according to
    /// a  specified rounding mode. The [`Rational`] on the right-hand side is taken by value.
    ///
    /// See the [`RoundToMultiple`](malachite_base::num::arithmetic::traits::RoundToMultiple)
    /// documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from(-5);
    /// x.round_to_multiple_assign(Rational::ZERO, RoundingMode::Down);
    /// assert_eq!(x, 0);
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(hundredth.clone(), RoundingMode::Down);
    /// assert_eq!(x.to_string(), "157/50");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(hundredth.clone(), RoundingMode::Floor);
    /// assert_eq!(x.to_string(), "157/50");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(hundredth.clone(), RoundingMode::Up);
    /// assert_eq!(x.to_string(), "63/20");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(hundredth.clone(), RoundingMode::Ceiling);
    /// assert_eq!(x.to_string(), "63/20");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(hundredth.clone(), RoundingMode::Nearest);
    /// assert_eq!(x.to_string(), "157/50");
    /// ```
    fn round_to_multiple_assign(&mut self, other: Rational, mut rm: RoundingMode) {
        if *self == other {
            return;
        }
        if other == 0u32 {
            if rm == RoundingMode::Down
                || rm == RoundingMode::Nearest
                || rm
                    == if *self >= 0u32 {
                        RoundingMode::Floor
                    } else {
                        RoundingMode::Ceiling
                    }
            {
                *self = Rational::ZERO;
                return;
            } else {
                panic!("Cannot round {} to zero using RoundingMode {}", self, rm);
            }
        }
        if !other.sign {
            rm.neg_assign();
        }
        *self /= &other;
        *self = Rational::from(Integer::rounding_from(&*self, rm)) * other;
    }
}

impl<'a> RoundToMultipleAssign<&'a Rational> for Rational {
    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`] in place, according to
    /// a  specified rounding mode. The [`Rational`] on the right-hand side is taken by reference.
    ///
    /// See the [`RoundToMultiple`](malachite_base::num::arithmetic::traits::RoundToMultiple)
    /// documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from(-5);
    /// x.round_to_multiple_assign(Rational::ZERO, RoundingMode::Down);
    /// assert_eq!(x, 0);
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(&hundredth, RoundingMode::Down);
    /// assert_eq!(x.to_string(), "157/50");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(&hundredth, RoundingMode::Floor);
    /// assert_eq!(x.to_string(), "157/50");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(&hundredth, RoundingMode::Up);
    /// assert_eq!(x.to_string(), "63/20");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(&hundredth, RoundingMode::Ceiling);
    /// assert_eq!(x.to_string(), "63/20");
    ///
    /// let mut x = q.clone();
    /// x.round_to_multiple_assign(&hundredth, RoundingMode::Nearest);
    /// assert_eq!(x.to_string(), "157/50");
    /// ```
    fn round_to_multiple_assign(&mut self, other: &'a Rational, mut rm: RoundingMode) {
        if self == other {
            return;
        }
        if *other == 0u32 {
            if rm == RoundingMode::Down
                || rm == RoundingMode::Nearest
                || rm
                    == if *self >= 0u32 {
                        RoundingMode::Floor
                    } else {
                        RoundingMode::Ceiling
                    }
            {
                *self = Rational::ZERO;
                return;
            } else {
                panic!("Cannot round {} to zero using RoundingMode {}", self, rm);
            }
        }
        if !other.sign {
            rm.neg_assign();
        }
        *self /= other;
        *self = Rational::from(Integer::rounding_from(&*self, rm)) * other;
    }
}
