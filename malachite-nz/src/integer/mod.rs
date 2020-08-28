use malachite_base::named::Named;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use natural::InnerNatural::Small;
use natural::Natural;
use std::str::FromStr;

/// An integer.
///
/// Any `Integer` whose absolute value is small enough to fit into a `Limb` is represented inline.
/// Only integers outside this range incur the costs of heap-allocation.
///
/// On a 64-bit system, an `Integer` takes up 40 bytes of space on the stack.
#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Integer {
    // whether the `Integer` is non-negative
    pub(crate) sign: bool,
    pub(crate) abs: Natural,
}

impl Integer {
    /// Returns true iff `self` is valid. To be valid, its absolute value must be valid, and if the
    /// absolute value is zero, the sign must be true. All `Integer`s must be valid.
    pub fn is_valid(&self) -> bool {
        self.abs.is_valid() && (self.sign || self.abs != 0)
    }

    pub fn trillion() -> Integer {
        Integer::from_str("1000000000000").unwrap()
    }
}

macro_rules! integer_zero {
    () => {
        Integer {
            sign: true,
            abs: natural_zero!(),
        }
    };
}

macro_rules! integer_one {
    () => {
        Integer {
            sign: true,
            abs: natural_one!(),
        }
    };
}

macro_rules! integer_two {
    () => {
        Integer {
            sign: true,
            abs: natural_two!(),
        }
    };
}

macro_rules! integer_negative_one {
    () => {
        Integer {
            sign: false,
            abs: natural_one!(),
        }
    };
}

/// The constant 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Zero for Integer {
    const ZERO: Integer = integer_zero!();
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl One for Integer {
    const ONE: Integer = integer_one!();
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Two for Integer {
    const TWO: Integer = integer_two!();
}

/// The constant -1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl NegativeOne for Integer {
    const NEGATIVE_ONE: Integer = integer_negative_one!();
}

// Implement `Named` for `Integer`.
impl_named!(Integer);

pub mod arithmetic {
    pub mod abs;
    pub mod add;
    pub mod add_mul;
    pub mod div;
    pub mod div_exact;
    pub mod div_mod;
    pub mod div_round;
    pub mod divisible_by;
    pub mod divisible_by_power_of_two;
    pub mod eq_mod;
    pub mod eq_mod_power_of_two;
    pub mod mod_op;
    pub mod mod_power_of_two;
    pub mod mul;
    pub mod neg;
    pub mod parity;
    pub mod power_of_two;
    pub mod round_to_multiple;
    pub mod round_to_multiple_of_power_of_two;
    pub mod shl;
    pub mod shl_round;
    pub mod shr;
    pub mod shr_round;
    pub mod sign;
    pub mod square;
    pub mod sub;
    pub mod sub_mul;
}

pub mod comparison {
    pub mod ord;
    pub mod ord_abs;
    pub mod partial_eq_natural;
    pub mod partial_eq_primitive_integer;
    pub mod partial_ord_abs_natural;
    pub mod partial_ord_abs_primitive_integer;
    pub mod partial_ord_natural;
    pub mod partial_ord_primitive_integer;
}

pub mod conversion;

pub mod exhaustive;

pub mod logic {
    pub mod and;
    pub mod bit_access;
    pub mod bit_block_access;
    pub mod bit_convertible;
    pub mod bit_iterable;
    pub mod bit_scan;
    pub mod checked_count_ones;
    pub mod checked_count_zeros;
    pub mod checked_hamming_distance;
    pub mod low_mask;
    pub mod not;
    pub mod or;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod xor;
}
