use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;
use Rational;

impl PartialOrdAbs<Natural> for Rational {
    /// Compares the absolute value of a `Rational` to the absolute value of a `Natural`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("22/7").unwrap().partial_cmp_abs(&Natural::from(3u32)),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Rational::from_str("-22/7").unwrap().partial_cmp_abs(&Natural::from(3u32)),
    ///     Some(Ordering::Greater)
    /// );
    /// ```
    fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
        // First check if either value is zero
        let self_sign = self.numerator_ref().sign();
        let other_sign = other.sign();
        let sign_cmp = self_sign.cmp(&other_sign);
        if sign_cmp != Ordering::Equal || self_sign == Ordering::Equal {
            return Some(sign_cmp);
        }
        // Then check if one is < 1 and the other is > 1
        let self_cmp_one = self.numerator.cmp(&self.denominator);
        let other_cmp_one = other.cmp(&Natural::ONE);
        let one_cmp = self_cmp_one.cmp(&other_cmp_one);
        if one_cmp != Ordering::Equal {
            return Some(one_cmp);
        }
        // Then compare numerators and denominators
        let n_cmp = self.numerator.cmp(other);
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
        if first_prod_bits < second_prod_bits - 1 {
            return Some(Ordering::Less);
        } else if first_prod_bits > second_prod_bits {
            return Some(Ordering::Greater);
        }
        // Finally, cross-multiply.
        Some(self.numerator.cmp(&(&self.denominator * other)))
    }
}

impl PartialOrdAbs<Rational> for Natural {
    /// Compares the absolute value of a `Natural` to the absolute value of a `Rational`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).partial_cmp_abs(&Rational::from_str("22/7").unwrap()),
    ///     Some(Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(3u32).partial_cmp_abs(&Rational::from_str("-22/7").unwrap()),
    ///     Some(Ordering::Less)
    /// );
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
