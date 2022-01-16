use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::logic::traits::SignificantBits;
use std::cmp::Ordering;
use Rational;

impl PartialOrd for Rational {
    /// Compares a `Rational` to another `Rational`.
    ///
    /// See the documentation for the `Ord` implementation.
    #[inline]
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    /// Compares a `Rational` to another `Rational`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Rational::from_str("2/3").unwrap() > Rational::from_str("1/2").unwrap());
    /// assert!(Rational::from_str("-2/3").unwrap() < Rational::from_str("1/2").unwrap());
    /// ```
    fn cmp(&self, other: &Rational) -> Ordering {
        if std::ptr::eq(self, other) {
            return Ordering::Equal;
        }
        // First check signs
        let self_sign = self.sign();
        let other_sign = other.sign();
        let sign_cmp = self_sign.cmp(&other_sign);
        if sign_cmp != Ordering::Equal || self_sign == Ordering::Equal {
            return sign_cmp;
        }
        // Then check if one is < 1 and the other is > 1
        let self_cmp_one = self.numerator.cmp(&self.denominator);
        let other_cmp_one = other.numerator.cmp(&other.denominator);
        let one_cmp = self_cmp_one.cmp(&other_cmp_one);
        if one_cmp != Ordering::Equal {
            return if self.sign {
                one_cmp
            } else {
                one_cmp.reverse()
            };
        }
        // Then compare numerators and denominators
        let n_cmp = self.numerator.cmp(&other.numerator);
        let d_cmp = self.denominator.cmp(&other.denominator);
        if n_cmp == Ordering::Equal && d_cmp == Ordering::Equal {
            return Ordering::Equal;
        } else {
            let nd_cmp = n_cmp.cmp(&d_cmp);
            if nd_cmp != Ordering::Equal {
                return if self.sign { nd_cmp } else { nd_cmp.reverse() };
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
            return if self.sign {
                bit_cmp
            } else {
                bit_cmp.reverse()
            };
        }
        // Finally, cross-multiply.
        let prod_cmp =
            (&self.numerator * &other.denominator).cmp(&(&self.denominator * &other.numerator));
        if self.sign {
            prod_cmp
        } else {
            prod_cmp.reverse()
        }
    }
}
