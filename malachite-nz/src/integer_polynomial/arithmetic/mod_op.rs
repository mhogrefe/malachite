// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use core::ops::{Rem, RemAssign};
use malachite_base::num::arithmetic::traits::{Mod, NegMod};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

impl Mod<Natural> for IntegerPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by a [`Natural`], keeping the
    /// remainders as a [`NaturalPolynomial`], taking the polynomial by value and the modulus by
    /// value.
    ///
    /// Each remainder is taken in $[0, m)$, the way [`Mod`] does for [`Integer`]s, so a negative
    /// coefficient $c$ that is not a multiple of $m$ becomes $m - (-c \bmod m)$. The result is
    /// therefore reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](malachite_base::num::arithmetic::traits::ModIsReduced::mod_is_reduced)
    /// returns `true` for it, and every coefficient of the result is congruent to the corresponding
    /// coefficient of the input.
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $m$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $-6x^2 + 3x - 1$ modulo $3$ is the constant $2$.
    ///
    /// $$
    /// f(p, m) = q, \quad \text{where} \quad q_i = p_i - m \left \lfloor \frac{p_i}{m}
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Every coefficient is taken modulo 3, and negative ones become non-negative.
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_op(Natural::from(3u32)).to_string(),
    ///     "x^2+2*x+1"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// assert_eq!(p.clone().mod_op(Natural::from(3u32)).to_string(), "2");
    /// ```
    #[inline]
    fn mod_op(self, m: Natural) -> NaturalPolynomial {
        self.mod_op(&m)
    }
}

impl<'a> Mod<&'a Natural> for IntegerPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by a [`Natural`], keeping the
    /// remainders as a [`NaturalPolynomial`], taking the polynomial by value and the modulus by
    /// reference.
    ///
    /// See the documentation for the [`Mod`] implementation on [`IntegerPolynomial`] that takes
    /// both arguments by value for details, including how negative coefficients are handled and how
    /// reducing can lower the degree.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Every coefficient is taken modulo 3, and negative ones become non-negative.
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_op(&Natural::from(3u32)).to_string(),
    ///     "x^2+2*x+1"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// assert_eq!(p.clone().mod_op(&Natural::from(3u32)).to_string(), "2");
    /// ```
    fn mod_op(self, m: &'a Natural) -> NaturalPolynomial {
        assert_ne!(*m, 0u32, "division by zero");
        NaturalPolynomial::from_coefficients_asc(
            self.coefficients
                .into_iter()
                .map(|Integer { sign, abs }| if sign { abs % m } else { abs.neg_mod(m) })
                .collect::<Vec<_>>(),
        )
    }
}

impl Mod<Natural> for &IntegerPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by a [`Natural`], keeping the
    /// remainders as a [`NaturalPolynomial`], taking the polynomial by reference and the modulus by
    /// value.
    ///
    /// See the documentation for the [`Mod`] implementation on [`IntegerPolynomial`] that takes
    /// both arguments by value for details, including how negative coefficients are handled and how
    /// reducing can lower the degree.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Every coefficient is taken modulo 3, and negative ones become non-negative.
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!((&p).mod_op(Natural::from(3u32)).to_string(), "x^2+2*x+1");
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// assert_eq!((&p).mod_op(Natural::from(3u32)).to_string(), "2");
    /// ```
    #[inline]
    fn mod_op(self, m: Natural) -> NaturalPolynomial {
        self.mod_op(&m)
    }
}

impl<'a> Mod<&'a Natural> for &IntegerPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by a [`Natural`], keeping the
    /// remainders as a [`NaturalPolynomial`], taking the polynomial by reference and the modulus by
    /// reference.
    ///
    /// See the documentation for the [`Mod`] implementation on [`IntegerPolynomial`] that takes
    /// both arguments by value for details, including how negative coefficients are handled and how
    /// reducing can lower the degree.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Every coefficient is taken modulo 3, and negative ones become non-negative.
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!((&p).mod_op(&Natural::from(3u32)).to_string(), "x^2+2*x+1");
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// assert_eq!((&p).mod_op(&Natural::from(3u32)).to_string(), "2");
    /// ```
    fn mod_op(self, m: &'a Natural) -> NaturalPolynomial {
        assert_ne!(*m, 0u32, "division by zero");
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        NaturalPolynomial::from_coefficients_asc(
            self.coefficients
                .iter()
                .map(|c| {
                    if c.sign {
                        &c.abs % m
                    } else {
                        (&c.abs).neg_mod(m)
                    }
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>> Mod<T> for &IntegerPolynomial
where
    Natural: From<T>,
{
    type Output = UnsignedPolynomial<T>;

    /// Divides every coefficient of an [`IntegerPolynomial`] by a value of an unsigned primitive
    /// integer type, keeping the remainders as an [`UnsignedPolynomial`] with that coefficient
    /// type, taking the polynomial by reference.
    ///
    /// Each remainder is taken in $[0, m)$, so negative coefficients become non-negative, and every
    /// remainder fits in `m`'s type. Apart from the result's type, this is the same operation as
    /// reducing modulo `Natural::from(m)`; see the documentation for the [`Mod`] implementation on
    /// [`IntegerPolynomial`] that takes both arguments by value for details, including how reducing
    /// can lower the degree.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-1000000000001*x^2+2000000000003*x-5").unwrap();
    /// let q: UnsignedPolynomial<u8> = (&p).mod_op(7u8);
    /// assert_eq!(q.to_string(), "5*x^2+5*x+2");
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// let p = IntegerPolynomial::from_str("-1024*x^2-3").unwrap();
    /// assert_eq!((&p).mod_op(4u64).to_string(), "1");
    /// ```
    fn mod_op(self, m: T) -> UnsignedPolynomial<T> {
        assert_ne!(m, T::ZERO, "division by zero");
        let m = Natural::from(m);
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        UnsignedPolynomial::from_coefficients_asc(
            self.coefficients
                .iter()
                .map(|c| {
                    T::exact_from(&if c.sign {
                        &c.abs % &m
                    } else {
                        (&c.abs).neg_mod(&m)
                    })
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>> Mod<T> for IntegerPolynomial
where
    Natural: From<T>,
{
    type Output = UnsignedPolynomial<T>;

    /// Divides every coefficient of an [`IntegerPolynomial`] by a value of an unsigned primitive
    /// integer type, keeping the remainders as an [`UnsignedPolynomial`] with that coefficient
    /// type, taking the polynomial by value.
    ///
    /// Taking the polynomial by value saves nothing, since the remainders go into new storage
    /// either way. See the documentation for the [`Mod`] implementation that takes the polynomial
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!(p.mod_op(3u32).to_string(), "x^2+2*x+1");
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> UnsignedPolynomial<T> {
        (&self).mod_op(m)
    }
}

impl Rem<Integer> for IntegerPolynomial {
    type Output = Self;

    /// Divides every coefficient of an [`IntegerPolynomial`] by an [`Integer`], keeping the
    /// remainders, taking the polynomial by value and the modulus by value.
    ///
    /// Each remainder has the sign of its coefficient and a smaller absolute value than $m$, as
    /// with [`Rem`] for [`Integer`]s, so the sign of $m$ makes no difference. This is the remainder
    /// of truncating division: with the coefficient-wise quotient $q_i = \operatorname{
    /// sgn}(p_im)\lfloor |p_i/m| \rfloor$, $p = mq + r$ holds exactly. For a remainder that is
    /// always non-negative, and a [`NaturalPolynomial`] result, use [`Mod`].
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $m$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $-6x^2 + 3x - 1$ modulo $3$ is the constant $-1$.
    ///
    /// $$
    /// f(p, m) = r, \quad \text{where} \quad r_i = p_i - m \operatorname{sgn}(p_im)
    ///     \left \lfloor \left | \frac{p_i}{m} \right | \right \rfloor.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Every coefficient is taken modulo 3, keeping its sign.
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!((p.clone() % Integer::from(3)).to_string(), "x^2-x-2");
    ///
    /// // The sign of the modulus makes no difference, and reducing the leading coefficient to zero
    /// // lowers the degree.
    /// let p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// assert_eq!((p % Integer::from(-3)).to_string(), "-1");
    /// ```
    #[inline]
    fn rem(mut self, m: Integer) -> Self {
        self %= m;
        self
    }
}

impl<'a> Rem<&'a Integer> for IntegerPolynomial {
    type Output = Self;

    /// Divides every coefficient of an [`IntegerPolynomial`] by an [`Integer`], keeping the
    /// remainders, taking the polynomial by value and the modulus by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`IntegerPolynomial`] that takes
    /// both arguments by value for details, including the signs of the remainders and how reducing
    /// can lower the degree.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Every coefficient is taken modulo 3, keeping its sign.
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!((p.clone() % &Integer::from(3)).to_string(), "x^2-x-2");
    ///
    /// // The sign of the modulus makes no difference, and reducing the leading coefficient to zero
    /// // lowers the degree.
    /// let p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// assert_eq!((p % &Integer::from(-3)).to_string(), "-1");
    /// ```
    #[inline]
    fn rem(mut self, m: &'a Integer) -> Self {
        self %= m;
        self
    }
}

impl Rem<Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by an [`Integer`], keeping the
    /// remainders, taking the polynomial by reference and the modulus by value.
    ///
    /// See the documentation for the [`Rem`] implementation on [`IntegerPolynomial`] that takes
    /// both arguments by value for details, including the signs of the remainders and how reducing
    /// can lower the degree.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Every coefficient is taken modulo 3, keeping its sign.
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!((&p % Integer::from(3)).to_string(), "x^2-x-2");
    ///
    /// // The sign of the modulus makes no difference, and reducing the leading coefficient to zero
    /// // lowers the degree.
    /// let p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// assert_eq!((&p % Integer::from(-3)).to_string(), "-1");
    /// // The polynomial is left alone.
    /// assert_eq!(p.to_string(), "-6*x^2+3*x-1");
    /// ```
    #[inline]
    fn rem(self, m: Integer) -> IntegerPolynomial {
        self % &m
    }
}

impl<'a> Rem<&'a Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by an [`Integer`], keeping the
    /// remainders, taking the polynomial by reference and the modulus by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`IntegerPolynomial`] that takes
    /// both arguments by value for details, including the signs of the remainders and how reducing
    /// can lower the degree.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Every coefficient is taken modulo 3, keeping its sign.
    /// let p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// assert_eq!((&p % &Integer::from(3)).to_string(), "x^2-x-2");
    ///
    /// // The sign of the modulus makes no difference, and reducing the leading coefficient to zero
    /// // lowers the degree.
    /// let p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// assert_eq!((&p % &Integer::from(-3)).to_string(), "-1");
    /// // The polynomial is left alone.
    /// assert_eq!(p.to_string(), "-6*x^2+3*x-1");
    /// ```
    fn rem(self, m: &'a Integer) -> IntegerPolynomial {
        assert_ne!(*m, 0u32, "division by zero");
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        IntegerPolynomial::from_coefficients_asc(
            self.coefficients.iter().map(|c| c % m).collect::<Vec<_>>(),
        )
    }
}

impl RemAssign<Integer> for IntegerPolynomial {
    /// Divides every coefficient of an [`IntegerPolynomial`] by an [`Integer`], replacing the
    /// polynomial by the one whose coefficients are the remainders, taking the modulus by value.
    ///
    /// See the documentation for the [`Rem`] implementation on [`IntegerPolynomial`] that takes
    /// both arguments by value for details, including the signs of the remainders and how reducing
    /// can lower the degree.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// p %= Integer::from(3);
    /// assert_eq!(p.to_string(), "x^2-x-2");
    ///
    /// let mut p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// p %= Integer::from(-3);
    /// assert_eq!(p.to_string(), "-1");
    /// ```
    #[inline]
    fn rem_assign(&mut self, m: Integer) {
        *self %= &m;
    }
}

impl<'a> RemAssign<&'a Integer> for IntegerPolynomial {
    /// Divides every coefficient of an [`IntegerPolynomial`] by an [`Integer`], replacing the
    /// polynomial by the one whose coefficients are the remainders, taking the modulus by
    /// reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`IntegerPolynomial`] that takes
    /// both arguments by value for details, including the signs of the remainders and how reducing
    /// can lower the degree.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-4*x-5").unwrap();
    /// p %= &Integer::from(3);
    /// assert_eq!(p.to_string(), "x^2-x-2");
    ///
    /// let mut p = IntegerPolynomial::from_str("-6*x^2+3*x-1").unwrap();
    /// p %= &Integer::from(-3);
    /// assert_eq!(p.to_string(), "-1");
    /// ```
    fn rem_assign(&mut self, m: &'a Integer) {
        assert_ne!(*m, 0u32, "division by zero");
        for c in &mut self.coefficients {
            *c %= m;
        }
        self.trim();
    }
}
