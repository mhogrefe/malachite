use malachite_base::num::arithmetic::traits::{CheckedSqrt, UnsignedAbs};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use Rational;

impl CheckedSqrt for Rational {
    type Output = Rational;

    /// Returns the the square root of a `Rational`, or `None` if the `Integer` is not the square
    /// of a `Rational`. The `Rational` is taken by value.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Q \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(99u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Rational::from(100u8).checked_sqrt().to_debug_string(), "Some(10)");
    /// assert_eq!(Rational::from(101u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Rational::from_signeds(22, 7).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Rational::from_signeds(25, 9).checked_sqrt().to_debug_string(), "Some(5/3)");
    /// ```
    fn checked_sqrt(self) -> Option<Rational> {
        let sign = self >= 0;
        let (n, d) = self.into_numerator_and_denominator();
        let sqrt_n;
        let sqrt_d;
        if n.significant_bits() <= d.significant_bits() {
            sqrt_n = Integer::from_sign_and_abs(sign, n).checked_sqrt()?;
            sqrt_d = d.checked_sqrt()?;
        } else {
            sqrt_d = d.checked_sqrt()?;
            sqrt_n = Integer::from_sign_and_abs(sign, n).checked_sqrt()?;
        }
        Some(Rational {
            sign: sqrt_n >= 0,
            numerator: sqrt_n.unsigned_abs(),
            denominator: sqrt_d,
        })
    }
}

impl<'a> CheckedSqrt for &'a Rational {
    type Output = Rational;

    /// Returns the the square root of a `Rational`, or `None` if the `Integer` is not the square
    /// of a `Rational`. The `Rational` is taken by reference.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Q \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::from(99u8)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Rational::from(100u8)).checked_sqrt().to_debug_string(), "Some(10)");
    /// assert_eq!((&Rational::from(101u8)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Rational::from_signeds(22, 7)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Rational::from_signeds(25, 9)).checked_sqrt().to_debug_string(), "Some(5/3)");
    /// ```
    fn checked_sqrt(self) -> Option<Rational> {
        let (n, d) = self.numerator_and_denominator_ref();
        let sqrt_n;
        let sqrt_d;
        if n.significant_bits() <= d.significant_bits() {
            sqrt_n = Integer::from_sign_and_abs_ref(*self >= 0, n).checked_sqrt()?;
            sqrt_d = d.checked_sqrt()?;
        } else {
            sqrt_d = d.checked_sqrt()?;
            sqrt_n = Integer::from_sign_and_abs_ref(*self >= 0, n).checked_sqrt()?;
        }
        Some(Rational {
            sign: sqrt_n >= 0,
            numerator: sqrt_n.unsigned_abs(),
            denominator: sqrt_d,
        })
    }
}
