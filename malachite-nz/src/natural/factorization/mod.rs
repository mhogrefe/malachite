// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`IsPower`](malachite_base::num::factorization::traits::IsPower) and
/// [`ExpressAsPower`](malachite_base::num::factorization::traits::ExpressAsPower), traits for
/// testing if a number is a perfect power and, if it is, expressing it as such.
pub mod is_power;
/// An implementation of [`IsSquare`](malachite_base::num::factorization::traits::IsSquare), a trait
/// for testing if a number if a perfect square.
pub mod is_square;
/// An implementation of [`Primes`](malachite_base::num::factorization::traits::Primes), a trait for
/// generating prime numbers.
pub mod primes;
#[doc(hidden)]
pub mod remove_power;
