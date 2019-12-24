use std::str::FromStr;

use malachite_base::crement::Crementable;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};

use natural::Natural;
use platform::Limb;

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
        self.abs.is_valid() && (self.sign || self.abs != 0 as Limb)
    }

    pub fn trillion() -> Integer {
        Integer::from_str("1000000000000").unwrap()
    }
}

/// The constant 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Zero for Integer {
    const ZERO: Integer = Integer {
        sign: true,
        abs: Natural::ZERO,
    };
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl One for Integer {
    const ONE: Integer = Integer {
        sign: true,
        abs: Natural::ONE,
    };
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Two for Integer {
    const TWO: Integer = Integer {
        sign: true,
        abs: Natural::TWO,
    };
}

/// The constant -1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl NegativeOne for Integer {
    const NEGATIVE_ONE: Integer = Integer {
        sign: false,
        abs: Natural::ONE,
    };
}

// Implement `Named` for `Integer`.
impl_named!(Integer);

impl Crementable for Integer {
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut i = Integer::from(10);
    ///     i.increment();
    ///     assert_eq!(i, 11);
    ///
    ///     let mut i = Integer::from(-5);
    ///     i.increment();
    ///     assert_eq!(i, -4);
    /// }
    /// ```
    #[inline]
    fn increment(&mut self) {
        self.add_assign_limb(1);
    }

    /// Decrements `self`.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut i = Integer::from(10);
    ///     i.decrement();
    ///     assert_eq!(i, 9);
    ///
    ///     let mut i = Integer::from(-5);
    ///     i.decrement();
    ///     assert_eq!(i, -6);
    /// }
    /// ```
    #[inline]
    fn decrement(&mut self) {
        self.sub_assign_limb(1);
    }
}

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
    pub mod shl_i;
    pub mod shl_u;
    pub mod shr_i;
    pub mod shr_u;
    pub mod sub;
    pub mod sub_mul;
}

pub mod comparison {
    pub mod ord;
    pub mod ord_abs;
    pub mod partial_eq_limb;
    pub mod partial_eq_natural;
    pub mod partial_eq_signed_limb;
    pub mod partial_ord_abs_limb;
    pub mod partial_ord_abs_natural;
    pub mod partial_ord_abs_signed_limb;
    pub mod partial_ord_limb;
    pub mod partial_ord_natural;
    pub mod partial_ord_signed_limb;
    pub mod sign;
}

pub mod conversion;

pub mod logic {
    pub mod and;
    pub mod bit_access;
    pub mod bit_scan;
    pub mod checked_count_ones;
    pub mod checked_count_zeros;
    pub mod checked_hamming_distance;
    pub mod not;
    pub mod or;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod xor;
}
