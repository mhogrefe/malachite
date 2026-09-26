// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{MakeMonicMod, MakeMonicModAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;

// Multiplies every coefficient by the inverse of the leading coefficient, which makes the leading
// coefficient 1, or returns the GCD of the leading coefficient and m when there is no inverse.
fn make_monic_mod_in_place<T: PrimitiveUnsigned>(coefficients: &mut [T], m: T) -> Result<(), T> {
    let Some(&leading) = coefficients.last() else {
        return Ok(());
    };
    let Some(inverse) = leading.mod_inverse(m) else {
        return Err(leading.gcd(m));
    };
    if inverse != T::ONE {
        let data = T::precompute_mod_mul_data(&m);
        for c in coefficients {
            c.mod_mul_precomputed_assign(inverse, m, &data);
        }
    }
    Ok(())
}

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

impl<T: PrimitiveUnsigned> MakeMonicMod<T> for UnsignedPolynomial<T> {
    type Output = Self;
    type Factor = T;

    /// Makes an [`UnsignedPolynomial`] monic modulo `m`, by multiplying it by the inverse of its
    /// leading coefficient, taking the polynomial by value. The coefficients must already be
    /// reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the error is its GCD with `m`, a
    /// nontrivial factor of `m`. The zero polynomial is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::MakeMonicMod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // 3 * 5 = 15, which is 1 mod 7.
    /// let p = UnsignedPolynomial::<u8>::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!(
    ///     p.clone().make_monic_mod(7).unwrap().to_string(),
    ///     "x^2+5*x+3"
    /// );
    /// // 2 has no inverse mod 4, and shares the factor 2 with it.
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert_eq!(p.clone().make_monic_mod(4), Err(2));
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::ZERO.make_monic_mod(7),
    ///     Ok(UnsignedPolynomial::ZERO)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_make_monic_f` from `fmpz_mod_poly/make_monic.c`, FLINT
    /// 3.6.0, with the factor returned as the error; `nmod_poly_make_monic` aborts instead.
    fn make_monic_mod(mut self, m: T) -> Result<Self, T> {
        assert_reduced(&self, m);
        make_monic_mod_in_place(&mut self.coefficients, m)?;
        Ok(self)
    }
}

impl<T: PrimitiveUnsigned> MakeMonicMod<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;
    type Factor = T;

    /// Makes an [`UnsignedPolynomial`] monic modulo `m`, by multiplying it by the inverse of its
    /// leading coefficient, taking the polynomial by reference. The coefficients must already be
    /// reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the error is its GCD with `m`, a
    /// nontrivial factor of `m`. The zero polynomial is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::MakeMonicMod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // 3 * 5 = 15, which is 1 mod 7.
    /// let p = UnsignedPolynomial::<u8>::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!((&p).make_monic_mod(7).unwrap().to_string(), "x^2+5*x+3");
    /// // 2 has no inverse mod 4, and shares the factor 2 with it.
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert_eq!((&p).make_monic_mod(4), Err(2));
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::ZERO).make_monic_mod(7),
    ///     Ok(UnsignedPolynomial::ZERO)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_make_monic_f` from `fmpz_mod_poly/make_monic.c`, FLINT
    /// 3.6.0, with the factor returned as the error; `nmod_poly_make_monic` aborts instead.
    fn make_monic_mod(self, m: T) -> Result<UnsignedPolynomial<T>, T> {
        assert_reduced(self, m);
        let mut coefficients = self.coefficients.clone();
        make_monic_mod_in_place(&mut coefficients, m)?;
        Ok(UnsignedPolynomial { coefficients })
    }
}

impl<T: PrimitiveUnsigned> MakeMonicModAssign<T> for UnsignedPolynomial<T> {
    type Factor = T;

    /// Makes an [`UnsignedPolynomial`] monic modulo `m` in place, by multiplying it by the inverse
    /// of its leading coefficient. The coefficients must already be reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the polynomial is left unchanged and
    /// the error is the leading coefficient's GCD with `m`, a nontrivial factor of `m`. The zero
    /// polynomial is left unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MakeMonicModAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!(p.make_monic_mod_assign(7), Ok(()));
    /// assert_eq!(p.to_string(), "x^2+5*x+3");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert_eq!(p.make_monic_mod_assign(4), Err(2));
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    fn make_monic_mod_assign(&mut self, m: T) -> Result<(), T> {
        assert_reduced(self, m);
        make_monic_mod_in_place(&mut self.coefficients, m)
    }
}
