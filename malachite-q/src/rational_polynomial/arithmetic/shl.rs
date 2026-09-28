// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer_polynomial::IntegerPolynomial;

// Multiplies a polynomial by 2 to the power of `bits`. The denominator's factors of 2 are cancelled
// first, and only what is left shifts the numerator; since the numerator's content was coprime to
// the denominator, it stays coprime to what is left of it.
fn shl_assign_u64(p: &mut RationalPolynomial, bits: u64) {
    if bits == 0 || p.numerator == IntegerPolynomial::ZERO {
        return;
    }
    let denominator_zeros = p.denominator.trailing_zeros().unwrap();
    if denominator_zeros >= bits {
        p.denominator >>= bits;
    } else {
        p.denominator >>= denominator_zeros;
        p.numerator <<= bits - denominator_zeros;
    }
}

fn shl_ref_u64(p: &RationalPolynomial, bits: u64) -> RationalPolynomial {
    if bits == 0 || p.numerator == IntegerPolynomial::ZERO {
        return p.clone();
    }
    let denominator_zeros = p.denominator.trailing_zeros().unwrap();
    if denominator_zeros >= bits {
        RationalPolynomial {
            numerator: p.numerator.clone(),
            denominator: &p.denominator >> bits,
        }
    } else {
        RationalPolynomial {
            numerator: &p.numerator << (bits - denominator_zeros),
            denominator: &p.denominator >> denominator_zeros,
        }
    }
}

fn shl_ref_signed<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    p: &'a RationalPolynomial,
    bits: S,
) -> RationalPolynomial
where
    &'a RationalPolynomial:
        Shl<U, Output = RationalPolynomial> + Shr<U, Output = RationalPolynomial>,
{
    if bits >= S::ZERO {
        p << bits.unsigned_abs()
    } else {
        p >> bits.unsigned_abs()
    }
}

fn shl_assign_signed<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    p: &mut RationalPolynomial,
    bits: S,
) where
    RationalPolynomial: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *p <<= bits.unsigned_abs();
    } else {
        *p >>= bits.unsigned_abs();
    }
}

macro_rules! impl_rational_polynomial_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for RationalPolynomial {
            type Output = RationalPolynomial;

            /// Left-shifts a [`RationalPolynomial`] (multiplies it by a power of 2), taking it by
            /// value.
            ///
            /// $$
            /// f(p, k) = 2^kp.
            /// $$
            ///
            /// Factors of 2 in the denominator are cancelled first, and only the rest of the power
            /// shifts the numerator's coefficients, so the result stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// numerator's coefficients and the denominator, $m$ is `bits`, and $k$ is
            /// `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> RationalPolynomial {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &RationalPolynomial {
            type Output = RationalPolynomial;

            /// Left-shifts a [`RationalPolynomial`] (multiplies it by a power of 2), taking it by
            /// reference.
            ///
            /// $$
            /// f(p, k) = 2^kp.
            /// $$
            ///
            /// Factors of 2 in the denominator are cancelled first, and only the rest of the power
            /// shifts the numerator's coefficients, so the result stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// numerator's coefficients and the denominator, $m$ is `bits`, and $k$ is
            /// `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> RationalPolynomial {
                shl_ref_u64(self, u64::exact_from(bits))
            }
        }

        impl ShlAssign<$t> for RationalPolynomial {
            /// Left-shifts a [`RationalPolynomial`] (multiplies it by a power of 2), in place.
            ///
            /// $$
            /// p \gets 2^kp.
            /// $$
            ///
            /// Factors of 2 in the denominator are cancelled first, and only the rest of the power
            /// shifts the numerator's coefficients, so the result stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// numerator's coefficients and the denominator, $m$ is `bits`, and $k$ is
            /// `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign_u64(self, u64::exact_from(bits));
            }
        }
    };
}
apply_to_unsigneds!(impl_rational_polynomial_shl_unsigned);

macro_rules! impl_rational_polynomial_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for RationalPolynomial {
            type Output = RationalPolynomial;

            /// Left-shifts a [`RationalPolynomial`] (multiplies it or divides it by a power of 2),
            /// taking it by value.
            ///
            /// $$
            /// f(p, k) = 2^kp.
            /// $$
            ///
            /// A negative `bits` divides by $2^{-k}$: factors of 2 shared by the numerator's
            /// coefficients are cancelled first, and only the rest of the power shifts the
            /// denominator. A positive `bits` cancels factors of 2 in the denominator first. Either
            /// way, the result stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// numerator's coefficients and the denominator, $m$ is `bits.unsigned_abs()`, and $k$
            /// is `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> RationalPolynomial {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &RationalPolynomial {
            type Output = RationalPolynomial;

            /// Left-shifts a [`RationalPolynomial`] (multiplies it or divides it by a power of 2),
            /// taking it by reference.
            ///
            /// $$
            /// f(p, k) = 2^kp.
            /// $$
            ///
            /// A negative `bits` divides by $2^{-k}$: factors of 2 shared by the numerator's
            /// coefficients are cancelled first, and only the rest of the power shifts the
            /// denominator. A positive `bits` cancels factors of 2 in the denominator first. Either
            /// way, the result stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// numerator's coefficients and the denominator, $m$ is `bits.unsigned_abs()`, and $k$
            /// is `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> RationalPolynomial {
                shl_ref_signed(self, bits)
            }
        }

        impl ShlAssign<$t> for RationalPolynomial {
            /// Left-shifts a [`RationalPolynomial`] (multiplies it or divides it by a power of 2),
            /// in place.
            ///
            /// $$
            /// p \gets 2^kp.
            /// $$
            ///
            /// A negative `bits` divides by $2^{-k}$: factors of 2 shared by the numerator's
            /// coefficients are cancelled first, and only the rest of the power shifts the
            /// denominator. A positive `bits` cancels factors of 2 in the denominator first. Either
            /// way, the result stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// numerator's coefficients and the denominator, $m$ is `bits.unsigned_abs()`, and $k$
            /// is `self.len()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign_signed(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_rational_polynomial_shl_signed);
