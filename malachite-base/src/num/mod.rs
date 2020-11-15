#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod arithmetic;
/// This module defines the traits for primitive integers and some of their basic functionality.
pub mod basic;
pub mod comparison;
/// This module provides traits for converting to and from numbers.
pub mod conversion;
/// This module contains iterators that generate primitive integers without repetition.
pub mod exhaustive;
pub mod floats;
/// This module contains an iterator that regroups chunks of bits into chunks of a different size.
pub mod iterators;
pub mod logic;
/// This module contains iterators that generate primitive integers randomly.
pub mod random;
