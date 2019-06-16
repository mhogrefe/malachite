#![allow(
    unknown_lints,
    clippy::suspicious_arithmetic_impl,
    clippy::suspicious_op_assign_impl
)]

#[macro_use]
extern crate malachite_base;
extern crate rand;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(feature = "32_bit_limbs")]
pub use platform_32 as platform;
#[cfg(not(feature = "32_bit_limbs"))]
pub use platform_64 as platform;

pub mod error;
#[cfg(feature = "32_bit_limbs")]
pub mod platform_32;
#[cfg(not(feature = "32_bit_limbs"))]
pub mod platform_64;
#[macro_use]
pub mod natural;
pub mod integer;
