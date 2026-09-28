// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::mem::replace;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::{Parity, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;

// The number of factors of 2 that every coefficient of a nonzero numerator shares, stopping at
// `bits`, since no more are needed. When the denominator is even, the numerator's content is odd,
// so there are none.
fn numerator_shared_zeros(p: &RationalPolynomial, bits: u64) -> u64 {
    if p.denominator.even() {
        return 0;
    }
    let mut zeros = bits;
    for c in p.numerator.coefficients_asc() {
        if let Some(c_zeros) = c.trailing_zeros() {
            zeros = zeros.min(c_zeros);
            if zeros == 0 {
                break;
            }
        }
    }
    zeros
}

// Shifts every coefficient of a numerator right by `bits`, which divides each of them exactly.
fn numerator_shr(coefficients: &[Integer], bits: u64) -> IntegerPolynomial {
    IntegerPolynomial::from_coefficients_asc(coefficients.iter().map(|c| c >> bits).collect())
}

// Divides a polynomial by 2 to the power of `bits`. The factors of 2 shared by the numerator's
// coefficients are cancelled first, and only what is left shifts the denominator; since the
// denominator was coprime to the numerator's content, what is left of that content is odd or the
// denominator is unchanged, so the two stay coprime.
fn shr_assign_u64(p: &mut RationalPolynomial, bits: u64) {
    if bits == 0 || p.numerator == IntegerPolynomial::ZERO {
        return;
    }
    let numerator_zeros = numerator_shared_zeros(p, bits);
    if numerator_zeros != 0 {
        let numerator = replace(&mut p.numerator, IntegerPolynomial::ZERO);
        p.numerator = numerator_shr(&numerator.into_coefficients_asc(), numerator_zeros);
    }
    p.denominator <<= bits - numerator_zeros;
}

fn shr_ref_u64(p: &RationalPolynomial, bits: u64) -> RationalPolynomial {
    if bits == 0 || p.numerator == IntegerPolynomial::ZERO {
        return p.clone();
    }
    let numerator_zeros = numerator_shared_zeros(p, bits);
    RationalPolynomial {
        numerator: if numerator_zeros == 0 {
            p.numerator.clone()
        } else {
            numerator_shr(p.numerator.coefficients_asc(), numerator_zeros)
        },
        denominator: &p.denominator << (bits - numerator_zeros),
    }
}

fn shr_ref_signed<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    p: &'a RationalPolynomial,
    bits: S,
) -> RationalPolynomial
where
    &'a RationalPolynomial:
        Shl<U, Output = RationalPolynomial> + Shr<U, Output = RationalPolynomial>,
{
    if bits >= S::ZERO {
        p >> bits.unsigned_abs()
    } else {
        p << bits.unsigned_abs()
    }
}

fn shr_assign_signed<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    p: &mut RationalPolynomial,
    bits: S,
) where
    RationalPolynomial: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *p >>= bits.unsigned_abs();
    } else {
        *p <<= bits.unsigned_abs();
    }
}

macro_rules! impl_rational_polynomial_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for RationalPolynomial {
            type Output = RationalPolynomial;

            /// Right-shifts a [`RationalPolynomial`] (divides it by a power of 2), taking it by
            /// value.
            ///
            /// $$
            /// f(p, k) = p/2^k.
            /// $$
            ///
            /// Factors of 2 shared by the numerator's coefficients are cancelled first, and only
            /// the rest of the power shifts the denominator, so the result stays in lowest terms.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> RationalPolynomial {
                self >>= bits;
                self
            }
        }

        impl Shr<$t> for &RationalPolynomial {
            type Output = RationalPolynomial;

            /// Right-shifts a [`RationalPolynomial`] (divides it by a power of 2), taking it by
            /// reference.
            ///
            /// $$
            /// f(p, k) = p/2^k.
            /// $$
            ///
            /// Factors of 2 shared by the numerator's coefficients are cancelled first, and only
            /// the rest of the power shifts the denominator, so the result stays in lowest terms.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> RationalPolynomial {
                shr_ref_u64(self, u64::exact_from(bits))
            }
        }

        impl ShrAssign<$t> for RationalPolynomial {
            /// Right-shifts a [`RationalPolynomial`] (divides it by a power of 2), in place.
            ///
            /// $$
            /// p \gets p/2^k.
            /// $$
            ///
            /// Factors of 2 shared by the numerator's coefficients are cancelled first, and only
            /// the rest of the power shifts the denominator, so the result stays in lowest terms.
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
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_u64(self, u64::exact_from(bits));
            }
        }
    };
}
apply_to_unsigneds!(impl_rational_polynomial_shr_unsigned);

macro_rules! impl_rational_polynomial_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for RationalPolynomial {
            type Output = RationalPolynomial;

            /// Right-shifts a [`RationalPolynomial`] (divides it or multiplies it by a power of 2),
            /// taking it by value.
            ///
            /// $$
            /// f(p, k) = p/2^k.
            /// $$
            ///
            /// A negative `bits` multiplies by $2^{-k}$: factors of 2 in the denominator are
            /// cancelled first, and only the rest of the power shifts the numerator's coefficients.
            /// A positive `bits` cancels factors of 2 shared by the numerator's coefficients first.
            /// Either way, the result stays in lowest terms.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> RationalPolynomial {
                self >>= bits;
                self
            }
        }

        impl Shr<$t> for &RationalPolynomial {
            type Output = RationalPolynomial;

            /// Right-shifts a [`RationalPolynomial`] (divides it or multiplies it by a power of 2),
            /// taking it by reference.
            ///
            /// $$
            /// f(p, k) = p/2^k.
            /// $$
            ///
            /// A negative `bits` multiplies by $2^{-k}$: factors of 2 in the denominator are
            /// cancelled first, and only the rest of the power shifts the numerator's coefficients.
            /// A positive `bits` cancels factors of 2 shared by the numerator's coefficients first.
            /// Either way, the result stays in lowest terms.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> RationalPolynomial {
                shr_ref_signed(self, bits)
            }
        }

        impl ShrAssign<$t> for RationalPolynomial {
            /// Right-shifts a [`RationalPolynomial`] (divides it or multiplies it by a power of 2),
            /// in place.
            ///
            /// $$
            /// p \gets p/2^k.
            /// $$
            ///
            /// A negative `bits` multiplies by $2^{-k}$: factors of 2 in the denominator are
            /// cancelled first, and only the rest of the power shifts the numerator's coefficients.
            /// A positive `bits` cancels factors of 2 shared by the numerator's coefficients first.
            /// Either way, the result stays in lowest terms.
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
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_signed(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_rational_polynomial_shr_signed);
