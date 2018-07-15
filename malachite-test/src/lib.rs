#![allow(
    unknown_lints,
    no_effect,
    type_complexity,
    unnecessary_operation,
    unused_must_use
)]

extern crate malachite_base;
extern crate malachite_nz;
extern crate num;
extern crate rand;
extern crate rug;
extern crate rust_wheels;

#[macro_use]
pub mod common;
pub mod hash;
pub mod inputs {
    pub mod base;
    pub mod common;
    pub mod integer;
    pub mod natural;
}
pub mod base;
pub mod integer;
pub mod natural;

pub fn register(registry: &mut common::DemoBenchRegistry) {
    base::register(registry);
    integer::register(registry);
    natural::register(registry);
}
