use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_base::num::logic::traits::SignificantBits;
use std::cmp::Ordering;
use Rational;

impl PartialOrdAbs for Rational {
    /// Compares the absolute value of an `Rational` to the absolute value of another `Rational`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::comparison::traits::OrdAbs;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3").unwrap().cmp_abs(&Rational::from_str("1/2").unwrap()),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(
    ///     Rational::from_str("-2/3").unwrap().cmp_abs(&Rational::from_str("1/2").unwrap()),
    ///     Ordering::Greater
    /// );
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

/// Asserts that `Rational` absolute value ordering is a total order.
impl OrdAbs for Rational {
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
        // Before cross-multiplying, compute approx significant bits of cross-products.
        //
        // Let a and b be the significant bits of x and y. Knowing a, we can say that x is between
        // 2 ^ (a - 1) and 2 ^ a - 1, inclusive, and analogously for y.
        //
        // Thus, xy is between 2 ^ (a + b - 2) and 2 ^ (a + b) - 2 ^ a - 2 ^ b + 1, inclusive.
        //
        // So, the number of significant bits of xy is a + b - 1 or a + b.
        //
        // If we can determine that one product has more significant bits than the other, we can
        // avoid the multiplication.
        let first_prod_bits =
            self.numerator.significant_bits() + other.denominator.significant_bits();
        let second_prod_bits =
            self.denominator.significant_bits() + other.numerator.significant_bits();
        let bit_cmp = if first_prod_bits < second_prod_bits - 1 {
            Some(Ordering::Less)
        } else if first_prod_bits - 1 > second_prod_bits {
            Some(Ordering::Greater)
        } else {
            None
        };
        if let Some(bit_cmp) = bit_cmp {
            return bit_cmp;
        }
        // Finally, cross-multiply.
        (&self.numerator * &other.denominator).cmp(&(&self.denominator * &other.numerator))
    }
}
