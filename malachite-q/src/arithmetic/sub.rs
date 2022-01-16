use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, Gcd, GcdAssign, NegAssign, UnsignedAbs,
};
use malachite_nz::integer::Integer;
use std::ops::{Sub, SubAssign};
use Rational;

impl Sub<Rational> for Rational {
    type Output = Rational;

    /// Subtracts a `Rational` from a `Rational`, taking both `Rational`s by value.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF - Rational::ONE_HALF, 0);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) - Rational::from_signeds(99, 100)).to_string(),
    ///     "1507/700"
    /// );
    /// ```
    fn sub(self, other: Rational) -> Rational {
        if self == 0u32 {
            return -other;
        } else if other == 0u32 {
            return self;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let diff_n = Integer::from_sign_and_abs(self.sign, self.numerator * &other.denominator)
                - Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            let diff_d = self.denominator * other.denominator;
            Rational {
                sign: diff_n >= 0,
                numerator: diff_n.unsigned_abs(),
                denominator: diff_d,
            }
        } else {
            let reduced_self_d = (self.denominator).div_exact(&gcd);
            let diff_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    self.numerator * (&other.denominator).div_exact(&gcd),
                ) - Integer::from_sign_and_abs(other.sign, other.numerator * &reduced_self_d);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs(),
                    denominator: other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs().div_exact(&gcd),
                    denominator: (other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl<'a> Sub<&'a Rational> for Rational {
    type Output = Rational;

    /// Subtracts a `Rational` from a `Rational`, taking the left `Rational` by value and the right
    /// `Rational` by reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF - &Rational::ONE_HALF, 0);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) - &Rational::from_signeds(99, 100)).to_string(),
    ///     "1507/700"
    /// );
    /// ```
    #[inline]
    fn sub(self, other: &'a Rational) -> Rational {
        -(other - self)
    }
}

impl<'a> Sub<Rational> for &'a Rational {
    type Output = Rational;

    /// Subtracts a `Rational` from a `Rational`, taking the left `Rational` by reference and the
    /// right `Rational` by value.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::ONE_HALF - Rational::ONE_HALF, 0);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) - Rational::from_signeds(99, 100)).to_string(),
    ///     "1507/700"
    /// );
    /// ```
    fn sub(self, other: Rational) -> Rational {
        if *self == 0u32 {
            return -other;
        } else if other == 0u32 {
            return self.clone();
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let diff_n =
                Integer::from_sign_and_abs(self.sign, &self.numerator * &other.denominator)
                    - Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            let diff_d = &self.denominator * other.denominator;
            Rational {
                sign: diff_n >= 0,
                numerator: diff_n.unsigned_abs(),
                denominator: diff_d,
            }
        } else {
            let reduced_self_d = (&self.denominator).div_exact(&gcd);
            let diff_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    &self.numerator * (&other.denominator).div_exact(&gcd),
                ) - Integer::from_sign_and_abs(other.sign, other.numerator * &reduced_self_d);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs(),
                    denominator: other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs().div_exact(&gcd),
                    denominator: (other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl<'a, 'b> Sub<&'a Rational> for &'b Rational {
    type Output = Rational;

    /// Subtracts a `Rational` from a `Rational`, taking both `Rational`s by reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::ONE_HALF - &Rational::ONE_HALF, 0);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) - &Rational::from_signeds(99, 100)).to_string(),
    ///     "1507/700"
    /// );
    /// ```
    fn sub(self, other: &'a Rational) -> Rational {
        if *self == 0u32 {
            return -other.clone();
        } else if *other == 0u32 {
            return self.clone();
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let diff_n =
                Integer::from_sign_and_abs(self.sign, &self.numerator * &other.denominator)
                    - Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            let diff_d = &self.denominator * &other.denominator;
            Rational {
                sign: diff_n >= 0,
                numerator: diff_n.unsigned_abs(),
                denominator: diff_d,
            }
        } else {
            let reduced_self_d = (&self.denominator).div_exact(&gcd);
            let diff_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    &self.numerator * (&other.denominator).div_exact(&gcd),
                ) - Integer::from_sign_and_abs(other.sign, &other.numerator * &reduced_self_d);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs(),
                    denominator: &other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs().div_exact(&gcd),
                    denominator: (&other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl SubAssign<Rational> for Rational {
    /// Subtracts a `Rational` from a `Rational`, taking the `Rational` on the right-hand side by
    /// value.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x -= Rational::ONE_HALF;
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x -= Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "1507/700");
    /// ```
    fn sub_assign(&mut self, other: Rational) {
        if *self == 0u32 {
            *self = -other;
            return;
        } else if other == 0u32 {
            return;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            self.numerator *= &other.denominator;
            let diff_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                - Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            self.sign = diff_n >= 0;
            self.numerator = diff_n.unsigned_abs();
            self.denominator *= other.denominator;
        } else {
            self.denominator.div_exact_assign(&gcd);
            self.numerator *= (&other.denominator).div_exact(&gcd);
            let diff_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                - Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            self.sign = diff_n >= 0;
            if gcd == 1u32 {
                self.numerator = diff_n.unsigned_abs();
                self.denominator *= other.denominator;
            } else {
                self.numerator = diff_n.unsigned_abs().div_exact(&gcd);
                self.denominator *= (other.denominator).div_exact(gcd);
            }
        }
    }
}

impl<'a> SubAssign<&'a Rational> for Rational {
    /// Subtracts a `Rational` from a `Rational`, taking the `Rational` on the right-hand side by
    /// reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x -= &Rational::ONE_HALF;
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x -= &Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "1507/700");
    /// ```
    fn sub_assign(&mut self, other: &'a Rational) {
        if *self == 0u32 {
            self.clone_from(other);
            self.neg_assign();
            return;
        } else if *other == 0u32 {
            return;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            self.numerator *= &other.denominator;
            let diff_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                - Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            self.sign = diff_n >= 0;
            self.numerator = diff_n.unsigned_abs();
            self.denominator *= &other.denominator;
        } else {
            self.denominator.div_exact_assign(&gcd);
            self.numerator *= (&other.denominator).div_exact(&gcd);
            let diff_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                - Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            self.sign = diff_n >= 0;
            if gcd == 1u32 {
                self.numerator = diff_n.unsigned_abs();
                self.denominator *= &other.denominator;
            } else {
                self.numerator = diff_n.unsigned_abs().div_exact(&gcd);
                self.denominator *= (&other.denominator).div_exact(gcd);
            }
        }
    }
}
