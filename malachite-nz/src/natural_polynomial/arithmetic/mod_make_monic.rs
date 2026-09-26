// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{
    Gcd, ModInverse, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign,
};
use malachite_base::polynomial::{ModMakeMonic, ModMakeMonicAssign};

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Multiplies every coefficient by the inverse of the leading coefficient, which makes the leading
// coefficient 1, or returns the GCD of the leading coefficient and m when there is no inverse.
fn mod_make_monic_in_place(coefficients: &mut [Natural], m: &Natural) -> Result<(), Natural> {
    let Some(leading) = coefficients.last() else {
        return Ok(());
    };
    let Some(inverse) = leading.mod_inverse(m) else {
        return Err(leading.gcd(m));
    };
    if inverse != 1u32 {
        let data: ModMulData =
            <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
        for c in coefficients {
            c.mod_mul_precomputed_assign(&inverse, m, &data);
        }
    }
    Ok(())
}

fn mod_make_monic_ref(p: &NaturalPolynomial, m: &Natural) -> Result<NaturalPolynomial, Natural> {
    assert_reduced(p, m);
    let mut coefficients = p.coefficients.clone();
    mod_make_monic_in_place(&mut coefficients, m)?;
    Ok(NaturalPolynomial { coefficients })
}

fn mod_make_monic_val(mut p: NaturalPolynomial, m: &Natural) -> Result<NaturalPolynomial, Natural> {
    assert_reduced(&p, m);
    mod_make_monic_in_place(&mut p.coefficients, m)?;
    Ok(p)
}

fn mod_make_monic_assign(p: &mut NaturalPolynomial, m: &Natural) -> Result<(), Natural> {
    assert_reduced(p, m);
    mod_make_monic_in_place(&mut p.coefficients, m)
}

impl ModMakeMonic<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Factor = Natural;

    /// Makes a [`NaturalPolynomial`] monic modulo a [`Natural`], by multiplying it by the inverse
    /// of its leading coefficient, taking the polynomial by reference and the modulus by reference.
    /// The coefficients must already be reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the error is its GCD with `m`, a
    /// nontrivial factor of `m`. The zero polynomial is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n b \log b \log\log b)$
    ///
    /// $M(n, b) = O(nb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::polynomial::ModMakeMonic;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 3 * 5 = 15, which is 1 mod 7.
    /// let p = NaturalPolynomial::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_make_monic(&Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+5*x+3"
    /// );
    /// // 2 has no inverse mod 4, and shares the factor 2 with it.
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert_eq!((&p).mod_make_monic(&Natural::from(4u32)), Err(Natural::TWO));
    /// assert_eq!(
    ///     (&NaturalPolynomial::ZERO).mod_make_monic(&Natural::from(7u32)),
    ///     Ok(NaturalPolynomial::ZERO)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_make_monic_f` from `fmpz_mod_poly/make_monic.c`, FLINT
    /// 3.6.0, with the factor returned as the error.
    #[inline]
    fn mod_make_monic(self, m: &Natural) -> Result<NaturalPolynomial, Natural> {
        mod_make_monic_ref(self, m)
    }
}

impl ModMakeMonic<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Factor = Natural;

    /// Makes a [`NaturalPolynomial`] monic modulo a [`Natural`], by multiplying it by the inverse
    /// of its leading coefficient, taking the polynomial by reference and the modulus by value. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the error is its GCD with `m`, a
    /// nontrivial factor of `m`. The zero polynomial is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n b \log b \log\log b)$
    ///
    /// $M(n, b) = O(nb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::polynomial::ModMakeMonic;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 3 * 5 = 15, which is 1 mod 7.
    /// let p = NaturalPolynomial::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_make_monic(Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+5*x+3"
    /// );
    /// // 2 has no inverse mod 4, and shares the factor 2 with it.
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert_eq!((&p).mod_make_monic(Natural::from(4u32)), Err(Natural::TWO));
    /// assert_eq!(
    ///     (&NaturalPolynomial::ZERO).mod_make_monic(Natural::from(7u32)),
    ///     Ok(NaturalPolynomial::ZERO)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_make_monic_f` from `fmpz_mod_poly/make_monic.c`, FLINT
    /// 3.6.0, with the factor returned as the error.
    #[inline]
    fn mod_make_monic(self, m: Natural) -> Result<NaturalPolynomial, Natural> {
        mod_make_monic_ref(self, &m)
    }
}

impl ModMakeMonic<&Natural> for NaturalPolynomial {
    type Output = Self;
    type Factor = Natural;

    /// Makes a [`NaturalPolynomial`] monic modulo a [`Natural`], by multiplying it by the inverse
    /// of its leading coefficient, taking the polynomial by value and the modulus by reference. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the error is its GCD with `m`, a
    /// nontrivial factor of `m`. The zero polynomial is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n b \log b \log\log b)$
    ///
    /// $M(n, b) = O(nb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::polynomial::ModMakeMonic;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 3 * 5 = 15, which is 1 mod 7.
    /// let p = NaturalPolynomial::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_make_monic(&Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+5*x+3"
    /// );
    /// // 2 has no inverse mod 4, and shares the factor 2 with it.
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_make_monic(&Natural::from(4u32)),
    ///     Err(Natural::TWO)
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.mod_make_monic(&Natural::from(7u32)),
    ///     Ok(NaturalPolynomial::ZERO)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_make_monic_f` from `fmpz_mod_poly/make_monic.c`, FLINT
    /// 3.6.0, with the factor returned as the error.
    #[inline]
    fn mod_make_monic(self, m: &Natural) -> Result<Self, Natural> {
        mod_make_monic_val(self, m)
    }
}

impl ModMakeMonic<Natural> for NaturalPolynomial {
    type Output = Self;
    type Factor = Natural;

    /// Makes a [`NaturalPolynomial`] monic modulo a [`Natural`], by multiplying it by the inverse
    /// of its leading coefficient, taking the polynomial by value and the modulus by value. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the error is its GCD with `m`, a
    /// nontrivial factor of `m`. The zero polynomial is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n b \log b \log\log b)$
    ///
    /// $M(n, b) = O(nb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::polynomial::ModMakeMonic;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 3 * 5 = 15, which is 1 mod 7.
    /// let p = NaturalPolynomial::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_make_monic(Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+5*x+3"
    /// );
    /// // 2 has no inverse mod 4, and shares the factor 2 with it.
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_make_monic(Natural::from(4u32)),
    ///     Err(Natural::TWO)
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.mod_make_monic(Natural::from(7u32)),
    ///     Ok(NaturalPolynomial::ZERO)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_make_monic_f` from `fmpz_mod_poly/make_monic.c`, FLINT
    /// 3.6.0, with the factor returned as the error.
    #[inline]
    fn mod_make_monic(self, m: Natural) -> Result<Self, Natural> {
        mod_make_monic_val(self, &m)
    }
}

impl ModMakeMonicAssign<&Natural> for NaturalPolynomial {
    type Factor = Natural;

    /// Makes a [`NaturalPolynomial`] monic modulo a [`Natural`] in place, by multiplying it by the
    /// inverse of its leading coefficient, taking the modulus by reference. The coefficients must
    /// already be reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the polynomial is left unchanged and
    /// the error is the leading coefficient's GCD with `m`, a nontrivial factor of `m`. The zero
    /// polynomial is left unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n b \log b \log\log b)$
    ///
    /// $M(b) = O(b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::polynomial::ModMakeMonicAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!(p.mod_make_monic_assign(&Natural::from(7u32)), Ok(()));
    /// assert_eq!(p.to_string(), "x^2+5*x+3");
    ///
    /// let mut p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_make_monic_assign(&Natural::from(4u32)),
    ///     Err(Natural::TWO)
    /// );
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    #[inline]
    fn mod_make_monic_assign(&mut self, m: &Natural) -> Result<(), Natural> {
        mod_make_monic_assign(self, m)
    }
}

impl ModMakeMonicAssign<Natural> for NaturalPolynomial {
    type Factor = Natural;

    /// Makes a [`NaturalPolynomial`] monic modulo a [`Natural`] in place, by multiplying it by the
    /// inverse of its leading coefficient, taking the modulus by value. The coefficients must
    /// already be reduced modulo `m`.
    ///
    /// If the leading coefficient has no inverse modulo `m`, the polynomial is left unchanged and
    /// the error is the leading coefficient's GCD with `m`, a nontrivial factor of `m`. The zero
    /// polynomial is left unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n b \log b \log\log b)$
    ///
    /// $M(b) = O(b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::polynomial::ModMakeMonicAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("3*x^2+x+2").unwrap();
    /// assert_eq!(p.mod_make_monic_assign(Natural::from(7u32)), Ok(()));
    /// assert_eq!(p.to_string(), "x^2+5*x+3");
    ///
    /// let mut p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_make_monic_assign(Natural::from(4u32)),
    ///     Err(Natural::TWO)
    /// );
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    #[inline]
    fn mod_make_monic_assign(&mut self, m: Natural) -> Result<(), Natural> {
        mod_make_monic_assign(self, &m)
    }
}
