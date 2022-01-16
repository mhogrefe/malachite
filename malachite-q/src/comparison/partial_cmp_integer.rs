use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;
use Rational;

impl PartialOrd<Integer> for Rational {
    /// Compares a `Rational` to an `Integer`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Rational::from_str("22/7").unwrap() > Integer::from(3));
    /// assert!(Rational::from_str("22/7").unwrap() < Integer::from(4));
    /// assert!(Rational::from_str("-22/7").unwrap() < Integer::from(-3));
    /// assert!(Rational::from_str("-22/7").unwrap() > Integer::from(-4));
    /// ```
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        // First check signs
        let self_sign = self.sign();
        let other_sign = other.sign();
        let sign_cmp = self_sign.cmp(&other_sign);
        if sign_cmp != Ordering::Equal || self_sign == Ordering::Equal {
            return Some(sign_cmp);
        }
        // Then check if one is < 1 and the other is > 1
        let self_cmp_one = self.numerator.cmp(&self.denominator);
        let other_cmp_one = other.unsigned_abs_ref().cmp(&Natural::ONE);
        let one_cmp = self_cmp_one.cmp(&other_cmp_one);
        if one_cmp != Ordering::Equal {
            return Some(if self.sign {
                one_cmp
            } else {
                one_cmp.reverse()
            });
        }
        // Then compare numerators and denominators
        let n_cmp = self.numerator.cmp(other.unsigned_abs_ref());
        let d_cmp = self.denominator.cmp(&Natural::ONE);
        if n_cmp == Ordering::Equal && d_cmp == Ordering::Equal {
            return Some(Ordering::Equal);
        } else {
            let nd_cmp = n_cmp.cmp(&d_cmp);
            if nd_cmp != Ordering::Equal {
                return Some(if self.sign { nd_cmp } else { nd_cmp.reverse() });
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
        if let Some(bit_cmp) = bit_cmp {
            return Some(if self.sign {
                bit_cmp
            } else {
                bit_cmp.reverse()
            });
        }
        // Finally, cross-multiply.
        let prod_cmp = self
            .numerator
            .cmp(&(&self.denominator * other.unsigned_abs_ref()));
        Some(if self.sign {
            prod_cmp
        } else {
            prod_cmp.reverse()
        })
    }
}

impl PartialOrd<Rational> for Integer {
    /// Compares an `Integer` to a `Rational`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Integer::from(3) < Rational::from_str("22/7").unwrap());
    /// assert!(Integer::from(4) > Rational::from_str("22/7").unwrap());
    /// assert!(Integer::from(-3) > Rational::from_str("-22/7").unwrap());
    /// assert!(Integer::from(-4) < Rational::from_str("-22/7").unwrap());
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
