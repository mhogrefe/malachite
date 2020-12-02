use std::str::FromStr;

use malachite_base::comparison::traits::Min;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::slices::slice_trailing_zeros;

use natural::InnerNatural::{Large, Small};
use platform::Limb;

/// A natural (non-negative) integer.
///
/// Any `Natural` small enough to fit into a `Limb` is represented inline. Only naturals outside
/// this range incur the costs of heap-allocation.
///
/// On a 64-bit system, a `Natural` takes up 32 bytes of space on the stack.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Natural(pub(crate) InnerNatural);

/// We want to limit the visibility of the `Small` and `Large` constructors to within this crate. To
/// do this, we wrap the `InnerNatural` enum in a struct that gets compiled away.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub(crate) enum InnerNatural {
    Small(Limb),
    Large(Vec<Limb>),
}

macro_rules! natural_zero {
    () => {
        Natural(Small(0))
    };
}

macro_rules! natural_one {
    () => {
        Natural(Small(1))
    };
}

macro_rules! natural_two {
    () => {
        Natural(Small(2))
    };
}

impl Natural {
    // If a `Natural` is `Large` but is small enough to be `Small`, make it `Small`.
    fn demote_if_small(&mut self) {
        if let Natural(Large(ref limbs)) = self {
            match limbs.len() {
                0 => *self = natural_zero!(),
                1 => *self = Natural(Small(limbs[0])),
                _ => {}
            }
        }
    }

    // If a `Natural` is `Small`, make it `Large`. Return a reference to the `Limb` vector.
    pub(crate) fn promote_in_place(&mut self) -> &mut Vec<Limb> {
        if let Natural(Small(x)) = self {
            *self = Natural(Large(vec![*x]));
        }
        if let Natural(Large(ref mut xs)) = self {
            xs
        } else {
            unreachable!();
        }
    }

    pub(crate) fn trim(&mut self) {
        if let Natural(Large(ref mut limbs)) = *self {
            let trailing_zero_count = slice_trailing_zeros(limbs);
            if trailing_zero_count != 0 {
                let len = limbs.len();
                limbs.truncate(len - trailing_zero_count);
            }
        }
        self.demote_if_small();
    }

    /// Returns true iff `self` is valid. To be valid, `self` can only be `Large` when it is at
    /// least 2<sup>`Limb::WIDTH`</sup>, and cannot have leading zero limbs. All `Natural`s must be
    /// valid.
    pub fn is_valid(&self) -> bool {
        match *self {
            Natural(Small(_)) => true,
            Natural(Large(ref xs)) => xs.len() > 1 && *xs.last().unwrap() != 0,
        }
    }

    /// A `Large` value (when using 32-bit limbs) used for testing.
    pub fn trillion() -> Natural {
        Natural::from_str("1000000000000").unwrap()
    }
}

/// The constant 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Zero for Natural {
    const ZERO: Natural = natural_zero!();
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl One for Natural {
    const ONE: Natural = natural_one!();
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Two for Natural {
    const TWO: Natural = natural_two!();
}

/// The minimum value of a `Natural`, 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Min for Natural {
    const MIN: Natural = natural_zero!();
}

// Implement `Named` for `Natural`.
impl_named!(Natural);

pub mod arithmetic {
    pub mod add;
    pub mod add_mul;
    pub mod checked_sub;
    pub mod checked_sub_mul;
    pub mod div;
    pub mod div_exact;
    pub mod div_mod;
    pub mod div_round;
    pub mod divisible_by;
    pub mod divisible_by_power_of_two;
    pub mod eq_mod;
    pub mod eq_mod_power_of_two;
    pub mod is_power_of_two;
    pub mod log_two;
    pub mod mod_add;
    pub mod mod_is_reduced;
    pub mod mod_mul;
    pub mod mod_neg;
    pub mod mod_op;
    pub mod mod_pow;
    pub mod mod_power_of_two;
    pub mod mod_power_of_two_add;
    pub mod mod_power_of_two_is_reduced;
    pub mod mod_power_of_two_mul;
    pub mod mod_power_of_two_neg;
    pub mod mod_power_of_two_pow;
    pub mod mod_power_of_two_shl;
    pub mod mod_power_of_two_shr;
    pub mod mod_power_of_two_square;
    pub mod mod_power_of_two_sub;
    pub mod mod_square;
    pub mod mod_sub;
    pub mod mul;
    pub mod neg;
    pub mod next_power_of_two;
    pub mod parity;
    pub mod pow;
    pub mod power_of_two;
    pub mod round_to_multiple;
    pub mod round_to_multiple_of_power_of_two;
    pub mod saturating_sub;
    pub mod saturating_sub_mul;
    pub mod shl;
    pub mod shl_round;
    pub mod shr;
    pub mod shr_round;
    pub mod sign;
    pub mod square;
    pub mod sub;
    pub mod sub_mul;
}

pub mod conversion;

pub mod comparison {
    pub mod ord;
    pub mod partial_eq_primitive_int;
    pub mod partial_ord_abs_primitive_int;
    pub mod partial_ord_primitive_int;
}

pub mod exhaustive;

pub mod logic {
    pub mod and;
    pub mod bit_access;
    pub mod bit_block_access;
    pub mod bit_convertible;
    pub mod bit_iterable;
    pub mod bit_scan;
    pub mod count_ones;
    pub mod hamming_distance;
    pub mod low_mask;
    pub mod not;
    pub mod or;
    pub mod power_of_two_digit_iterable;
    pub mod power_of_two_digits;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod xor;
}

pub mod random {
    pub mod random_natural_below;
    pub mod random_natural_up_to_bits;
    pub mod random_natural_with_bits;
    pub mod special_random_natural_below;
    pub mod special_random_natural_up_to_bits;
    pub mod special_random_natural_with_bits;
}
