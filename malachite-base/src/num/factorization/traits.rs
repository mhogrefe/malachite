// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// A trait for testing whether a number is prime.
pub trait IsPrime {
    fn is_prime(&self) -> bool;
}

/// A trait for testing whether a number is a square.
pub trait IsSquare {
    fn is_square(&self) -> bool;
}

/// A trait for testing whether a number is a perfect power.
pub trait IsPower {
    fn is_power(&self) -> bool;
}

/// A trait for expessing as number as the power of some number raised to an exponent greater than
/// 1, if such a representation exists.
pub trait ExpressAsPower: Sized {
    fn express_as_power(&self) -> Option<(Self, u64)>;
}

/// A trait for finding the prime factorization of a number.
pub trait Factor {
    type FACTORS;

    fn factor(&self) -> Self::FACTORS;
}

/// A trait for producing iterators of primes.
pub trait Primes {
    type I: Iterator<Item = Self>;
    type LI: Iterator<Item = Self>;

    fn primes_less_than(n: &Self) -> Self::LI;

    fn primes_less_than_or_equal_to(n: &Self) -> Self::LI;

    fn primes() -> Self::I;
}

/// A trait for finding a primitive root modulo a prime.
pub trait PrimitiveRootPrime {
    type Output;

    fn primitive_root_prime(&self) -> Self::Output;
}
