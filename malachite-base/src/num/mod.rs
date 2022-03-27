#[doc(hidden)]
#[macro_use]
pub mod macros;

/// Traits for primitive integer arithmetic.
pub mod arithmetic;
/// Traits for primitive integers and some of their basic functionality.
pub mod basic;
/// Traits for comparing the absolute values of numbers for equality or order.
pub mod comparison;
/// Traits for converting to and from numbers, converting to and from strings,
/// and extracting digits.
pub mod conversion;
/// Iterators that generate primitive integers without repetition.
pub mod exhaustive;
/// Functions specific to primitive floating-point numbers.
pub mod float;
/// Iterators related to numbers.
pub mod iterators;
/// Functions related to logic and bit manipulation.
pub mod logic;
/// Iterators that generate primitive integers randomly.
pub mod random;
