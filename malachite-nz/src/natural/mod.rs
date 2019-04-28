use std::str::FromStr;

use malachite_base::comparison::Min;
use malachite_base::crement::Crementable;
use malachite_base::limbs::limbs_trailing_zero_limbs;
use malachite_base::named::Named;
use malachite_base::num::traits::{One, Two, Zero};

use natural::Natural::*;
use platform::Limb;

/// A natural (non-negative) integer.
///
/// Any `Natural` small enough to fit into an `u32` is represented inline. Only naturals outside
/// this range incur the costs of heap-allocation.
///
/// On a 64-bit system, a `Natural` takes up 32 bytes of space on the stack.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Natural {
    Small(Limb),
    Large(Vec<Limb>),
}

impl Natural {
    fn demote_if_small(&mut self) {
        let demoted_value = if let Large(ref limbs) = *self {
            match limbs.len() {
                0 => Some(0),
                1 => Some(limbs[0]),
                _ => None,
            }
        } else {
            None
        };
        if let Some(small) = demoted_value {
            *self = Small(small);
        }
    }

    pub(crate) fn promote_in_place(&mut self) -> &mut Vec<Limb> {
        if let Small(x) = *self {
            *self = Large(vec![x]);
        }
        if let Large(ref mut xs) = *self {
            xs
        } else {
            unreachable!();
        }
    }

    pub(crate) fn trim(&mut self) {
        if let Large(ref mut limbs) = *self {
            let trailing_zero_count = limbs_trailing_zero_limbs(limbs);
            if trailing_zero_count != 0 {
                let len = limbs.len();
                limbs.truncate(len - trailing_zero_count);
            }
        }
        self.demote_if_small();
    }

    /// Returns true iff `self` is valid. To be valid, `self` can only be Large when it is at least
    /// 2<sup>32</sup>, and cannot have leading zero limbs. All Naturals must be valid.
    pub fn is_valid(&self) -> bool {
        match *self {
            Small(_) => true,
            Large(ref xs) => xs.len() > 1 && *xs.last().unwrap() != 0,
        }
    }

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
    const ZERO: Natural = Small(0);
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl One for Natural {
    const ONE: Natural = Small(1);
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Two for Natural {
    const TWO: Natural = Small(2);
}

/// The minimum value of a `Natural`, 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Min for Natural {
    const MIN: Natural = Small(0);
}

/// Implement `Named` for `Natural`.
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
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::crement::Crementable;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut i = Natural::from(10u32);
    ///     i.increment();
    ///     assert_eq!(i, 11);
    /// }
    /// ```
    fn increment(&mut self) {
        *self += 1 as Limb;
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
    /// fn main() {
    ///     let mut i = Natural::from(10u32);
    ///     i.decrement();
    ///     assert_eq!(i, 9);
    /// }
    /// ```
    fn decrement(&mut self) {
        *self -= 1 as Limb;
    }
}

macro_rules! mutate_with_possible_promotion {
    ($n:ident, $small:ident, $large:ident, $process_small:expr, $process_large:expr) => {
        if let Small(ref mut $small) = *$n {
            if let Some(small_result) = $process_small {
                *$small = small_result;
                return;
            }
        }
        if let Small(x) = *$n {
            *$n = Large(vec![x]);
        }
        if let Large(ref mut $large) = *$n {
            $process_large
        }
    };
}

pub mod arithmetic {
    pub mod add;
    pub mod add_limb;
    pub mod add_mul;
    pub mod add_mul_limb;
    pub mod checked_sub;
    pub mod checked_sub_limb;
    pub mod div_exact_limb;
    pub mod div_limb;
    pub mod div_mod;
    pub mod div_mod_limb;
    pub mod div_round_limb;
    pub mod divisible_by_limb;
    pub mod divisible_by_power_of_two;
    pub mod eq_limb_mod_limb;
    pub mod eq_limb_mod_power_of_two;
    pub mod eq_mod_power_of_two;
    pub mod is_power_of_two;
    pub mod log_two;
    pub mod mod_limb;
    pub mod mod_power_of_two;
    pub mod mul;
    pub mod mul_limb;
    pub mod neg;
    pub mod next_power_of_two;
    pub mod parity;
    pub mod saturating_sub;
    pub mod saturating_sub_limb;
    pub mod shl_i;
    pub mod shl_u;
    pub mod shr_i;
    pub mod shr_u;
    pub mod square;
    pub mod sub;
    pub mod sub_limb;
    pub mod sub_mul;
    pub mod sub_mul_limb;
}
pub mod conversion;
pub mod comparison {
    pub mod ord;
    pub mod partial_eq_limb;
    pub mod partial_ord_limb;
}
pub mod logic {
    pub mod and;
    pub mod and_limb;
    pub mod bit_access;
    pub mod bit_scan;
    pub mod count_ones;
    pub mod hamming_distance;
    pub mod hamming_distance_limb;
    pub mod not;
    pub mod or;
    pub mod or_limb;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod xor;
    pub mod xor_limb;
}
pub mod random {
    pub mod random_natural_below;
    pub mod random_natural_up_to_bits;
    pub mod random_natural_with_bits;
    pub mod special_random_natural_below;
    pub mod special_random_natural_up_to_bits;
    pub mod special_random_natural_with_bits;
}
