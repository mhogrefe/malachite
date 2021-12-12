#![allow(unstable_name_collisions, unused_must_use)]

extern crate itertools;
extern crate malachite_base;
extern crate malachite_base_test_util;
extern crate malachite_nz;
extern crate malachite_nz_test_util;
extern crate num;
extern crate rand;
extern crate rug;
extern crate rust_wheels;

#[macro_use]
pub mod common;

pub mod inputs {
    pub mod base;
    pub mod common;
    pub mod natural;
}
