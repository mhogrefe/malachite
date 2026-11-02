// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of
/// [`BinomialCoefficient`](malachite_base::num::number_theory::traits::BinomialCoefficient), a
/// trait for computing the binomial coefficient of two numbers.
pub mod binomial_coefficient;
/// Implementations of [`ExtendedGcd`](malachite_base::num::number_theory::traits::ExtendedGcd), a
/// trait for computing the extended GCD of two numbers.
pub mod extended_gcd;
/// Implementations of
/// [`FallingFactorial`](malachite_base::num::number_theory::traits::FallingFactorial), a trait for
/// computing the falling factorial of a number.
pub mod falling_factorial;
/// Implementations of [`Gcd`](malachite_base::num::number_theory::traits::Gcd) and
/// [`GcdAssign`](malachite_base::num::number_theory::traits::GcdAssign), traits for computing the
/// GCD (greatest common divisor) of two numbers.
pub mod gcd;
/// Implementations of
/// [`LegendreSymbol`](malachite_base::num::number_theory::traits::LegendreSymbol),
/// [`JacobiSymbol`](malachite_base::num::number_theory::traits::JacobiSymbol), and
/// [`KroneckerSymbol`](malachite_base::num::number_theory::traits::KroneckerSymbol), traits for
/// computing the Legendre, Jacobi, and Kronecker symbols of two numbers.
pub mod kronecker_symbol;
/// Implementations of
/// [`RisingFactorial`](malachite_base::num::number_theory::traits::RisingFactorial), a trait for
/// computing the rising factorial of a number.
pub mod rising_factorial;
