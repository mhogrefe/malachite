use std::str::FromStr;

use malachite_base::comparison::Min;
use malachite_base::crement::Crementable;
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

impl Natural {
    fn demote_if_small(&mut self) {
        let demoted_value = if let Natural(Large(ref limbs)) = *self {
            match limbs.len() {
                0 => Some(0),
                1 => Some(limbs[0]),
                _ => None,
            }
        } else {
            None
        };
        if let Some(small) = demoted_value {
            *self = Natural(Small(small));
        }
    }

    pub(crate) fn promote_in_place(&mut self) -> &mut Vec<Limb> {
        if let Natural(Small(x)) = *self {
            *self = Natural(Large(vec![x]));
        }
        if let Natural(Large(ref mut xs)) = *self {
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

    /// A `Large` value used for testing.
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
    const ZERO: Natural = Natural(Small(0));
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl One for Natural {
    const ONE: Natural = Natural(Small(1));
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Two for Natural {
    const TWO: Natural = Natural(Small(2));
}

/// The minimum value of a `Natural`, 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Min for Natural {
    const MIN: Natural = Natural::ZERO;
}

// Implement `Named` for `Natural`.
impl_named!(Natural);

impl Crementable for Natural {
    /// Increments `self`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::crement::Crementable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut i = Natural::from(10u32);
    /// i.increment();
    /// assert_eq!(i, 11);
    /// ```
    #[inline]
    fn increment(&mut self) {
        self.add_assign_limb(1);
    }

    /// Decrements `self`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `self` == 0`.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::crement::Crementable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut i = Natural::from(10u32);
    /// i.decrement();
    /// assert_eq!(i, 9);
    /// ```
    #[inline]
    fn decrement(&mut self) {
        self.sub_assign_limb(1);
    }
}

macro_rules! mutate_with_possible_promotion {
    ($n:ident, $small:ident, $large:ident, $process_small:expr, $process_large:expr) => {
        if let Natural(Small(ref mut $small)) = *$n {
            if let Some(small_result) = $process_small {
                *$small = small_result;
                return;
            }
        }
        if let Natural(Small(x)) = *$n {
            *$n = Natural(Large(vec![x]));
        }
        if let Natural(Large(ref mut $large)) = *$n {
            $process_large
        } else {
            unreachable!();
        }
    };
}

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
    pub mod mod_is_reduced;
    pub mod mod_op;
    pub mod mod_power_of_two;
    pub mod mod_power_of_two_is_reduced;
    pub mod mul;
    pub mod neg;
    pub mod next_power_of_two;
    pub mod parity;
    pub mod saturating_sub;
    pub mod saturating_sub_mul;
    pub mod shl_i;
    pub mod shl_u;
    pub mod shr_i;
    pub mod shr_u;
    pub mod square;
    pub mod sub;
    pub mod sub_mul;
}

pub mod conversion;

pub mod comparison {
    pub mod ord;
    pub mod partial_eq_primitive_integer;
    pub mod partial_ord_abs_primitive_integer;
    pub mod partial_ord_primitive_integer;
}

pub mod logic {
    pub mod and;
    pub mod bit_access;
    pub mod bit_block_access;
    pub mod bit_convertible;
    pub mod bit_iterable;
    pub mod bit_scan;
    pub mod count_ones;
    pub mod hamming_distance;
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
