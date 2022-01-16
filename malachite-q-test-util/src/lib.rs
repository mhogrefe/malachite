#![allow(
    unstable_name_collisions,
    clippy::assertions_on_constants,
    clippy::cognitive_complexity,
    clippy::many_single_char_names,
    clippy::range_plus_one,
    clippy::suspicious_arithmetic_impl,
    clippy::suspicious_op_assign_impl,
    clippy::too_many_arguments,
    clippy::type_complexity
)]

extern crate num;
extern crate rug;

pub mod arithmetic {
    pub mod add;
    pub mod div;
    pub mod mul;
    pub mod sign;
    pub mod sub;
}
pub mod bench;
pub mod common;
pub mod generators;
