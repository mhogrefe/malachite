#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod arithmetic;
/// This module defines the traits for primitive integers and some of their basic functionality.
pub mod basic;
/// This module provides traits for comparing the absolute values of numbers for equality or order.
pub mod comparison;
/// This module provides traits for converting to and from numbers, converting to and from strings,
/// and extracting digits.
pub mod conversion;
/// This module contains iterators that generate primitive integers without repetition.
pub mod exhaustive;
pub mod floats;
/// This module contains iterators related to numbers.
pub mod iterators;
/// This module contains functions related to logic and bit manipulation.
pub mod logic;
/// This module contains iterators that generate primitive integers randomly.
pub mod random;
