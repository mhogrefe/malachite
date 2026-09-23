// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use core::ops::{Rem, RemAssign};
use malachite_base::num::arithmetic::traits::{Mod, ModAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

impl Rem<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], keeping the
    /// remainders, taking the polynomial by value and the modulus by value.
    ///
    /// $p \\% m$ is the polynomial whose $i$th coefficient is $p_i \\% m$, and this is the
    /// remainder of dividing $p$ by the constant polynomial $m$ — under the convention that
    /// applies over the integers, where a remainder is bounded coefficient by coefficient rather
    /// than by degree. There is a quotient to go with it: the polynomial whose $i$th coefficient is
    /// $\\lfloor p_i / m \\rfloor$, for which $p = mq + r$ holds exactly.
    ///
    /// The result is reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](malachite_base::num::arithmetic::traits::ModIsReduced::mod_is_reduced)
    /// returns `true` for it.
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $m$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $4x^2 + 3$ modulo $4$ is the constant $3$, not a quadratic with a zero
    /// leading coefficient.
    ///
    /// $$
    /// f(p, m) = q, \\quad \text{where} \\quad q_i = p_i - m \left \lfloor \frac{p_i}{m}
    /// \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from_str("1000000000000").unwrap();
    /// let p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// assert_eq!((p % m).to_string(), "x^2+3*x+5");
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
    /// assert_eq!((p % Natural::from(4u32)).to_string(), "3");
    /// ```
    #[inline]
    fn rem(mut self, m: Natural) -> Self {
        self %= m;
        self
    }
}

impl<'a> Rem<&'a Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], keeping the
    /// remainders, taking the polynomial by value and the modulus by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes
    /// both arguments by value for details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from_str("1000000000000").unwrap();
    /// let p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// assert_eq!((p % &m).to_string(), "x^2+3*x+5");
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
    /// assert_eq!((p % &Natural::from(4u32)).to_string(), "3");
    /// ```
    #[inline]
    fn rem(mut self, m: &'a Natural) -> Self {
        self %= m;
        self
    }
}

impl Rem<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], keeping the
    /// remainders, taking the polynomial by reference and the modulus by value.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes
    /// both arguments by value for details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from_str("1000000000000").unwrap();
    /// let p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// assert_eq!((&p % m).to_string(), "x^2+3*x+5");
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
    /// assert_eq!((&p % Natural::from(4u32)).to_string(), "3");
    /// // The polynomial is left alone.
    /// assert_eq!(p.to_string(), "4*x^2+3");
    /// ```
    #[inline]
    fn rem(self, m: Natural) -> NaturalPolynomial {
        self % &m
    }
}

impl<'a> Rem<&'a Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], keeping the
    /// remainders, taking the polynomial by reference and the modulus by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes
    /// both arguments by value for details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from_str("1000000000000").unwrap();
    /// let p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// assert_eq!((&p % &m).to_string(), "x^2+3*x+5");
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
    /// assert_eq!((&p % &Natural::from(4u32)).to_string(), "3");
    /// // The polynomial is left alone.
    /// assert_eq!(p.to_string(), "4*x^2+3");
    /// ```
    fn rem(self, m: &'a Natural) -> NaturalPolynomial {
        assert_ne!(*m, 0u32, "division by zero");
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        NaturalPolynomial::from_coefficients_asc(
            self.coefficients.iter().map(|c| c % m).collect::<Vec<_>>(),
        )
    }
}

impl RemAssign<Natural> for NaturalPolynomial {
    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], replacing the
    /// polynomial by the one whose coefficients are the remainders, taking the modulus by value.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes
    /// both arguments by value for details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// p %= Natural::from_str("1000000000000").unwrap();
    /// assert_eq!(p.to_string(), "x^2+3*x+5");
    ///
    /// let mut p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
    /// p %= Natural::from(4u32);
    /// assert_eq!(p.to_string(), "3");
    /// ```
    #[inline]
    fn rem_assign(&mut self, m: Natural) {
        *self %= &m;
    }
}

impl<'a> RemAssign<&'a Natural> for NaturalPolynomial {
    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], replacing the
    /// polynomial by the one whose coefficients are the remainders, taking the modulus by
    /// reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes
    /// both arguments by value for details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// p %= &Natural::from_str("1000000000000").unwrap();
    /// assert_eq!(p.to_string(), "x^2+3*x+5");
    ///
    /// let mut p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
    /// p %= &Natural::from(4u32);
    /// assert_eq!(p.to_string(), "3");
    /// ```
    fn rem_assign(&mut self, m: &'a Natural) {
        assert_ne!(*m, 0u32, "division by zero");
        for c in &mut self.coefficients {
            *c %= m;
        }
        self.trim();
    }
}

impl Mod<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], keeping the
    /// remainders, taking the polynomial by value and the modulus by value.
    ///
    /// A [`NaturalPolynomial`]'s coefficients are never negative, so this agrees with `%`
    /// everywhere; it is the same operation under the name the mod-family traits use. See the
    /// documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes both
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+4*x+5").unwrap();
    /// assert_eq!(p.mod_op(Natural::from(3u32)).to_string(), "x^2+x+2");
    /// ```
    #[inline]
    fn mod_op(self, m: Natural) -> Self {
        self % m
    }
}

impl<'a> Mod<&'a Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], keeping the
    /// remainders, taking the polynomial by value and the modulus by reference.
    ///
    /// A [`NaturalPolynomial`]'s coefficients are never negative, so this agrees with `%`
    /// everywhere; it is the same operation under the name the mod-family traits use. See the
    /// documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes both
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+4*x+5").unwrap();
    /// assert_eq!(p.mod_op(&Natural::from(3u32)).to_string(), "x^2+x+2");
    /// ```
    #[inline]
    fn mod_op(self, m: &'a Natural) -> Self {
        self % m
    }
}

impl Mod<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], keeping the
    /// remainders, taking the polynomial by reference and the modulus by value.
    ///
    /// A [`NaturalPolynomial`]'s coefficients are never negative, so this agrees with `%`
    /// everywhere; it is the same operation under the name the mod-family traits use. See the
    /// documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes both
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+4*x+5").unwrap();
    /// assert_eq!((&p).mod_op(Natural::from(3u32)).to_string(), "x^2+x+2");
    /// ```
    #[inline]
    fn mod_op(self, m: Natural) -> NaturalPolynomial {
        self % m
    }
}

impl<'a> Mod<&'a Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], keeping the
    /// remainders, taking the polynomial by reference and the modulus by reference.
    ///
    /// A [`NaturalPolynomial`]'s coefficients are never negative, so this agrees with `%`
    /// everywhere; it is the same operation under the name the mod-family traits use. See the
    /// documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes both
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+4*x+5").unwrap();
    /// assert_eq!((&p).mod_op(&Natural::from(3u32)).to_string(), "x^2+x+2");
    /// ```
    #[inline]
    fn mod_op(self, m: &'a Natural) -> NaturalPolynomial {
        self % m
    }
}

impl ModAssign<Natural> for NaturalPolynomial {
    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], replacing the
    /// polynomial by the one whose coefficients are the remainders, taking the modulus by value.
    ///
    /// A [`NaturalPolynomial`]'s coefficients are never negative, so this agrees with `%=`
    /// everywhere; it is the same operation under the name the mod-family traits use. See the
    /// documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes both
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("4*x^2+4*x+5").unwrap();
    /// p.mod_assign(Natural::from(3u32));
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    #[inline]
    fn mod_assign(&mut self, m: Natural) {
        *self %= m;
    }
}

impl<'a> ModAssign<&'a Natural> for NaturalPolynomial {
    /// Divides every coefficient of a [`NaturalPolynomial`] by a [`Natural`], replacing the
    /// polynomial by the one whose coefficients are the remainders, taking the modulus by
    /// reference.
    ///
    /// A [`NaturalPolynomial`]'s coefficients are never negative, so this agrees with `%=`
    /// everywhere; it is the same operation under the name the mod-family traits use. See the
    /// documentation for the [`Rem`] implementation on [`NaturalPolynomial`] that takes both
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("4*x^2+4*x+5").unwrap();
    /// p.mod_assign(&Natural::from(3u32));
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    #[inline]
    fn mod_assign(&mut self, m: &'a Natural) {
        *self %= m;
    }
}

impl<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>> Rem<T> for &NaturalPolynomial
where
    Natural: From<T>,
{
    type Output = UnsignedPolynomial<T>;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a value of an unsigned primitive
    /// integer type, keeping the remainders as an [`UnsignedPolynomial`] with that coefficient
    /// type, taking the polynomial by reference.
    ///
    /// Every remainder is less than `m`, so every one fits in `m`'s type, and this is the natural
    /// way to go from a polynomial with arbitrarily large coefficients to one reduced modulo a
    /// word-sized modulus. Apart from the result's type, it is the same operation as reducing
    /// modulo `Natural::from(m)`; see the documentation for the [`Rem`] implementation on
    /// [`NaturalPolynomial`] that takes both arguments by value for details, including how
    /// reducing can lower the degree.
    ///
    /// The result is reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](malachite_base::num::arithmetic::traits::ModIsReduced::mod_is_reduced)
    /// returns `true` for it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits in the
    /// polynomial's coefficients, and $m$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// let q = &p % 1000u32;
    /// assert_eq!(q.to_string(), "x^2+3*x+5");
    ///
    /// // The result's coefficient type is the modulus's.
    /// let q: malachite_base::unsigned_polynomial::UnsignedPolynomial<u8> = &p % 7u8;
    /// assert_eq!(q.to_string(), "2*x^2+5*x+5");
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = NaturalPolynomial::from_str("1024*x^2+3").unwrap();
    /// assert_eq!((&p % 4u64).to_string(), "3");
    /// ```
    fn rem(self, m: T) -> UnsignedPolynomial<T> {
        assert_ne!(m, T::ZERO, "division by zero");
        let m = Natural::from(m);
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        UnsignedPolynomial::from_coefficients_asc(
            self.coefficients
                .iter()
                .map(|c| T::exact_from(&(c % &m)))
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>> Rem<T> for NaturalPolynomial
where
    Natural: From<T>,
{
    type Output = UnsignedPolynomial<T>;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a value of an unsigned primitive
    /// integer type, keeping the remainders as an [`UnsignedPolynomial`] with that coefficient
    /// type, taking the polynomial by value.
    ///
    /// Taking the polynomial by value saves nothing, since the remainders go into new storage
    /// either way. See the documentation for the [`Rem`] implementation that takes the polynomial
    /// by reference for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits in the
    /// polynomial's coefficients, and $m$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// assert_eq!((p % 1000u32).to_string(), "x^2+3*x+5");
    /// ```
    #[inline]
    fn rem(self, m: T) -> UnsignedPolynomial<T> {
        &self % m
    }
}

impl<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>> Mod<T> for &NaturalPolynomial
where
    Natural: From<T>,
{
    type Output = UnsignedPolynomial<T>;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a value of an unsigned primitive
    /// integer type, keeping the remainders as an [`UnsignedPolynomial`] with that coefficient
    /// type, taking the polynomial by reference.
    ///
    /// A [`NaturalPolynomial`]'s coefficients are never negative, so this agrees with `%`
    /// everywhere. See the documentation for the [`Rem`] implementation that takes the polynomial
    /// by reference for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits in the
    /// polynomial's coefficients, and $m$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// assert_eq!((&p).mod_op(1000u32).to_string(), "x^2+3*x+5");
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> UnsignedPolynomial<T> {
        self % m
    }
}

impl<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>> Mod<T> for NaturalPolynomial
where
    Natural: From<T>,
{
    type Output = UnsignedPolynomial<T>;

    /// Divides every coefficient of a [`NaturalPolynomial`] by a value of an unsigned primitive
    /// integer type, keeping the remainders as an [`UnsignedPolynomial`] with that coefficient
    /// type, taking the polynomial by value.
    ///
    /// A [`NaturalPolynomial`]'s coefficients are never negative, so this agrees with `%`
    /// everywhere. See the documentation for the [`Rem`] implementation that takes the polynomial
    /// by reference for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits in the
    /// polynomial's coefficients, and $m$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("1000000000001*x^2+2000000000003*x+5").unwrap();
    /// assert_eq!(p.mod_op(1000u32).to_string(), "x^2+3*x+5");
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> UnsignedPolynomial<T> {
        self % m
    }
}
