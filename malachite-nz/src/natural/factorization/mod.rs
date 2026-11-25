// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of
/// [`IsLucasProbablePrime`](malachite_base::num::factorization::traits::IsLucasProbablePrime), a
/// trait for testing whether a number is a Lucas probable prime.
pub mod is_lucas_probable_prime;
/// Implementations of [`IsPower`](malachite_base::num::factorization::traits::IsPower) and
/// [`ExpressAsPower`](malachite_base::num::factorization::traits::ExpressAsPower), traits for
/// testing if a number is a perfect power and, if it is, expressing it as such.
pub mod is_power;
// Nothing outside the tests uses these primality proofs until `IsPrime` is implemented for
// `Natural`.
#[cfg_attr(not(feature = "test_build"), allow(dead_code))]
#[doc(hidden)]
pub mod is_prime_n_pm_1;
/// An implementation of
/// [`IsProbablePrime`](malachite_base::num::factorization::traits::IsProbablePrime), a trait for
/// testing whether a number is a probable prime.
pub mod is_probable_prime;
/// An implementation of [`IsSquare`](malachite_base::num::factorization::traits::IsSquare), a trait
/// for testing if a number if a perfect square.
pub mod is_square;
/// Implementations of
/// [`IsStrongProbablePrime`](malachite_base::num::factorization::traits::IsStrongProbablePrime), a
/// trait for testing whether a number is a strong probable prime to a given base.
pub mod is_strong_probable_prime;
/// An implementation of [`Primes`](malachite_base::num::factorization::traits::Primes), a trait for
/// generating prime numbers.
pub mod primes;
#[doc(hidden)]
pub mod remove_power;
