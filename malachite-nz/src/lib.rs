#![allow(unknown_lints, suspicious_arithmetic_impl, suspicious_op_assign_impl)]

#[macro_use]
extern crate malachite_base;
extern crate rand;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub mod error;
#[macro_use]
pub mod natural;
pub mod integer;
