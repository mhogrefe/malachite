use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use std::cmp::Ordering;
use crate::Rational;

impl PartialOrdAbs for Rational {
    /// Compares the absolute values of two [`Rational`]s.
    ///
    /// See the documentation for the
    /// [`OrdAbs`](malachite_base::num::comparison::traits::OrdAbs) implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

impl OrdAbs for Rational {
    /// Compares the absolute values of two [`Rational`]s.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::num::comparison::traits::OrdAbs;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3").unwrap().cmp_abs(&Rational::ONE_HALF),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(
    ///     Rational::from_str("-2/3").unwrap().cmp_abs(&Rational::ONE_HALF),
    ///     Ordering::Greater
    /// );
    /// ```
    fn cmp_abs(&self, other: &Rational) -> Ordering {
        if std::ptr::eq(self, other) {
            return Ordering::Equal;
        }
        // First check if either value is zero
        let self_sign = self.numerator_ref().sign();
        let other_sign = other.numerator_ref().sign();
        let sign_cmp = self_sign.cmp(&other_sign);
        if sign_cmp != Ordering::Equal || self_sign == Ordering::Equal {
            return sign_cmp;
        }
        // Then check if one is < 1 and the other is > 1
        let self_cmp_one = self.numerator.cmp(&self.denominator);
        let other_cmp_one = other.numerator.cmp(&other.denominator);
        let one_cmp = self_cmp_one.cmp(&other_cmp_one);
        if one_cmp != Ordering::Equal {
            return one_cmp;
        }
        // Then compare numerators and denominators
        let n_cmp = self.numerator.cmp(&other.numerator);
        let d_cmp = self.denominator.cmp(&other.denominator);
        if n_cmp == Ordering::Equal && d_cmp == Ordering::Equal {
            return Ordering::Equal;
        } else {
            let nd_cmp = n_cmp.cmp(&d_cmp);
            if nd_cmp != Ordering::Equal {
                return nd_cmp;
            }
        }
        // Then compare floor ∘ log_2 ∘ abs
        let log_cmp = self
            .floor_log_base_2_of_abs()
            .cmp(&other.floor_log_base_2_of_abs());
        if log_cmp != Ordering::Equal {
            return log_cmp;
        }
        // Finally, cross-multiply.
        (&self.numerator * &other.denominator).cmp(&(&self.denominator * &other.numerator))
    }
}
