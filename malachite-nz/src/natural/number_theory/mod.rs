// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`BellNumber`](malachite_base::num::number_theory::traits::BellNumber), a
/// trait for computing the number of ways to partition a set.
pub mod bell_number;
/// Implementations of
/// [`BinomialCoefficient`](malachite_base::num::number_theory::traits::BinomialCoefficient), a
/// trait for computing the binomial coefficient of two numbers.
pub mod binomial_coefficient;
/// The continued-fraction and half-gcd machinery, adopted from FLINT, shared by rational
/// reconstruction and, in malachite-q, the simplest rational in an interval.
#[doc(hidden)]
pub mod cfrac_helpers;
/// Implementations of [`CoprimeWith`](malachite_base::num::number_theory::traits::CoprimeWith), a
/// trait for determining whether two numbers are coprime.
pub mod coprime_with;
/// Implementations of [`Factorial`](malachite_base::num::number_theory::traits::Factorial),
/// [`DoubleFactorial`](malachite_base::num::number_theory::traits::DoubleFactorial),
/// [`Multifactorial`](malachite_base::num::number_theory::traits::Multifactorial), and
/// [`Subfactorial`](malachite_base::num::number_theory::traits::Subfactorial).
pub mod factorial;
/// Implementations of
/// [`FallingFactorial`](malachite_base::num::number_theory::traits::FallingFactorial), a trait for
/// computing the falling factorial of a number.
pub mod falling_factorial;
/// [`Fibonacci`](malachite_base::num::number_theory::traits::Fibonacci) and
/// [`LucasNumber`](malachite_base::num::number_theory::traits::LucasNumber), traits for computing
/// Fibonacci and Lucas numbers.
pub mod fibonacci;
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
/// A function for computing Landau's function.
pub mod landau_function;
/// Implementations of [`Lcm`](malachite_base::num::number_theory::traits::Lcm),
/// [`LcmAssign`](malachite_base::num::number_theory::traits::LcmAssign), and
/// [`CheckedLcm`](malachite_base::num::number_theory::traits::CheckedLcm), traits for computing the
/// LCM (least common multiple) of two numbers.
pub mod lcm;
/// An implementation of [`Primorial`](malachite_base::num::number_theory::traits::Primorial), a
/// trait for computing the primorial of a number.
pub mod primorial;
/// Implementations of
/// [`RisingFactorial`](malachite_base::num::number_theory::traits::RisingFactorial), a trait for
/// computing the rising factorial of a number.
pub mod rising_factorial;
/// An implementation of [`StirlingNumberOfTheSecondKind`](
/// malachite_base::num::number_theory::traits::StirlingNumberOfTheSecondKind), a trait for
/// computing Stirling numbers of the second kind.
pub mod stirling_number_of_the_second_kind;
