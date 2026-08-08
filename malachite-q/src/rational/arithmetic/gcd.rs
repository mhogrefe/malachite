// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2020 Daniel Schultz
//
//      Copyright © 2023 Albin Ahlbäck
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign, Lcm, LcmAssign};

// In each impl below, the GCD of two canonical rationals p/q and r/s is gcd(p, r)/lcm(q, s): FLINT
// defines the GCD as the canonical form of gcd(ps, qr)/(qs), which reduces to the component form
// for canonical inputs, as the assertions of _fmpq_gcd_cofactors check. The result is canonical
// outright: a common prime of gcd(p, r) and lcm(q, s) would divide both numerators and one
// denominator, contradicting that input's canonicity.

impl Gcd<Self> for Rational {
    type Output = Self;

    /// Computes the GCD of two [`Rational`]s, taking both by value.
    ///
    /// The GCD of $p/q$ and $r/s$ in canonical form is $\gcd(p, r)/\operatorname{lcm}(q, s)$: the
    /// largest rational that divides both into integers. It is stable under scaling of numerator
    /// and denominator, and agrees with the GCD on the integers.
    ///
    /// $$
    /// f(p/q, r/s) = \frac{\gcd(p, r)}{\operatorname{lcm}(q, s)}.
    /// $$
    ///
    /// The GCD of 0 and 0 is 0, and the GCD of $x$ and 0 is $|x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let x = Rational::from_str("2/3").unwrap();
    /// let y = Rational::from_str("-3/4").unwrap();
    /// assert_eq!(x.gcd(y).to_string(), "1/12");
    /// ```
    ///
    /// This is fmpq_gcd from fmpq/gcd.c, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: Self) -> Self {
        Self {
            sign: true,
            numerator: self.numerator.gcd(other.numerator),
            denominator: self.denominator.lcm(other.denominator),
        }
    }
}

impl Gcd<&Self> for Rational {
    type Output = Self;

    /// Computes the GCD of two [`Rational`]s, taking the first by value and the second by
    /// reference.
    ///
    /// The GCD of $p/q$ and $r/s$ in canonical form is $\gcd(p, r)/\operatorname{lcm}(q, s)$: the
    /// largest rational that divides both into integers. It is stable under scaling of numerator
    /// and denominator, and agrees with the GCD on the integers.
    ///
    /// $$
    /// f(p/q, r/s) = \frac{\gcd(p, r)}{\operatorname{lcm}(q, s)}.
    /// $$
    ///
    /// The GCD of 0 and 0 is 0, and the GCD of $x$ and 0 is $|x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let x = Rational::from_str("2/3").unwrap();
    /// let y = Rational::from_str("-3/4").unwrap();
    /// assert_eq!(x.gcd(&y).to_string(), "1/12");
    /// ```
    ///
    /// This is fmpq_gcd from fmpq/gcd.c, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: &Self) -> Self {
        Self {
            sign: true,
            numerator: self.numerator.gcd(&other.numerator),
            denominator: self.denominator.lcm(&other.denominator),
        }
    }
}

impl Gcd<Rational> for &Rational {
    type Output = Rational;

    /// Computes the GCD of two [`Rational`]s, taking the first by reference and the second by
    /// value.
    ///
    /// The GCD of $p/q$ and $r/s$ in canonical form is $\gcd(p, r)/\operatorname{lcm}(q, s)$: the
    /// largest rational that divides both into integers. It is stable under scaling of numerator
    /// and denominator, and agrees with the GCD on the integers.
    ///
    /// $$
    /// f(p/q, r/s) = \frac{\gcd(p, r)}{\operatorname{lcm}(q, s)}.
    /// $$
    ///
    /// The GCD of 0 and 0 is 0, and the GCD of $x$ and 0 is $|x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let x = Rational::from_str("2/3").unwrap();
    /// let y = Rational::from_str("-3/4").unwrap();
    /// assert_eq!((&x).gcd(y).to_string(), "1/12");
    /// ```
    ///
    /// This is fmpq_gcd from fmpq/gcd.c, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: Rational) -> Rational {
        Rational {
            sign: true,
            numerator: other.numerator.gcd(&self.numerator),
            denominator: other.denominator.lcm(&self.denominator),
        }
    }
}

impl Gcd<&Rational> for &Rational {
    type Output = Rational;

    /// Computes the GCD of two [`Rational`]s, taking both by reference.
    ///
    /// The GCD of $p/q$ and $r/s$ in canonical form is $\gcd(p, r)/\operatorname{lcm}(q, s)$: the
    /// largest rational that divides both into integers. It is stable under scaling of numerator
    /// and denominator, and agrees with the GCD on the integers.
    ///
    /// $$
    /// f(p/q, r/s) = \frac{\gcd(p, r)}{\operatorname{lcm}(q, s)}.
    /// $$
    ///
    /// The GCD of 0 and 0 is 0, and the GCD of $x$ and 0 is $|x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let x = Rational::from_str("2/3").unwrap();
    /// let y = Rational::from_str("-3/4").unwrap();
    /// assert_eq!((&x).gcd(&y).to_string(), "1/12");
    /// ```
    ///
    /// This is fmpq_gcd from fmpq/gcd.c, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: &Rational) -> Rational {
        Rational {
            sign: true,
            numerator: (&self.numerator).gcd(&other.numerator),
            denominator: (&self.denominator).lcm(&other.denominator),
        }
    }
}

impl GcdAssign<Self> for Rational {
    /// Replaces a [`Rational`] with the GCD of it and another [`Rational`], taking the other by
    /// value.
    ///
    /// See [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) for the definition.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut x = Rational::from_str("2/3").unwrap();
    /// x.gcd_assign(Rational::from_str("-3/4").unwrap());
    /// assert_eq!(x.to_string(), "1/12");
    /// ```
    ///
    /// This is fmpq_gcd from fmpq/gcd.c, FLINT 3.6.0, where res and op1 are aliased.
    #[inline]
    fn gcd_assign(&mut self, other: Self) {
        self.sign = true;
        self.numerator.gcd_assign(other.numerator);
        self.denominator.lcm_assign(other.denominator);
    }
}

impl GcdAssign<&Self> for Rational {
    /// Replaces a [`Rational`] with the GCD of it and another [`Rational`], taking the other by
    /// reference.
    ///
    /// See [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) for the definition.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut x = Rational::from_str("2/3").unwrap();
    /// x.gcd_assign(&Rational::from_str("-3/4").unwrap());
    /// assert_eq!(x.to_string(), "1/12");
    /// ```
    ///
    /// This is fmpq_gcd from fmpq/gcd.c, FLINT 3.6.0, where res and op1 are aliased.
    #[inline]
    fn gcd_assign(&mut self, other: &Self) {
        self.sign = true;
        self.numerator.gcd_assign(&other.numerator);
        self.denominator.lcm_assign(&other.denominator);
    }
}
