#[doc(hidden)]
#[macro_use]
pub mod macros;

/// Traits for arithmetic.
pub mod arithmetic;
/// Traits for primitive integers or floats and some of their basic functionality.
pub mod basic;
/// Traits for comparing the absolute values of numbers for equality or order.
pub mod comparison;
/// Traits for converting to and from numbers, converting to and from strings, and extracting
/// digits.
pub mod conversion;
/// Iterators that generate numbers without repetition.
pub mod exhaustive;
/// Traits for generating primes, primality testing, and factorization (TODO!)
pub mod factorization;
/// [`NiceFloat`](float::NiceFloat), a wrapper around primitive floats.
pub mod float;
/// Iterators related to numbers.
pub mod iterators;
/// Traits for logic and bit manipulation.
pub mod logic;
#[cfg(feature = "random")]
/// Iterators that generate numbers randomly.
pub mod random;
