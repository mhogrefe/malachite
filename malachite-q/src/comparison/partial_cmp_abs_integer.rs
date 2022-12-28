use crate::Rational;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

impl PartialOrdAbs<Integer> for Rational {
    /// Compares the absolute values of a [`Rational`] and an
    /// [`Integer`](malachite_nz::integer::Integer).
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
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).partial_cmp_abs(&Integer::from(3)),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).partial_cmp_abs(&Integer::from(-3)),
    ///     Some(Ordering::Greater)
    /// );
    /// ```
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        // First check whether either value is zero
        let self_sign = self.numerator_ref().sign();
        let other_sign = other.unsigned_abs_ref().sign();
        let sign_cmp = self_sign.cmp(&other_sign);
        if sign_cmp != Ordering::Equal || self_sign == Ordering::Equal {
            return Some(sign_cmp);
        }
        // Then check if one is < 1 and the other is > 1
        let self_cmp_one = self.numerator.cmp(&self.denominator);
        let other_cmp_one = other.unsigned_abs_ref().cmp(&Natural::ONE);
        let one_cmp = self_cmp_one.cmp(&other_cmp_one);
        if one_cmp != Ordering::Equal {
            return Some(one_cmp);
        }
        // Then compare numerators and denominators
        let n_cmp = self.numerator.cmp(other.unsigned_abs_ref());
        let d_cmp = self.denominator.cmp(&Natural::ONE);
        if n_cmp == Ordering::Equal && d_cmp == Ordering::Equal {
            return Some(Ordering::Equal);
        } else {
            let nd_cmp = n_cmp.cmp(&d_cmp);
            if nd_cmp != Ordering::Equal {
                return Some(nd_cmp);
            }
        }
        // Then compare floor ∘ log_2 ∘ abs
        let log_cmp = self
            .floor_log_base_2_of_abs()
            .cmp(&i64::exact_from(other.significant_bits() - 1));
        if log_cmp != Ordering::Equal {
            return Some(log_cmp);
        }
        // Finally, cross-multiply.
        Some(
            self.numerator
                .cmp(&(&self.denominator * other.unsigned_abs_ref())),
        )
    }
}

impl PartialOrdAbs<Rational> for Integer {
    /// Compares the absolute values of an [`Integer`](malachite_nz::integer::Integer) and a
    /// [`Rational`].
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
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(
    ///     Integer::from(3).partial_cmp_abs(&Rational::from_signeds(22, 7)),
    ///     Some(Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-3).partial_cmp_abs(&Rational::from_signeds(-22, 7)),
    ///     Some(Ordering::Less)
    /// );
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
