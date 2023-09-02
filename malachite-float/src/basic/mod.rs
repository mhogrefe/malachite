/// Classification of a [`Float`](super::Float) into several kinds.
pub mod classification;
/// Measuring the complexity to a [`Float`](super::Float).
pub mod complexity;
/// Various [`Float`](super::Float) constants. This module contains actual Rust constants like
/// [`One`](super::Float#impl-One-for-Float), and functions like [`one`](super::Float::one_prec)
/// which accept a precision.
#[macro_use]
pub mod constants;
/// Getting and setting the components of a [`Float`](super::Float).
pub mod get_and_set;
/// Getting [`Float`](super::Float)'s ulp (unit in the last place).
pub mod ulp;
