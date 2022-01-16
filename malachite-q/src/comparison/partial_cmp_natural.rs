use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;
use Rational;

impl PartialOrd<Natural> for Rational {
    /// Compares a `Rational` to a `Natural`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Rational::from_str("22/7").unwrap() > Natural::from(3u32));
    /// assert!(Rational::from_str("22/7").unwrap() < Natural::from(4u32));
    /// ```
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        // First check signs
        let self_sign = self.sign();
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

impl PartialOrd<Rational> for Natural {
    /// Compares a `Natural` to a `Rational`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Natural::from(3u32) < Rational::from_str("22/7").unwrap());
    /// assert!(Natural::from(4u32) > Rational::from_str("22/7").unwrap());
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
