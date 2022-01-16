use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;
use Rational;

impl PartialOrdAbs<Integer> for Rational {
    /// Compares the absolute value of a `Rational` to the absolute value of an `Integer`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("22/7").unwrap().partial_cmp_abs(&Integer::from(3)),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Rational::from_str("-22/7").unwrap().partial_cmp_abs(&Integer::from(-3)),
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
        let first_prod_bits = self.numerator.significant_bits();
        let second_prod_bits = self.denominator.significant_bits() + other.significant_bits();
        let bit_cmp = if first_prod_bits < second_prod_bits - 1 {
            Some(Ordering::Less)
        } else if first_prod_bits > second_prod_bits {
            Some(Ordering::Greater)
        } else {
            None
        };
        if bit_cmp.is_some() {
            return bit_cmp;
        }
        // Finally, cross-multiply.
        Some(
            self.numerator
                .cmp(&(&self.denominator * other.unsigned_abs_ref())),
        )
    }
}

impl PartialOrdAbs<Rational> for Integer {
    /// Compares the absolute value of an `Integer` to the absolute value of a `Rational`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Integer::from(3).partial_cmp_abs(&Rational::from_str("22/7").unwrap()),
    ///     Some(Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-3).partial_cmp_abs(&Rational::from_str("-22/7").unwrap()),
    ///     Some(Ordering::Less)
    /// );
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
